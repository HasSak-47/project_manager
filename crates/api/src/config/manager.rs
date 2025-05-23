use core::time;
use std::{
    fs::{DirBuilder, File},
    path::{PathBuf, Path},
    io::{Write, BufReader, Read, BufWriter}, time::{SystemTime, UNIX_EPOCH},
};

use serde::{Serialize, Deserialize};
use dirs::data_local_dir;
use toml::{self, map::Map, Table, Value};

use crate::{error::*, utils::get_dir};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectData{
    pub name: String,
    pub path: PathBuf,
    pub last_updated: Option<u64>,
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
            ..Default::default()
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
        self.projects.iter()
            .map(|p| Project::read_project_from_dir(&p.path))
            .collect()
    }

    pub fn get_unbroken_projects(&self) -> Vec<Project>{
        self.get_projects()
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|p| p.unwrap())
            .collect()
    }

    pub fn find_project_name(&self, name: String) -> ProjectResult<Project>{
        self.projects.iter()
            .find(|p| p.name == name)
            .ok_or(ProjectError::ProjectNotFound { name: Some(name), path: None })
            .and_then(|p| Project::read_project_from_dir(&p.path))
    }

    pub fn find_project_path<P: AsRef<Path>>(&self, path: P) -> ProjectResult<Project> {
        let path = path.as_ref();
        self.projects.iter()
            .find(|p| p.path.as_path() == path)
            .ok_or(ProjectError::ProjectNotFound { name: None, path: Some(path.to_path_buf()) })
            .and_then(|p| Project::read_project_from_dir(&p.path))
    }

    pub fn find_project<P>(&self, predicate: P) -> ProjectResult<Project> 
    where
        P : FnMut(&&ProjectData) -> bool,
    {
        self.projects.iter()
            .find(predicate)
            .ok_or(ProjectError::ProjectNotFound { name: None, path: None })
            .and_then(|p| Project::read_project_from_dir(&p.path))
    }

    pub fn update_project<S: AsRef<str>>(&mut self, name: S) -> ProjectResult<()>{
        let name = name.as_ref();
        let project = self .projects.iter_mut()
            .find(|p| p.name.as_str() == name)
            .ok_or(ProjectError::Option)?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        project.last_updated = Some(now);

        Ok(())
    }
}
