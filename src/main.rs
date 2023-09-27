mod config;
mod error;

use error::*;
use config::{manager, project};

fn main() -> ProjectResult<()>{
    let config = manager::get_config();
    let folders = &config.projects;

    for folder in folders{
        println!("{folder}");
    }

    Ok(())
    
}
