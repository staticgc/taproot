
use crate::open_vstore;
use structopt::StructOpt;
use anyhow::Error;
use vstore::diff::DiffType;

#[derive(Debug, StructOpt)]
pub struct DiffCmdArgs {
    pub store_path: String,
    pub aver: u16,
    pub bver: u16,
}

pub fn cmd(args: DiffCmdArgs) -> Result<(), Error> {
    let v = open_vstore(&args.store_path)?;

    let mut d = v.diff(args.aver, args.bver)?;

    while let Some(item_vec) = d.next()? {
        for item in item_vec {
            let k = std::str::from_utf8(item.key.as_slice()).unwrap_or("binary");
            match item.diff_type {
                DiffType::New(new_val) => {
                    println!("new {} {}", k, new_val.len());
                }
                DiffType::Delete(last_ver,last_val) => {
                    println!("del {} {} {}", k, last_ver, last_val.len());
                }
                DiffType::Value(dv) => {
                    println!("mod {} {}->{}", k, dv.a_ver, dv.b_ver);
                },
            };
        }
    }

    Ok(())
}
