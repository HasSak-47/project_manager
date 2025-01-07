mod list;
// mod rename;
// mod git;
mod features;
// mod delete;
mod init;
mod new;
// mod done;
mod mark_feature;
// mod utils;

use std::path::PathBuf;

use ly::log::prelude::*;
use ly::log::write::ANSI;

use project_manager_api::Database;

use anyhow::{Result, anyhow};
use clap::{Subcommand, Parser, Args};

use self::{
    // delete::DelStruct,
    features::AddFeat,
    mark_feature::MarkFeature,
    // git::GitStruct,
    init::InitStruct,
    new::NewStruct,
    list::ListStruct,
};

use crate::utils::load_database;


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
    pub fn run(&self, _ : Arguments, _db: Database) -> Result<()>{
        Err(anyhow!("not yet implemented"))
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Tree{
    // Daemon(DaemonStruct),
    List(ListStruct),
    Init(InitStruct),
    // Delete(DelStruct),
    New(NewStruct),

    // SetParent(NotDone),
    // SetSubproject(NotDone),
    AddFeat(AddFeat),
    // AddSubFeat(NotDone),

    // Tui(NotDone),
    DoneFeat(NotDone),
    Update(NotDone),

    MarkFeature(MarkFeature),
    // Git(GitStruct),
}

pub fn cli() -> anyhow::Result<()> {
    // set up stuff
    ly::log::set_logger(ANSI::new());

    let args = Arguments::parse();
    if args.tree.is_none(){
        return Err(anyhow!("no arguments given!"));
    }
    if args.debug {
        ly::log::set_level(ly::log::Level::Debug);
        log!("running in debug...");
    }
    else{
        ly::log::set_level(ly::log::Level::Error);
    }

    let tree = args.tree.clone().unwrap();

    let mut db = Database::default();
    load_database(&mut db)?;

    use Tree as TR;

    match tree{
        TR::List(l) => l.run(args, db)?,
        TR::Init(i) => i.run(args, db)?,
        TR::New(n)  => n.run(args, db)?,
        // TR::Delete(d) => d.run(args, db)?,
        TR::AddFeat(f) => f.run(args, db)?,
        // TR::Git(g) => g.run(args, db)?,
        TR::MarkFeature(f) => f.run(args, db)?,
        _ => NotDone::default().run(args, db)?,
    }

    Ok(())
}
