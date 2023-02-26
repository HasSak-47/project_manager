use serde::{Serialize, Deserialize};
use toml::{map::Map, value::Value};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config{
    pub projects: Option<Map<String, Value>>,
    pub force: Force,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Force{
    pub commit_message: String,
    pub push_message: String,
}

impl Config{
    pub fn get_config() -> Self{
        let config_path = dirs::config_dir().unwrap().to_str().unwrap().to_string();
        let config_prmg = {let mut a = config_path.clone(); a.push_str("/project_manager"); a};
        let config_file = {let mut a = config_prmg.clone(); a.push_str("/config");          a};

        if !Path::new(&config_prmg).exists(){
            std::fs::create_dir(&config_prmg).unwrap();
        }

        if std::fs::File::open(&config_file).is_err(){
            std::fs::write(&config_file, DEFAULT_CONFIG).unwrap();
        }

        let config_data = String::from_utf8(std::fs::read(config_file).unwrap()).unwrap();
        toml::from_str(config_data.as_str()).unwrap()
    }
}

pub const DEFAULT_CONFIG: &str =
"[projects]
[force]
commit_message= \"Forced commit\"
push_message= \"Forced push\"";

