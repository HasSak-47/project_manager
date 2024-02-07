/*
 * this is just the toml parser thingy
 */
use std::{
    fs::{DirBuilder, File},
    path::{PathBuf, Path},
    io::{Write, BufReader, Read, BufWriter}, time::{SystemTime, UNIX_EPOCH}, collections::HashMap,
};

use serde::{Serialize, Deserialize};
use toml::{self, map::Map, Table, Value};

use crate::{error::*, utils::get_dir};

#[cfg(target_arch="x86_64")]
use dirs::data_local_dir;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum Location{
    Path{path: PathBuf},
    Other{local: String},
}

impl Location{
    pub fn path(path: PathBuf) -> Self{ Self::Path {path} }
}

impl Default for Location{ fn default() -> Self { Self::Other{local: String::new()} } }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectData{
    pub location: Location,
    pub last_updated: Option<u64>,
    pub subprojects: Option<Vec<String>>, // Vector of the name of the projects
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manager{
    pub version : String,
    pub projects: HashMap<String, ProjectData>,
}
/*
use super::project::Project;

impl ProjectData{
    pub fn load_projec(&self) -> ProjectResult<Project> {
        Project::read_project_from_dir(&self.path)
    }
}

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

    pub fn find_project<P>(&self, predicate: P) -> ProjectResult<&ProjectData>
    where
        P : FnMut(&&ProjectData) -> bool,
    {
        self.projects
            .iter()
            .find(predicate)
            .ok_or(ProjectError::ProjectNotFound { name: None, path: None })
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
*/

/*
 * Stuff that is not available for wasm
 */
#[cfg(target_arch="x86_64")]
impl Manager{
    pub fn get_path() -> ProjectResult<PathBuf>{
        let mut path = get_dir(data_local_dir)?;
        path.push("project_manager/projects");
        path.set_extension("toml");

        Ok(path)
    }
}
