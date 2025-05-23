mod cli;
mod update;
mod config;
mod error;
mod utils;

use cli::run_cli;
use error::ProjectResult;
use config::manager::Manager;
use std::env::args;

#[allow(dead_code)]
const EDITION : &str = "0.1.0";

fn main() -> ProjectResult<()>{
    Manager::create_data()?;
    let mut args : Vec<_> = args().collect();
    args.remove(0);
    if args[0] == "tui"{
        
    }
    else{
        run_cli(args)?;
    }
    Ok(())
}
