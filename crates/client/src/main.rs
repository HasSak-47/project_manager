mod cli;

use cli::cli;
use project_manager_api::error::ProjectResult;
use project_manager_api::config::manager::Manager;

#[allow(dead_code)]
const EDITION : &str = "0.1.0";

fn main() -> ProjectResult<()>{
    Manager::create_data()?;
    cli()?;
    Ok(())
}
