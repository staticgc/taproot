
use crate::create_vstore;
use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
pub struct DelCmdArgs {
    pub store_path: String,
    pub key: String,
}

pub fn cmd(args: DelCmdArgs) -> Result<(), Error> {
    let v = create_vstore(&args.store_path)?;

    let t = v.writable()?;
    t.delete(args.key.as_bytes())?;

    v.sync_tree(&t)?;

    Ok(())
}