use std::sync::Arc;

use log::debug;

use keyvalue::KeyValue;
use crate::tree::{Tree, ImmutableTree};
use crate::commit::{CommitState};
use crate::diff::DiffIter;
use crate::index::Index;
use crate::Error;

#[derive(Clone)]
pub struct VStore {
    kv: Arc<Box<dyn KeyValue>>,
    cstate: CommitState,
}

impl VStore {
    pub fn new(kv: Arc<Box<dyn KeyValue>>) -> Result<Self, Error>  {
        let v = if !VStore::check_init(&kv)? {
            let v= VStore::first_init(&kv)?;
            v.mark_init()?;
            kv.sync()?;
            v
        }else{
            debug!("already initialized");
            VStore::load(&kv)?
        };

        debug!("commit state at init: {:?}", v.cstate);

        Ok(v)
    }

    fn load(kv: &Arc<Box<dyn KeyValue>>) -> Result<Self, Error> {
        let buf = kv.get(0, "commits".as_bytes())?
            .ok_or(Error::InitError(format!("Commits key not found")))?;

        let cstate = CommitState::from_buf(&buf)?;

        let v = VStore {
            kv: kv.clone(),
            cstate,
        };

        Ok(v)
    }

    fn first_init(kv: &Arc<Box<dyn KeyValue>>) -> Result<Self, Error> {
        debug!("first time initialization");

        let cstate = CommitState::default();
        let cstate_buf = cstate.to_vec()?;

        kv.put(0, "commits".as_bytes(), cstate_buf)?;
        
        let v = VStore {
            kv: kv.clone(),
            cstate,
        };

        Ok(v)
    }

    fn check_init(kv: &Arc<Box<dyn KeyValue>>) -> Result<bool, Error> {
        match kv.get(0, "init".as_bytes())? {
            Some(buf) => {
                if buf[0] != 100 {
                    Err(Error::InitError("Store not initialized properly".to_owned()))
                }else{
                    Ok(true)
                }
            },
            None => {
                Ok(false)
            }
        }
    }

    fn mark_init(&self) -> Result<(), Error> {
        debug!("marking init");

        let val = [100];
        self.kv.put(0, "init".as_bytes(), Vec::from(&val[..]))?;

        Ok(())
    }

    fn write_commit_state(&self) -> Result<(), Error> {
        debug!("writing commit state {:?}", self.cstate);
        let buf = self.cstate.to_vec()?;
        self.kv.put_str(0, "commits", buf)?;
        Ok(())
    }

    pub fn writable(&self) -> Result<Tree, Error> {
        debug!("(pre) writable commit {:?}", self.cstate);
        let c = self.cstate.open()?;
        debug!("(post) writable commit {:?}", self.cstate);
        debug!("writable commit open: {:?}", c);

        self.write_commit_state()?;
        self.kv.sync()?;


        let t = Tree::new(c.clone(), self.kv.clone())?;
        Ok(t)
    }

    pub fn immutable(&self, ver: u16) -> Result<ImmutableTree, Error> {
        let c = self.cstate.get_commit(ver).ok_or(Error::CommitNotFound(ver))?;

        let t = Tree::new_readonly(c, self.kv.clone())?;
        let t = ImmutableTree {
            t,
        };
        Ok(t)
    }

    pub fn sync_tree(&self, t: &Tree) -> Result<(), Error> {
        debug!("syncing tree");
        let buf = t.idx.to_vec()?;

        debug!("syncing index");
        self.kv.put(t.commit.ver, "index".as_bytes(), buf)?;

        debug!("syncing commit state {:?}", self.cstate);
        let buf = self.cstate.to_vec()?;
        self.kv.put(0, "commits".as_bytes(), buf)?;

        debug!("kv sync");
        self.kv.sync()?;
        Ok(())
    }

    pub fn commit(&self, t: Tree) -> Result<(), Error> {
        debug!("commiting start");

        self.cstate.commit();

        self.sync_tree(&t)?;

        debug!("commiting done");
        Ok(())
    }

    pub fn commit_state<'a>(&'a self) -> &'a CommitState {
        &self.cstate
    }

    fn load_index_at(&self, ver: u16)-> Result<Option<Index>, Error> {
        debug!("load index at ver: {}", ver);
        let index_key_buf = "index".as_bytes();

        let idx = match self.kv.get(ver, index_key_buf)? {
            Some(buf) => {
                let idx = Index::new_with_buf(&buf)?;
                Some(idx)
            },
            None => {
                None
            }
        };

        Ok(idx)
    }

    pub fn diff(&self, aver: u16, bver: u16) -> Result<DiffIter, Error> {
        let a_idx = self.load_index_at(aver)?.ok_or(Error::IndexNotFound)?;
        let b_idx = self.load_index_at(bver)?.ok_or(Error::IndexNotFound)?;

        let d = DiffIter::new(self.kv.clone(), a_idx, b_idx);

        Ok(d)
    }
}