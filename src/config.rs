
const CONFIG_PATH: &str = "project_manager/config";

#[derive(Serialize, Deserialize, Default)]
pub struct Config{
    folders: Vec<String>
}

use error::*;

pub fn get_config() -> Config{
    let config_file= format!("{}/{CONFIG_PATH}", get_dir(config_dir)?);
    let file = File::open(config_file);
}
