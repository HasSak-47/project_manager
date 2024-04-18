mod print;
mod rename;
mod git;
mod features;
mod delete;
mod init;
mod new;
mod mark_feature;
mod utils;


use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Subcommand, Parser, Args};

use print::PrintStruct;
use init::InitStruct;

use crate::{SystemHandler, SystemLoader};
use self::{
    delete::DelStruct, features::AddFeat, git::GitStruct, mark_feature::MarkFeature, new::NewStruct
};

#[derive(Parser, Debug)]
#[clap(author="Daniel", version, about)]
pub struct Arguments{
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

    MarkFeature(MarkFeature),

    Git(GitStruct),
}

pub fn cli() -> Result<()> {
    // set up stuff
    let args = Arguments::parse();
    if args.tree.is_none(){
        return Ok(());
    }

    let tree = args.tree.clone().unwrap();

    let mut loader = SystemLoader::new();
    loader.manager_path = if args.manager_path.is_some(){
        if args.debug{
            println!("setting path to: {:?}", args.manager_path.clone().unwrap());
        }
        args.manager_path.clone().unwrap()
    }
    else{
        let mut dir = dirs::data_dir().unwrap();
        dir.push("project_manager");
        dir.push("projects");
        dir.set_extension("toml");
        dir
    };
    if args.debug{
        println!("handler path: {:?}", loader.manager_path);
    }
    let handler = SystemHandler::init(loader)?;

    use Tree as TR;

    match tree{
        TR::Print(p) => p.run(args, handler)?,
        TR::Init(i) => i.run(args, handler)?,
        TR::Delete(d) => d.run(args, handler)?,
        TR::New(n) => n.run(args, handler)?,
        TR::AddFeat(f) => f.run(args, handler)?,
        TR::Git(g) => g.run(args, handler)?,
        TR::MarkFeature(f) => f.run(args, handler)?,
        _ => NotDone::default().run(args, handler)?,
    }

    Ok(())
}
