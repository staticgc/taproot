
use crate::open_vstore;
use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
pub struct PutCmdArgs {
    pub store_path: String,
    pub key: String,
    pub value: String,
}

pub fn cmd_put(args: PutCmdArgs) -> Result<(), Error> {
    let v = open_vstore(&args.store_path)?;

    let t = v.writable()?;
    t.put(args.key.as_bytes(), args.value.as_bytes())?;

    v.sync_tree(&t)?;

    Ok(())
}