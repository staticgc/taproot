
use std::sync::Arc;
use crate::index::Index;
use crate::commit::Commit;

use log::debug;

use keyvalue::KeyValue;
use valuepack::Pack;

use crate::Error;

pub struct Tree {
    kv: Arc<Box<dyn KeyValue>>,
    pub (crate) idx: Index,
    pub commit: Commit,
}

impl Tree {
    pub (crate) fn new(c: Commit, kv: Arc<Box<dyn KeyValue>>) -> Result<Self, Error> {
        let idx = match Tree::load_index(&c, &kv)? {
            Some(idx) => idx,
            None => Index::new(20, c.ver),
        };

        idx.set_version(c.ver);

        let t = Tree {
            kv,
            idx,
            commit: c,
        };

        Ok(t)
    }

    pub (crate) fn new_readonly(c: Commit, kv: Arc<Box<dyn KeyValue>>) -> Result<Self, Error> {
        let idx = Tree::load_index(&c, &kv)?.ok_or(Error::IndexNotFound)?;

        let t = Tree {
            kv,
            idx,
            commit: c,
        };

        Ok(t)
    }

    fn load_index_at(ver: u16, kv: &Arc<Box<dyn KeyValue>>) -> Result<Option<Index>, Error> {
        debug!("load index at ver: {}", ver);
        let index_key_buf = "index".as_bytes();

        let idx = match kv.get(ver, index_key_buf)? {
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

    fn load_index(c: &Commit, kv: &Arc<Box<dyn KeyValue>>) -> Result<Option<Index>, Error> {
        let idx = match Tree::load_index_at(c.ver, &kv)? {
            Some(idx) => Some(idx),
            None => {
                if c.prev_ver == 0 {
                    None
                }else{
                    match Tree::load_index_at(c.prev_ver, &kv)? {
                        Some(idx) => Some(idx),
                        None => None
                    }
                }
            }
        };

        Ok(idx)
    }

    fn load_pack(&self, ver: u16, part: u32) -> Result<Option<Pack>, Error> {
        debug!("load pack ver: {} part: {}", ver, part);

        let key = part.to_be_bytes();
        let pack = match self.kv.get(ver, &key[..])? {
            None => None,
            Some(buf) => {
                Some(Pack::from_buf(&buf)?)
            }
        };

        Ok(pack)
    }

    pub fn put_str(&self, key: &str, val: &[u8]) -> Result<(), Error> {
        debug!("put_str key: {}", key);

        self.put(key.as_bytes(), val)
    }

    pub fn put(&self, key: &[u8], val: &[u8]) -> Result<(), Error> {
        let (p_ver, p, _) = self.idx.get_part(key);
        debug!("put pver: {} part: {}", p_ver, p);

        let mut pack = if p_ver == 0 {
            Pack::new()
        }else{
            let pack = self.load_pack(p_ver, p)?.ok_or(Error::PackNotFound(p_ver, p))?;
            pack
        };

        pack.put(self.commit.ver, key, Vec::from(val));
        let buf = pack.to_vec()?;

        debug!("put store kv cver: {} pver: {} part: {}", self.commit.ver, p_ver, p);
        self.kv.put(self.commit.ver, &p.to_be_bytes()[..], &buf)?;
        self.idx.set_part(p, self.commit.ver);
        debug!("index part set {:?}", self.idx.get_part(key));

        Ok(())
    }


    pub fn delete_str(&self, key: &str) -> Result<(), Error> {
        self.delete(key.as_bytes())
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), Error> {
        let (p_ver, p, _) = self.idx.get_part(key);
        debug!("delete pver: {} part: {}", p_ver, p);

        if p_ver == 0 {
            return Ok(());
        }

        match self.load_pack(p_ver, p)? {
            None => {
                return Ok(())
            },
            Some(mut pack) => {
                pack.map.remove(key);

                let buf = pack.to_vec()?;

                debug!("put store kv cver: {} pver: {} part: {}", self.commit.ver, p_ver, p);
                self.kv.put(self.commit.ver, &p.to_be_bytes()[..], &buf)?;
                self.idx.set_part(p, self.commit.ver);
                debug!("index part set {:?}", self.idx.get_part(key));
            }
        }

        Ok(())
    }

    pub fn get_str(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        debug!("get key={}", key);

        self.get(key.as_bytes())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let (p_ver, p, _) = self.idx.get_part(key);
        debug!("get pver: {} part: {}", p_ver, p);

        if p_ver == 0 {
            return Ok(None);
        }

        let pack = self.load_pack(p_ver, p)?.ok_or(Error::PackNotFound(p_ver, p))?;
        let val = match pack.get(key) {
            None => None,
            Some((_, val_buf)) => {
                Some(val_buf.clone())
            }
        };

        Ok(val)
    }

}

pub struct ImmutableTree {
    pub (crate) t: Tree,
}

impl ImmutableTree {
    pub fn get_str(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        self.t.get_str(key)
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.t.get(key)
    }
}