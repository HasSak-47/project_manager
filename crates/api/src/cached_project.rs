
use std::{path::PathBuf, time::{UNIX_EPOCH, Duration, SystemTime}};

use crate::{config::project::Feature, ProjectsHandler};

use super::config::{project::Project, manager::{ProjectData, Location}};
use super::ProjectLoader;
use anyhow::{Result, anyhow};

/* project cache that may be requested later
   it may need updating when the project is refreshed */
#[derive(Debug, Default, Clone)]
pub(crate) struct ProjectCache{
    _todo: Option<f64>,
    _done: Option<f64>,
    _comp: Option<f64>,
    _read_me: String,
}

#[derive(Debug, Default, Clone)]
pub struct CachedProject {
    pub(crate) _name: String,
    pub(crate) _data: ProjectData,           // project info according to loader
    pub(crate) _cache: ProjectCache,         // stuff that is calculated when loading/reloading
    pub(crate) _proj: Option<Project>,       // the project when its loaded and not broken
    pub(crate) _loaded: bool,                // if the project is loaded
}

#[derive(Debug)]
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
    pub fn load_project<L>(&mut self, loader: &L)
    where
        L: ProjectLoader,
    {
        let project : Result<Project> = 
            loader.get_project(&self._data.location)
                .and_then(|p| Ok(toml::from_str(p.as_str())?));

        self._proj = project.ok();
        self._loaded = true;
    }

    pub fn broken(&self) -> Option<bool>{
        if self._loaded { Some(self._proj.is_none()) }
        else { None}
        
    }

    pub fn get_last_updated(&self) -> Option<Duration>{
        self._data.last_updated
            .and_then(|t| Some(Duration::from_secs(t)))
    }

    pub fn get_completion_mut(&mut self) -> f64 {
        if self._cache._comp.is_some(){
            return self._cache._comp.unwrap();
        }
        if self._loaded && self._proj.is_some(){
            let comp = self._proj
                .as_ref()
                .and_then(|p| Some(p.get_completion())).unwrap_or(0.);
            self._cache._comp = Some(comp);
            return comp;
        }
        0.
    }

    pub fn get_completion(&self) -> f64 {
        self._cache._comp.unwrap_or(0.)
    }

    pub fn update(&mut self) -> Result<()>{
        let delta = SystemTime::now().duration_since(UNIX_EPOCH)?;
        self._data.last_updated = Some(delta.as_secs());
        Ok(())
    }

    pub fn cache_completion(&mut self) -> Result<()>{
        if self._loaded && self._proj.is_some(){
            let comp = self._proj.as_ref().and_then(|p| Some(p.get_completion()) ).unwrap_or(0.);
            self._cache._comp = Some(comp);
            return Ok(())
        }
        Err(anyhow!("Project was not loaded!"))

    }

    pub fn get_name(&self) -> &String{ &self._name }

    pub fn get_location(&self) -> &Location {
        &self._data.location
    }

    pub(crate) fn match_criteria(&self, find_criteria: &FindCriteria) -> bool{
        match find_criteria{
            FindCriteria::Location(local) => self._data.location == *local,
            FindCriteria::Name(name) => self._name == *name,
        }
    }

    pub fn add_todo(&mut self, f: Feature) {
        if let Some(ref mut p) = self._proj{
            p.add_todo(f)
        }
    }

    // searches for a feature in the project 
    // removes it from todo and adds it to done
    pub fn mark_todo_done(&mut self, f: String) {
        self._proj.as_mut().and_then(|p| Some(p.mark_done(f)));
    }

    pub fn add_done(&mut self, f: Feature) {
        if let Some(ref mut p) = self._proj{
            p.add_done(f)
        }
    }
}

// prints the project name
// features and completion
pub fn format_project(p: &CachedProject) -> String {
    let name = p.get_name();
    let comp = p.get_completion();
    let mut buffer = String::new();
    buffer += &format!("{}: {:.2}%\n", name, comp * 100.);
    p._proj.as_ref().and_then(|p| {
        // prints feature tree
        let todo_vec = p.todo.as_ref().and_then(|v| Some(v.clone())).unwrap_or(Vec::new());
        let done_vec = p.done.as_ref().and_then(|v| Some(v.clone())).unwrap_or(Vec::new());

        buffer += "Todo:\n";
        for todo in todo_vec{
            buffer += &format!("{todo}");
        }
        buffer += "Done:\n";
        for done in done_vec{
            buffer += &format!("{done}");
        }
        Some(())
    });


    buffer
}
