mod util;
mod init;
mod get;
mod put;
mod commit;
mod head;
mod diff;
mod delete;

use util::*;

use structopt::StructOpt;
use anyhow::Error;

#[derive(Debug, StructOpt)]
#[structopt(about = "Cli for vstore")]
enum Cli {
    Init {
        path: String,
    },
    Get(get::GetCmdArgs),
    Head(head::HeadCmdArgs),
    Put(put::PutCmdArgs),
    Commit(commit::CommitCmdArgs),
    Diff(diff::DiffCmdArgs),
    Delete(delete::DelCmdArgs),
}


fn main() -> Result<(), Error> {
    env_logger::Builder::from_env("VST_LOG").init();

    let opt = Cli::from_args();
    match opt {
        Cli::Init{path} => {
            init::cmd_init(path)?;
        },
        Cli::Get(args) => {
            get::cmd_get(args)?;
        },
        Cli::Put(args) => {
            put::cmd_put(args)?;
        },
        Cli::Commit(args) => {
            commit::cmd_commit(args)?;
        }
        Cli::Head(args) => {
            head::cmd(args)?;
        },
        Cli::Diff(args) => {
            diff::cmd(args)?;
        },
        Cli::Delete(args) => {
            delete::cmd(args)?;
        },
    }
    Ok(())
}
