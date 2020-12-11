use std::sync::Arc;
use std::path::Path;

use vstore::VStore;
use keyvalue::{sqlite::SqliteDB, compress::CompressKV, KeyValue};

use anyhow::Error;



pub fn create_kv(p: &str) -> Result<Box<dyn KeyValue>, Error> {
    let kv = SqliteDB::new_box(&Path::new(p))?;
    Ok(kv)
}

pub fn create_vstore(p: &str) -> Result<VStore, Error> {
    let kv = create_kv(p)?;
    let kv = CompressKV::new(Arc::new(kv));
    let v = VStore::create(Arc::new(kv))?;

    Ok(v)
}

pub fn open_vstore(p: &str) -> Result<VStore, Error> {
    let kv = create_kv(p)?;
    let kv = CompressKV::new(Arc::new(kv));
    let v = VStore::open(Arc::new(kv))?;

    Ok(v)
}