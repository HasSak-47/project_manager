use std::{fs::File, io::Read};
use serde::{Serialize, Deserialize};
use dirs::config_dir;
use toml;

use std::io::BufReader;

use crate::error::*;

const CONFIG_PATH: &str = "project_manager/config.toml";

#[derive(Serialize, Deserialize)]
pub struct ManagerConfig{
    pub projects: Vec<String>,
    pub version : String,
}

impl std::default::Default for ManagerConfig{
    fn default() -> Self {
        Self {projects: vec!["/home".to_string()], version: "0.0.0".to_string()}
    }
}


pub fn load_config(path: String) -> ProjectResult<ManagerConfig>{
    let file_path = format!("{path}/{CONFIG_PATH}");
    let file = File::open(file_path).unwrap();

    let mut bufread = BufReader::new(file);
    let mut data = Vec::new();
    bufread.read_to_end(&mut data)?;

    let config : ManagerConfig  = toml::from_str(std::str::from_utf8(&data)?)?;
    Ok(config)
}

pub fn get_config() -> ManagerConfig{
    match get_dir(config_dir){
        Ok(path) => {load_config(path).unwrap_or(ManagerConfig::default())},
        Err(_) => {ManagerConfig::default()},
    }
}

