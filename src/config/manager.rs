use std::{fs::{DirBuilder, File}, path::{PathBuf, Path},
    io::{Write, BufReader, BufRead, Read, BufWriter},
};

use serde::{Serialize, Deserialize};
use dirs::{config_dir, data_local_dir};
use toml::{self, map::Map, Table, Value};

use crate::{error::*, utils::get_dir};

const DATA_PATH: &str = "project_manager/config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData{
    pub name: String,
    pub path: PathBuf,
    pub ignore: Option<bool>,
    pub subprojects: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ManagerData{
    pub version : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManagerToml{
    pub manager : ManagerData,
    pub projects: Table,
}

#[derive(Serialize, Default, Debug)]
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
            path: v.as_str().unwrap().to_string().into(),
            ignore: None,
            subprojects: None,
        });
    }

    r
}

fn data_to_map(v: &Vec<ProjectData>) -> Table{
    let mut map = Table::new();
    for p in v{
        map.insert(
            p.name.clone(),
            Value::String(p.path.to_str().unwrap().to_string())
            );
    }

    map
}

use super::project::Project;
impl Manager{
    pub fn load_data_from<P: AsRef<Path>>(path: P)
        -> ProjectResult<Self>
    {
        let data = crate::utils::read_file(path)?;
        let config : ManagerToml = toml::from_str(std::str::from_utf8(data.as_bytes())?)?;

        Ok(Manager{manager: config.manager, projects: map_to_data(config.projects)})
    }

    pub fn read_buffer<R: Read>(reader: &mut BufReader<R>) -> ProjectResult<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();
        let config : ManagerToml = toml::from_str(std::str::from_utf8(&data)?)?;

        Ok(Manager{manager: config.manager, projects: map_to_data(config.projects)})
    }

    pub fn write_buffer<W: Write>(&self, writer: &mut BufWriter<W>) -> ProjectResult<()>
    {
        writer.write(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn write_data_to<P: AsRef<Path>>(&self, path: P)
        -> ProjectResult<()>
    {
        let mut file = File::create(path)?;
        let data = data_to_map(&self.projects);
        let toml = ManagerToml {
            manager: self.manager.clone(),
            projects: data,
        };
        let buffer = toml::to_string(&toml)?;
        file.write(buffer.as_bytes())?;
        Ok(())
    }

    pub fn get_path() -> ProjectResult<PathBuf>{
        let mut path = get_dir(data_local_dir)?;
        path.push("project_manager/projects");
        path.set_extension("toml");

        Ok(path)
    }

    // creates the data folder and the projects.toml file
    pub fn create_data() -> ProjectResult<()>{
        let mut path = get_dir(data_local_dir)?;
        path.push("project_manager");
        if !path.exists(){
            DirBuilder::new().create(&path)?;
        }
        path.push("projects");
        path.set_file_name("toml");
        if !path.exists() {
            File::create(path)?;
        }
        Ok(())
    }

    pub fn get_projects(&self) -> ProjectResult<Vec<Project>>{
        let mut v = Vec::new();
        
        for p in &self.projects{
            v.push(Project::load_project(&p.path)?);
        }

        Ok(v)
    }

    pub fn get_unbroken_projects(&self) -> Vec<Project>{
        let mut v = Vec::new();
        
        for p in &self.projects{
            let _p = Project::load_project(&p.path);
            if _p.is_ok(){
                v.push(_p.unwrap());
            }
            else{
                // println!("broken: {:?}", _p);
            }
        }

        v
    }
}
