use crate::create_vstore;
use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
pub struct HeadCmdArgs {
    pub store_path: String,
}

pub fn cmd(args: HeadCmdArgs) -> Result<(), Error> {
    let v = create_vstore(&args.store_path)?;
    let cs = v.commit_state();

    println!("head version: {}", cs.head_version());
    println!("open version: {}", cs.open_version());

    Ok(())
}