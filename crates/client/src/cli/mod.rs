mod print;
mod git;
mod features;
mod delete;
mod init;
mod new;

use std::{path::PathBuf, io::Write};

use clap::{Subcommand, Parser, Args};
use project_manager_api::{
    error::{ProjectResult, ProjectError},
    config::{manager::Manager, default::DEFAULT_MANAGER}, ProjectsHandler, ProjectLoader
};
use print::PrintStruct;
use init::InitStruct;

use crate::SystemHandler;

use self::{delete::DelStruct, new::NewStruct, features::AddFeat, git::GitStruct};

#[derive(Parser, Debug)]
#[clap(author="Daniel", version, about)]
struct Arguments{
    #[clap(short, long)]
    verbose: bool,
    #[clap(long)]
    debug: bool,

    #[clap(long)]
    manager_path: Option<PathBuf>,

    #[command(subcommand)]
    tree: Option<Tree>,
}

#[allow(dead_code)]
pub struct Params{
    verbose: bool,
    debug: bool,
    manager_path: PathBuf,
}

#[derive(Args, Default, Debug, Clone, Copy)]
struct NotDone;

impl NotDone{
    pub fn run(&self, _ : Params) -> ProjectResult<()>{
        println!("not yet implemented!!");
        Err(ProjectError::Other("not yet implemented".to_string()))
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

pub fn cli(handler: SystemHandler) -> ProjectResult<()>
{
    // set up stuff
    let args = Arguments::parse();
    if args.tree.is_none(){
        return Ok(());
    }

    let tree = args.tree.unwrap();
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
