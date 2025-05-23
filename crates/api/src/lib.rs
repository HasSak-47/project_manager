use std::collections::HashMap;

use config::{project::{Project, ProjectInfo}, manager::{ProjectData, Manager, Location}};
use anyhow::{Result, anyhow};

pub use cached_project::*;

pub mod error;
pub mod cached_project;
pub mod config;

pub trait ProjectLoader{
    fn get_manager(&self) -> Result<String>;
    fn get_project(&self, location: &Location) -> Result<String>;
    fn write_manager(&mut self, data: String) -> Result<()>;
    fn write_project(&mut self, data: String, location: &Location) -> Result<()>;
    fn ensure_existance(&mut self) -> Result<()>;
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
    pub fn init(mut loader: Loader) -> Result<Self>{
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

    pub fn new_project(&mut self, name: String, location: Location) -> Result<()>{
        let existing_name = self._projects.iter().find(|(pname, _)| *pname == &name);
        let existing_path = self._projects.iter().find(|(_, p)| &p._data.location == &location);

        if existing_name.is_some() {
            return Err(anyhow!("Project name {name} is already managed!"));
        }
        if existing_path.is_some() {
            return Err(anyhow!("Project location {location} is already managed!"));
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

    pub fn commit_manager(&mut self) -> Result<()>{
        let mut manager = Manager::default();
        for (name, p) in self._projects.iter(){
            manager.projects.insert(name.clone(), p._data.clone());
        }

        self._loader.write_manager(toml::to_string(&manager).unwrap())?;
        Ok(())
    }

    pub fn commit_project(&mut self, name: String) -> Result<()> {
        let p = &self._projects[&name];
        if p._proj.is_none(){
            return Err(anyhow!("Project is missing or it is not loaded!"))
        }
        let tml = toml::to_string(&p._proj.as_ref().unwrap()).unwrap();
        self._loader.write_project(tml, &p._data.location)?;
        Ok(())
    }

    pub fn commit_projects(&mut self) -> Result<()> {
        let keys : Vec<_> = self._projects.keys().map(|f| f.clone()).collect();
        for name in keys {
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
}
