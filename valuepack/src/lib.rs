
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Pack {
    version: u32, 
    map: HashMap<Vec<u8>, (u16, Vec<u8>)>,
}


impl Pack {
    pub fn new() -> Self {
        Pack {
            version: 1,
            map: HashMap::new(),
        }
    }

    pub fn put(&mut self, ver: u16, key: &[u8], val: Vec<u8>) {
        self.map.insert(key.to_owned(), (ver, val));
    }

    pub fn get<'a>(&'a self, key: &[u8]) -> Option<&'a (u16, Vec<u8>)> {
        self.map.get(key)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        let buf = rmp_serde::to_vec(&self)?;
        Ok(buf)
    }

    pub fn from_buf(buf: &[u8]) -> Result<Pack, rmp_serde::decode::Error> {
        let p: Pack = rmp_serde::from_read_ref(buf)?;
        Ok(p)
    }
}
