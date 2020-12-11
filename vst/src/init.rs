
use crate::create_vstore;
use anyhow::Error;


pub fn cmd_init(path: String) -> Result<(), Error> {
    create_vstore(&path)?;
    Ok(())
}