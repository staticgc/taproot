
use crate::stat::Stat;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Inode {
    pub ino: u64,
    pub path: String,
    pub stat: Stat,
}

impl Inode {
    pub fn new_file(i: u64, path: &str) -> Inode {
        Inode {
            ino: i,
            path: path.to_owned(),
            stat: Stat::new_file(i),
        }
    }

    pub fn new_dir(i: u64, path: &str) -> Inode {
        Inode {
            ino: i,
            path: path.to_owned(),
            stat: Stat::new_dir(i),
        }
    }
}