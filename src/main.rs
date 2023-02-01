use std::path::Path;

mod manager;
mod configs;

use configs::project::{self, Project};
use configs::config::{self, Config};


fn main(){
    let config_path = dirs::config_dir().unwrap().to_str().unwrap().to_string();
    let config_prmg = {let mut a = config_path.clone(); a.push_str("/project_manager"); a};
    let config_file = {let mut a = config_prmg.clone(); a.push_str("/config");          a};

    if !Path::new(&config_prmg).exists(){
        std::fs::create_dir(&config_prmg).unwrap();
    }

    if std::fs::File::open(&config_file).is_err(){
        std::fs::write(&config_file, config::DEFAULT_CONFIG).unwrap();
    }

    let config_data = String::from_utf8(std::fs::read(config_file).unwrap()).unwrap();
    let data : Config = toml::from_str(config_data.as_str()).unwrap();

    let projects = Project::get_projects(&data).unwrap();

    manager::child(data, projects);
}
