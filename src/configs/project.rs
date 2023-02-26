use serde::{Serialize, Deserialize};
use super::config;
use toml;

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
    pub todo_path: String,
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

    pub const fn new() -> Project{
        Project {
               project: ProjectId{ name: String::new(), version: String::new(), folder: String::new(), todo_path: String::new()} ,
               local: Local { countdown: String::new(), add_all: false, force_commit: false }, 
               remote: Remote { push: false, remote: String::new(), countdown: String::new(), local_commit: true, force_push: true}
        }
    }

    pub fn get_projects(config: &config::Config) -> Result<Vec<Self>, ()>{
        // gets all projects path an name (name, path)
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
name = \"{}\"
version = \"{}\"
folder = \"{}\"
todo_path = \"{}\"
[local]
countdown = \"{}\"
add_all = {}
force_commit = {}
[remote]
push = {}
remote = \"{}\"
countdown = \"{}\"
local_commit = {}, 
force_push   = {}";
