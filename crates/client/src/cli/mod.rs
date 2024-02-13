mod print;
mod git;
mod features;
mod delete;
mod init;
mod new;


use anyhow::{Result, anyhow};
use clap::{Subcommand, Parser, Args};

use print::PrintStruct;
use init::InitStruct;

use std::path::PathBuf;

use crate::SystemHandler;
use self::{
    delete::DelStruct,
    new::NewStruct,
    features::AddFeat,
    git::GitStruct
};

#[derive(Parser, Debug)]
#[clap(author="Daniel", version, about)]
pub struct Arguments{
    #[clap(short = 'V', long)]
    version: bool,
    #[clap(short, long)]
    verbose: bool,
    #[clap(long)]
    debug: bool,

    #[clap(long)]
    manager_path: Option<PathBuf>,

    #[command(subcommand)]
    tree: Option<Tree>,
}

#[derive(Args, Default, Debug, Clone, Copy)]
struct NotDone;

impl NotDone{
    pub fn run(&self, _ : Arguments, _handler: SystemHandler) -> Result<()>{
        Err(anyhow!("not yet implemented"))
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Tree{
    // Daemon(DaemonStruct),
    Print(PrintStruct),
    Init(InitStruct),
    Delete(DelStruct),
    New(NewStruct),

    SetParent(NotDone),
    SetSubproject(NotDone),

    AddFeat(AddFeat),
    AddSubFeat(NotDone),

    Tui(NotDone),
    DoneFeat(NotDone),
    Update(NotDone),

    Git(GitStruct),
}

pub fn cli(handler: SystemHandler) -> Result<()>
{
    // set up stuff
    let args = Arguments::parse();
    if args.version{
        println!("version: {}", env!("CARGO_PKG_VERSION"));
    }
    if args.tree.is_none(){
        return Ok(());
    }

    let tree = args.tree.clone().unwrap();
    use Tree as TR;

    match tree{
        TR::Print(p) => p.run(args, handler)?,
        TR::Init(i) => i.run(args, handler)?,
        TR::Delete(d) => d.run(args, handler)?,
        TR::New(n) => n.run(args, handler)?,
        TR::AddFeat(f) => f.run(args, handler)?,
        TR::Git(g) => g.run(args, handler)?,
        _ => NotDone::default().run(args, handler)?,
    }

    Ok(())
}
