mod print;
mod delete;
mod init;
mod new;

use std::{path::PathBuf, io::Write};

use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::{manager::Manager, default::DEFAULT_MANAGER}};
use print::PrintStruct;
use init::InitStruct;

use self::{delete::DelStruct, new::NewStruct};

#[derive(Parser, Debug)]
#[clap(author="Daniela", version, about)]
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

pub struct Params{
    verbose: bool,
    debug: bool,
    manager_path: PathBuf,
}

trait RunCmd{
    fn run(&self, params: Params) -> ProjectResult<()>;
}

#[derive(Args, Default, Debug, Clone, Copy)]
struct NotDone;

impl RunCmd for NotDone{
    fn run(&self, _ : Params) -> ProjectResult<()>{
        println!("not yet implemented!!");
        Ok(())
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
    Tui(NotDone),
    AddFeat(NotDone),
    AddSubFeat(NotDone),
    DoneFeat(NotDone),
    Update(NotDone),
}

pub fn cli() -> ProjectResult<()>{
    // set up stuff
    let path = Manager::get_path()?;
    if !path.exists(){
        let mut f = std::fs::File::create(path)?;
        f.write( &DEFAULT_MANAGER.as_bytes())?;
    }


    let args = Arguments::parse();
    if args.tree.is_none(){
        return Ok(());
    }

    let tree = args.tree.unwrap();
    use Tree as TR;
    let params = Params{
        debug : args.debug,
        verbose : args.verbose,
        manager_path: args.manager_path.unwrap_or(Manager::get_path()?),
    };
    match tree{
        TR::Print(p) => p.run(params)?,
        TR::Init(i) => i.run(params)?,
        TR::Delete(d) => d.run(params)?,
        TR::New(n) => n.run(params)?,
        _ => NotDone::default().run(params)?,
    }

    Ok(())
}
