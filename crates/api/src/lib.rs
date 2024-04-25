pub mod manager;
pub mod project;

use std::{collections::HashMap, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Ok, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Url(String),
}

#[derive(Debug, Clone)]
pub enum FindCriteria{
    Location(Location),
    Name(String),
}


impl Default for Location{ fn default() -> Self{ Location::Path(PathBuf::new()) } }

impl Location{
    pub fn path<P : AsRef<Path>>(path: P) -> Self{ Location::Path(path.as_ref().to_path_buf()) }
    pub fn url<S: AsRef<str>>(url: S) -> Self{ Location::Url(url.as_ref().to_string()) }

    pub fn get_path(&self) -> Result<&Path>{
        match self{
            Location::Path(path) => Ok(path.as_path()),
            _ => Err(anyhow!("Location is not a path!!")),
        }
    }

    pub fn get_url(&self) -> Result<&str>{
        match self{
            Location::Url(url) => Ok(url.as_str()),
            _ => Err(anyhow!("Location is not a url!!")),
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            Location::Path(path) => path.to_string_lossy().to_string(),
            Location::Url(url) => url.clone(),
        }
    }
}


use project::*;
use manager::*;

#[derive(Debug, Default, Clone)]
pub struct CachedProject{
    pub project: Project,

    pub todo: Option<usize>,
    pub done: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Handler{
    pub project_handler: ProjectHandler,
    pub manager_handler: ManagerHandler,
    pub manager: Manager,
    pub projects: HashMap<String, CachedProject>,
    _inited: bool,
}

impl Handler{
    pub fn new() -> Self{ Self::default() }

    // setup functions
    pub fn set_project_io<IO>(&mut self, io: IO)
    where
        IO: project::IO+ 'static
    { self.project_handler.set_io(Box::new(io)); }

    pub fn set_manager_io<IO>(&mut self, io: IO)
    where
        IO: manager::IO+ 'static { self.manager_handler.set_io(Box::new(io)); }

    /**
     loads the manager
     */
    pub fn init(&mut self) -> Result<()>{
        self.manager = self.manager_handler.read()?;
        self._inited = true;
        Ok(())
    }

    /**
     gets a project from the project cache
     */
    pub fn get_project<S: AsRef<str>>(&self, name: S) -> Result<&Project> {
        let name = name.as_ref().to_string();
        let project_info = self.manager.projects
            .get(&name)
            .ok_or(anyhow!("Project Not found!"))?;

        self.projects
            .get(&project_info.name)
            .and_then(|p| Some(&p.project))
            .ok_or(anyhow!("project not found"))
    }


    /**
     * gets a project from the cache
     * if the project is not in the cache it loads it
     */
    pub fn get_project_mut<S: AsRef<str>>(&mut self, name: S) -> Result<&mut Project> {
        let name = name.as_ref().to_string();
        let project_info = self.manager.projects
            .get_mut(&name)
            .ok_or(anyhow!("Project Not found!"))?;

        // there is already a project in the cache
        if self.projects.contains_key(&project_info.name){
            return Ok(&mut self.projects.get_mut(&project_info.name).unwrap().project);
        }

        // there is no project in the cache
        // and it starts loading it and insert it into the cache
        let mut project = Project{
            info: project_info.clone(),
            ..Default::default()
        };
        self.project_handler.read(&mut project)?;
        self.projects.insert(project_info.name.clone(), CachedProject{ project, ..Default::default()});
        self.projects
            .get_mut(&project_info.name)
            .and_then(|p| Some(&mut p.project))
            .ok_or(anyhow!("Project Not found!"))
    }

    /**
     * Load all projects in the manager to the cache
     */
    pub fn load_projects(&mut self) -> Result<()>{
        let names : Vec<String> = self.manager.projects
            .iter()
            .map(|(_, p)| p.name.clone())
            .collect();
        for name in names{
            let _ = self.get_project_mut(&name)?;
        }
        Ok(())
    }

    /**
     * writes a project to its location
     */
    pub fn commit_project<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
        let name = name.as_ref().to_string();
        let project = self.get_project(&name)?.clone();
        self.project_handler.write(&project)?;
        Ok(())
    }


    pub fn commit_projects(&mut self) -> Result<()>{
        for (_, p) in self.projects.clone(){
            let _ = self.commit_project(&p.project.info.name);
        } Ok(())
        
    }

    /**
     * returns a copy of all cached projects
     */
    pub fn get_cached_projects(&mut self) -> Vec<CachedProject>{
        self.projects.iter().map(|(_, p)| p.clone()).collect()
    }

    pub fn act_on_project<F>(&mut self, f: F) -> Result<()>
    where 
        F: Fn(&mut Project) -> Result<()>
    {
        for (_, p) in self.projects.iter_mut(){
            f(&mut p.project)?;
        } 
        Ok(())
    }

}

impl CachedProject{
    pub fn get_name(&self) -> &String{ &self.project.info.name }
    pub fn get_location(&self) -> &Location { &self.project.info.location}
}
