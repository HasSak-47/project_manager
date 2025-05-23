mod config;
mod error;
mod utils;

use error::*;
#[allow(unused_imports)]
use config::{manager, project::load_project};

fn main() -> ProjectResult<()>{
    let project = load_project("/home/lilith/project_manager").unwrap();


    Ok(())
}
