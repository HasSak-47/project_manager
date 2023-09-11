use std::fs::File;

use dirs::{home_dir, config_dir};
use serde::{Serialize, Deserialize};

mod config;
mod error;

use error::*;


fn get_dir(a: fn() -> Option<std::path::PathBuf>) -> ProjectResult<String>{
    use ProjectError as PE;
    let str = a().ok_or(PE::DirNotFound)?.to_str().ok_or(PE::DirToStr)?.to_string();
    Ok(str)
}

#[derive(Serialize, Deserialize, Default)]
struct Config{
    folders: Vec<String>
}

const CONFIG_PATH: &str = "project_manager/config";

fn main() -> ProjectResult<()>{



    let config_file= format!("{}/{CONFIG_PATH}", get_dir(config_dir)?);
    let file = File::open(config_file);

    let mut project_folder = Vec::new(); 
    project_folder.push(get_dir(home_dir)?);

    Ok(())
    
}
