mod cli;
mod update;
mod config;
mod error;
mod utils;
mod baked;

use cli::{PrintPercentajes, CliUtil};
#[allow(unused_imports)]
use config::project::{Feature, Project, ProjectToml};
use error::*;
#[allow(unused_imports)]
use config::{manager::Manager, project};

#[allow(dead_code)]
const EDITION : &str = env!("CARGO_PKG_VERSION");

fn main() -> ProjectResult<()>{
    PrintPercentajes::run()?;

    Ok(())
}
