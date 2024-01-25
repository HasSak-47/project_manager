#![allow(dead_code)]
#![allow(unused_imports)]
mod cli;
mod update;
mod config;
mod error;
mod utils;

use cli::cli;
use error::ProjectResult;
use config::manager::Manager;

#[allow(dead_code)]
const EDITION : &str = "0.1.0";

fn main() -> ProjectResult<()>{
    Manager::create_data()?;
    cli()?;
    Ok(())
}
