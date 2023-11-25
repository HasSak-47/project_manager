mod cli;
mod update;
mod config;
mod error;
mod utils;
mod baked;

use cli::{PrintPercentajes, CliUtil, UpdateStatusEdition};
#[allow(unused_imports)]
use config::project::{Feature, Project, ProjectToml};
use error::*;
#[allow(unused_imports)]
use config::{manager::Manager, project};

#[allow(dead_code)]
const EDITION : &str = "0.1.0";

fn main() -> ProjectResult<()>{
    UpdateStatusEdition::run()?;

    Ok(())
}
