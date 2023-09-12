mod config;
mod error;

use error::*;

fn main() -> ProjectResult<()>{
    let config = config::get_manager_config();
    let folders = &config.projects;

    for folder in folders{
        println!("{folder}");
    }

    Ok(())
    
}
