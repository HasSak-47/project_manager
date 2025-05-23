mod config;
mod error;

use dirs::config_dir;
use error::*;
use config::{manager, project};

fn main() -> ProjectResult<()>{
    let config = manager::load_config(getdir(config_dir))?;
    let folders = &config.projects;

    for folder in folders{
        println!("{folder}");
    }

    Ok(())
    
}
