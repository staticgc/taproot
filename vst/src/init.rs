
use crate::create_vstore;
use anyhow::Error;


pub fn cmd_init(path: String) -> Result<(), Error> {
    let _ = create_vstore(&path)?;
    Ok(())
}