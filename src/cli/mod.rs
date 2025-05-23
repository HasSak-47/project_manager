mod print;

use clap::{Subcommand, Parser, Args};
use crate::error::ProjectResult;
use print::PrintStruct;

#[derive(Parser, Debug)]
#[clap(author="Daniela", version, about)]
struct Arguments{
    #[clap(short, long)]
    verbose: bool,

    #[command(subcommand)]
    tree: Option<Tree>,
}

trait RunCmd{
    fn run(&self) -> ProjectResult<()>;
    fn run_verbose(&self) -> ProjectResult<()>{
        self.run()
    }
}

#[derive(Args, Default, Debug, Clone, Copy)]
struct NotDone;

impl RunCmd for NotDone{
    fn run(&self) -> ProjectResult<()>{
        println!("not yet implemented!!");
        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone, Copy)]
enum Tree{
    Print(PrintStruct),
    Init(NotDone),
    New(NotDone),
    SetParent(NotDone),
    Delete(NotDone),
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
    match tree{
        TR::Print(p) => p.run()?,
        _ => NotDone::default().run()?,
    }

    Ok(())
}
