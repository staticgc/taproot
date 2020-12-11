use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DirEntry {
    pub name: String,
    pub etype: u8,
    pub ino: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dir {
    pub entries: Vec<DirEntry>,
}

impl Dir {
    pub fn new() -> Self {
        Dir {
            entries: Vec::new(),
        }
    }
}