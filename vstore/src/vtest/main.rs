
use std::sync::Arc;
use std::path::Path;
use keyvalue::rocksdb::RocksDB;
use keyvalue::sqlite::SqliteDB;
use vstore::{VStore, Tree};
use log::info;

use anyhow::Error;

fn create_vstore() -> Result<VStore, Error> {
    //let dbdir = Path::new("./data");
    //let kv = RocksDB::new_box(&dbdir)?;
    let dbdir = Path::new("./test.db");
    let kv = SqliteDB::new_box(&dbdir)?;
    
    let kv = Arc::new(kv);

    let v = VStore::new(kv)?;
    Ok(v)
}

fn add(v: &Tree) -> Result<(), Error> {

    let val = "1";
    for i in 0..(1024 * 1024) {
        let key = format!("a-{}", i);
        v.put_str(key.as_str(), Vec::from(val.as_bytes()))?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    env_logger::init();

    info!("Creating vstore");
    let v = create_vstore()?;

    info!("Creating tree");
    let t = v.writable()?;

    info!("Adding keys");
    add(&t)?;

    v.commit(t)?;

    Ok(())
}