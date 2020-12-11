use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const INODE_TYPE_FILE: u8 = 0;
pub const INODE_TYPE_DIR: u8 = 1;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Stat {
    pub i: u64,
    pub m: u64,
    pub s: u64,
    pub t: u8, 
}

impl Stat {
    pub fn new_file(i: u64) -> Self {
        let m = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

        Stat {
            i,
            m: m.as_secs(),
            s: 0,
            t: INODE_TYPE_FILE,
        }
    }

    pub fn new_dir(i: u64) -> Self {
        let m = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

        Stat {
            i,
            m: m.as_secs(),
            s: 0,
            t: INODE_TYPE_DIR,
        }
    }
}