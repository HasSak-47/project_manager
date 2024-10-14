mod list;
// mod rename;
// mod git;
// mod features;
// mod delete;
mod init;
mod new;
// mod done;
// mod mark_feature;
// mod utils;


use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Subcommand, Parser, Args};

use self::{
    // delete::DelStruct,
    // features::AddFeat,
    // git::GitStruct,
    init::InitStruct,
    new::NewStruct,
    list::ListStruct,
};

use project_manager_api::Database;

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
    // AddFeat(AddFeat),
    // AddSubFeat(NotDone),

    // Tui(NotDone),
    DoneFeat(NotDone),
    Update(NotDone),

    // MarkFeature(MarkFeature),
    // Git(GitStruct),
}

impl Arguments {
    pub fn debug<S: AsRef<str>>(&self, info: &S){
        let info = info.as_ref();
        if self.debug{
            println!("{}", info);
        }
    }
    
}

pub fn cli(mut db: Database) -> anyhow::Result<()> {
    // set up stuff
    let args = Arguments::parse();
    if args.tree.is_none(){
        return Err(anyhow!("no arguments given!"));
    }
    if args.debug {
        ly::log::set_level(ly::log::Level::Log);
    }

    let tree = args.tree.clone().unwrap();

    db.load_data()?;

    use Tree as TR;

    match tree{
        TR::List(l) => l.run(args, db)?,
        TR::Init(i) => i.run(args, db)?,
        TR::New(n) => n.run(args, db)?,
        // TR::Delete(d) => d.run(args, db)?,
        // TR::AddFeat(f) => f.run(args, db)?,
        // TR::Git(g) => g.run(args, db)?,
        // TR::MarkFeature(f) => f.run(args, db)?,
        _ => NotDone::default().run(args, db)?,
    }

    Ok(())
}
