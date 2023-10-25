mod config;
mod error;
mod utils;

use error::*;
#[allow(unused_imports)]
use config::{manager, project};

fn main() -> ProjectResult<()>{
    let manager = manager::get_config();
    for project in manager.projects{
        let p = project::load_project(project.path)?;
        println!("{p:?}");
    }


    Ok(())
}
