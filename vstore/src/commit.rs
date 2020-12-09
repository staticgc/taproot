use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use parking_lot::RwLock;
use crate::Error;

use log::debug;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Commit {
    pub ver: u16,
    pub prev_ver: u16,
}

impl Commit {
    pub fn increment(&mut self) {
        self.prev_ver = self.ver;
        self.ver += 1;
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        let buf = rmp_serde::to_vec(&self)?;
        Ok(buf)
    }

    pub fn from_buf(buf: &[u8]) -> Result<Commit, Error> {
        let c = rmp_serde::from_read_ref(buf)?;
        Ok(c)
    }
}


#[derive(Default, Debug, Clone)]
pub struct CommitState {
    inner: Arc<RwLock<CommitStateInner>>,
}

impl CommitState {
    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        let inner = self.inner.read();
        let buf = rmp_serde::to_vec(&*inner)?;
        Ok(buf)
    }

    pub fn from_buf(buf: &[u8]) -> Result<CommitState, Error> {
        let inner: CommitStateInner = rmp_serde::from_read_ref(buf)?;
        let c = CommitState {
            inner: Arc::new(RwLock::new(inner)),
        };

        Ok(c)
    }

    pub fn get_commit(&self, ver: u16) -> Option<Commit> {
        let i = self.inner.read();
        i.get_commit(ver)

    }

    pub fn head_version(&self) -> u16 {
        let i = self.inner.read();
        i.head_ver
    }

    pub fn open_version(&self) -> u16 {
        let i = self.inner.read();
        i.open_ver
    }

    pub fn open(&self) -> Result<Commit, Error> {
        let mut i = self.inner.write();
        i.open()
    }

    pub fn commit(&self) {
        let mut i = self.inner.write();
        i.commit();
    }

}


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct CommitStateInner {
    head_ver: u16,
    open_ver: u16,
    cmap: HashMap<u16, Commit>,
}

impl CommitStateInner {
    fn get_commit(&self, ver: u16) -> Option<Commit> {
        self.cmap.get(&ver).map(|c| c.clone())
    }

    fn new_version_from_head(&mut self) -> Result<Commit, Error> {
        let mut c = self.get_commit(self.head_ver)
            .ok_or(Error::HeadVersionNotFound(self.head_ver))?;

        c.increment();
        Ok(c)
    }

    fn create_first_version(&self) -> Commit {
        Commit {
            ver: 1,
            prev_ver: 0,
        }
    } 

    fn commit(&mut self) {
        self.open_ver = 0;
    }

    fn open(&mut self) -> Result<Commit, Error> {
        let c = if self.head_ver == 0 {
            debug!("creating first version");
            self.create_first_version()
        }else{
            if self.open_ver == 0 {
                debug!("creating version from head");
                self.new_version_from_head()?
            }else {
                self.get_commit(self.open_ver)
                    .ok_or(Error::VersionNotFound(self.open_ver))?
            }
        };

        self.head_ver = c.ver;
        self.open_ver = c.ver;
        self.cmap.insert(c.ver, c.clone());

        Ok(c)
    }
}