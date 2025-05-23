use serde::{Serialize, Deserialize};
use super::config;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
pub struct Project{
    pub project: ProjectId,
    pub local  : Local,
    pub remote : Remote,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectId{
    pub name    : String,
    pub version : String,
    pub folder  : String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Local{
    pub countdown   : String,
    pub add_all     : bool,
    pub force_commit: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Remote{
    pub push        : bool,
    pub remote      : String,
    pub countdown   : String,
    pub local_commit: bool,
    pub force_push  : bool,
}

impl Project{
    pub fn get_projects(config: &config::Config) -> Result<Vec<Self>, ()>{
        let projects = match &config.projects{
            Some(map) => {
                map.iter().fold(Vec::<(String, String)>::new(), |mut vec, entry|{
                    match &entry.1{
                        toml::Value::String(s) => vec.push((entry.0.clone(), s.clone())),
                        _ => {},
                    }
                    vec
                })
            },
            None => {return Err(())},
        };
    
        let mut v = Vec::new();
        for project in projects {
            let project_data = String::from_utf8(std::fs::read(&project.1).unwrap()).unwrap();
            let project_config : Project = toml::from_str(project_data.as_str()).unwrap();

            if std::fs::read_dir(&project_config.project.folder).is_ok() == true {
                v.push(project_config);
            }
        }
    
        Ok(v)
    }
}


pub const PROJECT_CONFIG_TEMPLATE: &str =
"[project]
name= \"{}\"
version= \"{}\"
folder= \"{}\"

[local]
countdown= 0.5
add_all= false
force_commit= false

[remote]
remote= \"\"
countdown= 1.0
force_push= false";

