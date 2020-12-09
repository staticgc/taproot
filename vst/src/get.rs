
use crate::create_vstore;
use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
pub struct GetCmdArgs {
    pub store_path: String,
    pub ver: u16,
    pub key: String,
}

pub fn cmd_get(args: GetCmdArgs) -> Result<(), Error> {
    let v = create_vstore(&args.store_path)?;

    let t = v.immutable(args.ver)?;

    match t.get(args.key.as_bytes())? {
        None => println!("key not found"),
        Some(buf) => {
            println!("key len: {}", buf.len())
        }
    };

    Ok(())
}