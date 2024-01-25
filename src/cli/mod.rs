mod print;
mod delete;
mod init;

use std::path::PathBuf;

use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::manager::Manager};
use print::PrintStruct;
use init::InitStruct;

use self::delete::DelStruct;


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
    Print(PrintStruct),
    Init(InitStruct),
    Delete(DelStruct),

    New(NotDone),
    SetParent(NotDone),
    Tui(NotDone),
    AddFeat(NotDone),
    AddSubFeat(NotDone),
    DoneFeat(NotDone),
    Update(NotDone),
}

pub fn cli() -> ProjectResult<()>{
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
        _ => NotDone::default().run(params)?,
    }

    Ok(())
}
