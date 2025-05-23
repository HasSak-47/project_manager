use std::{
    fs::{DirBuilder, File},
    path::{PathBuf, Path},
    io::{Write, BufReader, Read, BufWriter},
};

use serde::{Serialize, Deserialize};
use dirs::data_local_dir;
use toml::{self, map::Map, Table, Value};

use crate::{error::*, utils::get_dir};

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
    pub fn read_buffer<R: Read>(reader: &mut BufReader<R>) -> ProjectResult<Self> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();
        let config : ManagerToml = toml::from_str(std::str::from_utf8(&data)?)?;

        Ok(Manager{manager: config.manager, projects: map_to_data(config.projects)})
    }

    pub fn write_buffer<W: Write>(&self, writer: &mut BufWriter<W>) -> ProjectResult<()> {
        let mn = ManagerToml{
            manager: self.manager.clone(),
            projects: data_to_map(&self.projects),
        };
        writer.write(toml::to_string(&mn)?.as_bytes())?;
        Ok(())
    }

    pub fn load_data_from<P: AsRef<Path>>(path: P) -> ProjectResult<Self> {
        let mut buf_reader = BufReader::new(File::open(path)?);
        Self::read_buffer(&mut buf_reader)
    }

    pub fn write_data_to<P: AsRef<Path>>(&self, path: P) -> ProjectResult<()> {
        let mut buf_write = BufWriter::new(File::create(path)?);
        self.write_buffer(&mut buf_write)
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

    #[allow(dead_code)]
    pub fn get_projects(&self) -> Vec<ProjectResult<Project>>{
        let mut v = Vec::new();
        
        for p in &self.projects{
            v.push(Project::read_project_from_dir(&p.path));
        }

        v
    }

    pub fn get_unbroken_projects(&self) -> Vec<Project>{
        let mut v = Vec::new();
        
        for p in &self.projects{
            let _p = Project::read_project_from_dir(&p.path);
            if _p.is_ok(){ v.push(_p.unwrap()); }
        }

        v
    }

    pub fn find_project_name(&self, name: String) -> ProjectResult<Project>{
        for proj in self.projects.iter(){
            if proj.name == name{
                return Ok(Project::read_project_from_dir(&proj.path)?);
            }
        }
        Err(ProjectError::ProjectNotFound { name: Some(name), path: None })
    }

    pub fn find_project_path<P: AsRef<Path>>(&self, path: P)
        -> ProjectResult<Project>
    {
        for proj in self.projects.iter(){
            if proj.path.as_path() == path.as_ref(){
                return Ok(Project::read_project_from_dir(&proj.path)?);
            }
        }
        Err(ProjectError::ProjectNotFound {
            name: None,
            path: Some(path.as_ref().to_path_buf())
        })
    }
}
