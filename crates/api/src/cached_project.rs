
use std::path::PathBuf;

use super::config::{project::Project, manager::{ProjectData, Location}};
use super::ProjectLoader;
use anyhow::Result;

/* project cache that may be requested later
   it may need updating when the project is refreshed */
#[derive(Debug, Default, Clone)]
pub(crate) struct ProjectCache{
    _todo: Option<f64>,
    _done: Option<f64>,
    _comp: Option<f64>,
}

#[derive(Debug, Default, Clone)]
pub struct CachedProject {
    pub(crate) _name: String,
    pub(crate) _data: ProjectData,           // project info according to loader
    pub(crate) _cache: ProjectCache,         // stuff that is calculated when loading/reloading
    pub(crate) _proj: Option<Project>,       // the project when its loaded and not broken
    pub(crate) _loaded: bool,                // if the project is loaded
}

pub enum FindCriteria{
    Location(Location),
    Name(String),
}

impl FindCriteria{
    pub const fn location(l: Location) -> Self{ FindCriteria::Location(l) }
    pub const fn path(path: PathBuf) -> Self{ FindCriteria::Location(Location::Path { path }) }
    pub const fn name(n: String) -> Self{ FindCriteria::Name(n) }
}

impl CachedProject{
    pub(crate) fn __load_project<L>(&mut self, loader: &L)
    where
        L: ProjectLoader,
    {
        let project : Result<Project> = 
            loader.get_project(&self._data.location)
                .and_then(|p| Ok(toml::from_str(p.as_str())?))
        ;

        self._proj = project.ok();
        self._loaded = true;
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

    pub fn get_completion(&self) -> f64 {
        return self._cache._comp.unwrap();
    }

    pub fn update(&mut self) {
        self._data.last_updated = Some(0);
    }

    pub fn cache_completion(&mut self) {
        self._data.last_updated = Some(0);
        if self._loaded && self._proj.is_some(){
            let comp = self._proj.as_ref().and_then(|p| Some(p.get_completion()) ).unwrap_or(0.);
            self._cache._comp = Some(comp);
        }

    }

    pub fn get_name(&self) -> &String{ &self._name }

    pub fn get_location(&self) -> Location {
        self._data.location.clone()
    }

    pub(crate) fn match_criteria(&self, find_criteria: &FindCriteria) -> bool{
        match find_criteria{
            FindCriteria::Location(local) => self._data.location == *local,
            FindCriteria::Name(name) => self._name == *name,
        }
    }
}

