use serde::{Serialize, Deserialize};
use dirs::config_dir;
use toml::{self, map::Map};

use crate::error::*;

const CONFIG_PATH: &str = "project_manager/config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData{
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ManagerData{
    pub version : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManagerToml{
    pub manager : ManagerData,
    pub projects: Map<String, toml::Value>,
}

#[derive(Default, Debug)]
pub struct Manager{
    pub manager : ManagerData,
    pub projects: Vec<ProjectData>
}

impl std::default::Default for ManagerToml{
    fn default() -> Self {
        Self {
            projects: Map::new(),
            manager : ManagerData::default(),
        }
    }
}

fn map_to_data(m: Map<String, toml::Value>) -> Vec<ProjectData>{
    let mut r = Vec::new();
    for (k, v) in m{
        r.push(ProjectData{
            name: k,
            path: v.as_str().unwrap().to_string(),
        });
    }

    r
}


pub fn load_config<S: std::fmt::Display>(path: S)
    -> ProjectResult<Manager>
{
    let file_path = format!("{path}/{CONFIG_PATH}");
    let data = crate::utils::read_file(&file_path)?;

    let config : ManagerToml = toml::from_str(std::str::from_utf8(data.as_bytes())?)?;
    Ok(Manager{manager: config.manager, projects: map_to_data(config.projects)})
}

pub fn get_config() -> Manager{
    match get_dir(config_dir){
        Ok(path) => {load_config(path).unwrap_or(Manager::default())},
        Err(_) => {Manager::default()},
    }
}

