use std::sync::Arc;
use vstore::{Tree, VStore};

use parking_lot::Mutex;

use crate::inode::Inode;
use crate::dir::Dir;
use crate::Error;

#[derive(Clone)]
pub (crate) struct State {
    pub(crate) ino: Arc<Mutex<u64>>,
    pub(crate) vstore: Arc<VStore>,
    pub(crate) tree: Arc<Tree>,
}

impl State {
    pub fn write_ino_num(&self, ino: u64) -> Result<(), Error> {
        let val = ino.to_be_bytes();
        self.tree.put_str("/m/ino", &val[..])?;
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.vstore.sync_tree(&self.tree)?;
        Ok(())
    }

    pub fn create_root(&mut self) -> Result<(), Error> {
        let ino = self.gen_ino();
        let ino_obj = Inode::new_dir(ino, "/r/");
        let d = Dir::new(); 

        self.write_ino(&ino_obj)?;
        self.write_dir("/r/",&d)?;
        Ok(())
    }

    pub fn read_dir(&self, path: &str) -> Result<Option<Dir>, Error> {
        match self.tree.get_str(&format!("/r{}", path))? {
            None => Ok(None),
            Some(buf) => {
               Ok(Some(rmp_serde::decode::from_read_ref(&buf)?))
            }
        }
    }

    pub fn write_dir(&self, path: &str, d: &Dir) -> Result<(), Error> {
        self.tree.put_str(&format!("/r{}", path), &rmp_serde::encode::to_vec(d)?)?;
        Ok(())
    }

    pub fn write_ino(&self, i: &Inode) -> Result<(), Error> {
        self.tree.put_str(&format!("/i/{}", i.ino), &rmp_serde::encode::to_vec(i)?)?;
        Ok(())
    }

    pub fn gen_ino(&self) -> u64 {
        let ino = self.ino.lock();
        *ino += 1;
        *ino
    }
}
