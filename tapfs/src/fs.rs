
use std::sync::Arc;
use std::path::{Path, PathBuf};
use vstore::{Tree, VStore};
use parking_lot::{Mutex, RwLock};

use keyvalue::{compress::CompressKV, sqlite::SqliteDB, KeyValue};
use crate::state::State;
use crate::Error;

#[derive(Clone)]
pub struct FileSystemArgs {
    pub read_only: bool,
    pub version: u16,
}

pub struct FileSystem {
    args: FileSystemArgs,
    state: Arc<RwLock<State>>,
}

impl FileSystem {
    pub fn create(args: &FileSystemArgs, v: Arc<VStore>) -> Result<Self, Error> {
        let tree = v.writable()?;

        let mut state = State {
            ino: Arc::new(Mutex::new(0)),
            vstore: v,
            tree: Arc::new(tree),
        };

        let fs = FileSystem {
            args: args.clone(),
            state: Arc::new(RwLock::new(state)),
        };

        Ok(fs)
    }

    fn init(s: &mut State) -> Result<(), Error> {
        s.write_ino_num(0)?;

        Ok(())
    }

    fn check_init(s: &State) -> Result<bool, Error> {
        let flag = match s.tree.get_str("fs.init")? {
            None => false,
            Some(b) => b.len() > 0 && b[0] == 10
        };
        Ok(flag)
    }

    fn mark_init(s: &State) -> Result<(), Error> {
        s.tree.put_str("fs.init", &[10][..])?;
        Ok(())
    }
}

