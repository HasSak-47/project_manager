#![allow(unused_import_braces)]
mod cli;

use cli::cli;
use project_manager_api::error::ProjectResult;
use project_manager_api::config::manager::Manager;

fn main() -> ProjectResult<()>{
    Manager::create_data()?;
    cli()?;
    Ok(())
}
