use toml::{Value, map::Map};
use serde::{Deserialize, Serialize};
use dirs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Manager{
    pub projects: Option<Map<String, Value>>,
}

fn create_manager() {
    let config_path = format!("{}/project_manager/config", dirs::config_dir().unwrap().to_str().unwrap());
    std::fs::write(config_path, "[projects]").unwrap();
}

#[allow(dead_code)]
pub fn load_manager() -> Manager{
    let config_path = format!("{}/project_manager/config", dirs::config_dir().unwrap().to_str().unwrap());
    
    let config_data = String::from_utf8( match std::fs::read(config_path){
        Ok(v) => v,
        Err(_) => {
            create_manager();
            return Manager{projects: None}
        },
    }).unwrap_or("[projects]".to_string());

    Manager::from(toml::from_str(config_data.as_str()).unwrap_or(Manager {projects : None}))
}
