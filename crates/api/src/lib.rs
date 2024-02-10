use std::{collections::HashMap, path::PathBuf};

use config::{project::{Project, ProjectInfo}, manager::{ProjectData, Manager, Location}};
use error::{ProjectResult, ProjectError};
pub use cached_project::*;

pub mod error;
pub mod cached_project;
pub mod utils;
pub mod config;

pub trait ProjectLoader{
    fn get_manager(&self) -> ProjectResult<String>;
    fn get_project(&self, location: &Location) -> ProjectResult<String>;
    fn write_manager(&mut self, data: String) -> ProjectResult<()>;
    fn write_project(&mut self, data: String, location: &Location) -> ProjectResult<()>;
    fn ensure_existance(&mut self) -> ProjectResult<()>;
}

#[derive(Debug, Default)]
pub struct ProjectsHandler<Loader: ProjectLoader>{
    _loader: Loader,
    _projects : HashMap<String, CachedProject>,
}

impl<Loader> ProjectsHandler<Loader>
where
    Loader: ProjectLoader
{
    pub fn init(mut loader: Loader) -> ProjectResult<Self>{
        match loader.ensure_existance(){
            Ok(_) => {},
            Err(_) => loader.write_manager(
                toml::to_string(&Manager::default())?
            )?,
        };
        loader.ensure_existance().unwrap();
        let manager : Manager = 
            loader
            .get_manager()
            .and_then(|p| Ok(toml::from_str(p.as_str())))??;
        let _projects = manager.projects
            .into_iter()
            .map(|(name, pdata)| (name.clone(), CachedProject{
                _name: name, _data: pdata, ..Default::default()
            }))
            .collect();
        let s = Self { _loader : loader, _projects };
        Ok(s)
    }

    pub fn load_projects(&mut self) {
        for proj in self._projects.values_mut(){
            proj.__load_project(&self._loader);
        } 
    }

    #[allow(dead_code)]
    fn drop_projects(&self){
        todo!("find project via name");
    }

    #[allow(dead_code)]
    fn drop_cache(&self){
        todo!("find project via name");
    }

    pub fn get_project_mut(&mut self, name: String) -> Option<&mut CachedProject>{
        self._projects.get_mut(&name)
    }

    pub fn get_project(&self, name: String) -> Option<&CachedProject>{
        self._projects.get(&name)
    }

    pub fn get_projects_mut(&mut self) -> Vec<&mut CachedProject>{
        self._projects
            .values_mut()
            .collect()
    }

    pub fn get_projects(&mut self) -> Vec<&CachedProject>{
        self._projects
            .values()
            .collect()
    }

    pub fn new_project(&mut self, name: String, location: Location) -> ProjectResult<()>{
        let existing = self._projects.iter().find(|(pname, p)| *pname == &name || &p._data.location == &location).is_some();
        if existing {
            return Err(ProjectError::Other(format!("name {name} or location {location:?} already in manager")));
        }
        let c = CachedProject{
            _name: name.clone(),
            _data: ProjectData { location, ..Default::default() },
            _loaded: true,
            _proj: Some(Project{
                project: ProjectInfo{
                    name: name.clone(),
                    ..Default::default()
                },
                ..Default::default()
            }),
            _cache: ProjectCache::default(),
        };

        self._projects.insert(name, c);
        Ok(())
    }

    pub fn commit_manager(&mut self) -> ProjectResult<()>{
        let mut manager = Manager::default();
        for (name, p) in self._projects.iter(){
            manager.projects.insert(name.clone(), p._data.clone());
        }

        self._loader.write_manager(toml::to_string(&manager).unwrap())?;
        Ok(())
    }

    pub fn remove_project(&mut self, name: String) -> ProjectResult<()>{
        self._projects.remove(&name);
        Ok(())
    }

    pub fn commit_project(&mut self, name: String) -> ProjectResult<()> {
        let p = &self._projects[&name];
        if p._proj.is_none(){
            return Err(ProjectError::Other("Project is not loaded or it's broken!".to_string()));
        }
        let tml = toml::to_string(&p._proj.as_ref().unwrap()).unwrap();
        self._loader.write_project(tml, &p._data.location)?;
        Ok(())
    }

    pub fn commit_projects(&mut self) -> ProjectResult<()> {
        let keys : Vec<_> = self._projects.keys().map(|f| f.clone()).collect();
        for name in keys{
            self.commit_project(name.clone())?;
        }
        Ok(())
    }

    pub fn add_project(&mut self, name: String, location: Location) {
        let cached = CachedProject{
            _name : name.clone(),
            _data : ProjectData{
                location, ..Default::default()
            }, ..Default::default()
        };
        self._projects.insert(name, cached);
    }

    pub fn find_via_name(&self, _name: String) -> ProjectResult<&CachedProject> {
        todo!("find project via name");
    }

    pub fn find_via_path(&self, _path: PathBuf) -> ProjectResult<&CachedProject> {
        todo!("find project via name");
    }
}
