use std::collections::HashMap;

use config::{project::{Project, ProjectInfo}, manager::{self, ProjectData, Manager, Location}};
use error::{ProjectResult, ProjectError};

pub mod error;
pub mod utils;
pub mod config;

/* project cache that may be requested later
   it may need updating when the project is refreshed */
#[derive(Debug, Default, Clone)]
struct ProjectCache{
    _todo: Option<f64>,
    _done: Option<f64>,
    _comp: Option<f64>,
}

#[derive(Debug, Default, Clone)]
pub struct CachedProject {
    _name: String,
    _data: ProjectData,           // project info according to loader
    _cache: ProjectCache,         // stuff that is calculated when loading/reloading
    _proj: Option<Project>,       // the project when its loaded and not broken
    _loaded: bool,                // if the project is loaded
}

impl CachedProject{
    fn __load_project<L>(&mut self, loader: &L)
    where
        L: ProjectLoader,
    {
        let project : ProjectResult<Project> = 
            loader.get_project(&self._data.location)
                .and_then(|p| Ok(toml::from_str(p.as_str())?))
                .map_err(ProjectError::from)
        ;

        self._loaded = true;
        self._proj = project.ok();
    }

    pub fn get_completion_mut(&mut self) -> f64 {
        if self._cache._comp.is_some(){
            return self._cache._comp.unwrap();
        }
        if self._loaded && self._proj.is_some(){
            let comp = self._proj.as_ref().and_then(|p| Some(p.get_completion()) ).unwrap_or(0.);
            self._cache._comp = Some(comp);
            return comp;
        }
        0.
    }

    pub fn get_name(&self) -> &String{ &self._name }
}

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

    fn save(&self){}
    fn remove_project(&self){}
    pub fn load_projects(&mut self){
        for proj in self._projects.values_mut(){
            proj.__load_project(&self._loader);
        } 
    }

    fn drop_projects(&self){}
    fn drop_cache(&self){}
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

    pub fn add_project(&mut self, name: String, location: Location) -> ProjectResult<()>{
        let existing = self._projects.iter().find(|(pname, p)| *pname == &name || &p._data.location == &location).is_some();
        if existing {
            return Err(ProjectError::Other(format!("name {name} or location {location:?} already in manager")));
        }
        let c = CachedProject{
            _name: name.clone(),
            _data: ProjectData { location, ..Default::default() },
            ..Default::default()
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

    pub fn commit_project(&mut self, name: String) -> ProjectResult<()> {
        let p = &self._projects[&name];
        if p._proj.is_none(){
            return Err(ProjectError::Other("Project is not loaded!".to_string()));
        }
        self._loader.write_project(
            toml::to_string(&p._proj.as_ref().unwrap()).unwrap(),
            &p._data.location)?;
        Ok(())
    }

    pub fn commit_projects(&mut self) -> ProjectResult<()> {
        let keys : Vec<_> = self._projects.keys().map(|f| f.clone()).collect();
        for name in keys{
            self.commit_project(name.clone())?;
        }
        Ok(())
    }

    pub fn new_project(&mut self, name: String, location: Location) {
        let cached = CachedProject{
            _name : name.clone(),
            _data : ProjectData{
                location, ..Default::default()
            }, ..Default::default()
        };
        self._projects.insert(name, cached);
    }

    pub fn find_project_via_name(&self, name: String) -> ProjectResult<&CachedProject> {
        Err(ProjectError::Option)
    }
}
