
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use parking_lot::RwLock;
use xxhash_rust::xxh3::xxh3_64;

use crate::Error;

#[derive(Deserialize, Serialize)]
struct IndexInner {
    ver: u16,
    prefix_bits: usize,
    version_list: Vec<u16>,
}

#[derive(Clone)]
pub (crate) struct Index {
    prefix_bits: usize,
    inner: Arc<RwLock<IndexInner>>,
    //db: Arc<Box<dyn KeyValue>>,
}

impl Index {
    pub fn new(prefix_bits: usize, ver: u16) -> Self {
        let len: usize = 2usize.pow(prefix_bits as u32);

        let inner = IndexInner {
            ver,
            prefix_bits,
            version_list: vec![0; len],
        };

        let idx = Index{
            prefix_bits,
            inner: Arc::new(RwLock::new(inner)),
            //db,
        };

        idx
    }

    pub fn set_version(&self, ver: u16) {
        let mut inner = self.inner.write();
        inner.ver = ver
    }

    pub fn new_with_buf(buf: &[u8]) -> Result<Self, Error> {
        let inner: IndexInner = rmp_serde::from_read_ref(buf)?;

        let idx = Index{
            prefix_bits: inner.prefix_bits,
            inner: Arc::new(RwLock::new(inner)),
            //db,
        };

        Ok(idx)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, Error> {
        let inner = self.inner.read();
        let buf = rmp_serde::to_vec(&*inner)?;
        Ok(buf)
    }

    fn hash(&self, key: &[u8]) -> (u32, [u8; 8]) {
        let h = xxh3_64(key);

        let part = h >> (64-self.prefix_bits);
        (part as u32, h.to_be_bytes())
    }

    pub fn get_part(&self, key: &[u8]) -> (u16, u32, [u8; 8]) {
        let (part, h) = self.hash(key);
        let inner = self.inner.read();
        let ver = inner.version_list[part as usize];

        (ver, part, h)
    }

    pub fn set_part(&self, part: u32, ver: u16) {
        let mut inner = self.inner.write();
        inner.version_list[part as usize] = ver;
    }
}