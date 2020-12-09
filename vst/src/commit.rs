
use crate::create_vstore;
use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
pub struct CommitCmdArgs {
    pub store_path: String,
}

pub fn cmd_commit(args: CommitCmdArgs) -> Result<(), Error> {
    let v = create_vstore(&args.store_path)?;

    let t = v.writable()?;

    v.commit(t)?;

    Ok(())
}