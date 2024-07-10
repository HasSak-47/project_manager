pub mod manager;
pub mod project;

use std::{collections::HashMap, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Ok, Result};

pub fn current_edition() -> String {
    return env!("CARGO_PKG_VERSION").to_string();
}

#[derive(Debug, Clone, PartialEq, Eq,
         Serialize, Deserialize)]
#[serde(tag = "loc_type", content = "location")]
pub enum Location{
    Path(PathBuf),
    Url(String),
}

pub trait Finder{
    fn matches(&self, project: &ProjectInfo) -> bool;
}

impl<T> Finder for &T
where
    T : Finder
{
    fn matches(&self, project: &ProjectInfo) -> bool {
        (*self).matches(project)
    }
}

impl Finder for String{
    fn matches(&self, project: &ProjectInfo) -> bool{
        project.name == *self
    }
}

impl Finder for Location{
    fn matches(&self, project: &ProjectInfo) -> bool{
        project.location == *self
    }
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
    pub fn get_project<F: Finder>(&self, req: F) -> Result<&Project>
    {
        let project_info = &self.find_project(req)?;

        self.projects
            .get(&project_info.name)
            .and_then(|p| Some(&p.project))
            .ok_or(anyhow!("Project {} isn't loaded!", project_info.name))
    }


    /**
     * gets a project from the cache
     * if the project status is not in the cache it loads it
     */
    pub fn get_project_mut<F: Finder>(&mut self, req: F) -> Result<&mut Project> {
        let project_info = &mut self.find_project_mut(req)?.clone();

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
        self.projects.insert(project_info.name.clone(), CachedProject::new(project));
        self.projects
            .get_mut(&project_info.name)
            .and_then(|p| Some(&mut p.project))
            .ok_or(anyhow!("ProjectHandler::get_project_mut Project {} wasn't found loaded!", project_info.name))
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
            let _ = self.get_project_mut(name);
        }
        Ok(())
    }

    /**
      load projects with status to the cache
     */

    pub fn load_projects_with_status(&mut self) -> Result<()>{
        let names : Vec<String> = self.manager.projects
            .iter()
            .map(|(_, p)| p.name.clone())
            .collect();
        for name in names{
            self.get_project_mut(name)?;
        }
        Ok(())
    }

    /**
     * writes a project to its location
     */
    pub fn commit_project<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
        let name = name.as_ref().to_string();
        let project = self.get_project(name)?.clone();
        self.project_handler.write(&project)?;
        Ok(())
    }


    pub fn commit_projects(&mut self) -> Result<()>{
        for (_, p) in self.projects.clone(){
            let _ = self.commit_project(&p.project.info.name);
        } Ok(())
        
    }

    /**
    writes the manager to its location
    */
    pub fn commit_manager(&mut self) -> Result<()>{
        self.manager_handler.write(&self.manager)
    }

    /**
     * returns a copy of all cached projects
     */
    pub fn get_cached_projects(&mut self) -> Vec<CachedProject>{
        self.projects.iter().map(|(_, p)| p.clone()).collect()
    }

    /**
     * gets all projects info in the manager
     */
    pub fn get_projects_info(&self) -> Vec<ProjectInfo>{
        self.manager.projects.iter().map(|(_, p)| p.clone()).collect()
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


    fn find_project<F>(&self, finder: F) -> Result<&ProjectInfo>
    where
        F: Finder
    {
        let project = self.manager.projects
            .iter()
            .find(|(_, p)| finder.matches(&p))
            .and_then(|(_, p)| Some(p))
            .ok_or(anyhow!("Handler::find_project Project not found!"))?;
        Ok(project)
    }

    /**
    Searches for a project in the cache
    */
    fn find_project_mut<F: Finder>(&mut self, finder: F) -> Result<&mut ProjectInfo>
    {
        let project = self.manager.projects
            .iter_mut()
            .find(|(_, p)| finder.matches(&p))
            .and_then(|(_, p)| Some(p))
            .ok_or(anyhow!("Handler::find_project_mut Project not found!"))?;
        Ok(project)
    }

    /**
    creates a status.toml in the location with an empty toml
    adds project to the database
    */
    pub fn new_project(&mut self, project: Project) -> Result<()>{
        self.project_handler.write(&project)?;
        let name = project.info.name.clone();

        self.manager.projects.insert(name.clone(), project.info.clone());
        let cached_proj = CachedProject { project, ..Default::default() };
        self.projects.insert(name, cached_proj);
        Ok(())
    }

    /**
    adds project to the database
    also loads the status.toml
    */
    pub fn init_project(&mut self, mut project: Project) -> Result<()>{
        self.project_handler.read(&mut project)?;
        let name = project.info.name.clone();

        self.manager.projects.insert(name.clone(), project.info.clone());
        let cached_proj = CachedProject { project, ..Default::default() };
        self.projects.insert(name, cached_proj);
        Ok(())
    }
}

impl CachedProject{
    pub fn get_name(&self) -> &String{ &self.project.info.name }
    pub fn get_location(&self) -> &Location { &self.project.info.location}

    pub fn new(project: Project) -> Self{
        let mut nw = Self{
            project,
            ..Default::default()
        };

        nw.todo = nw.project.status.as_ref().and_then(|x| Some(x.get_todo_difficulty()));
        nw.done = nw.project.status.as_ref().and_then(|x| Some(x.get_done_difficulty()));

        nw
    }

    pub fn get_completion(&self) -> f64{
        let todo = self.todo.unwrap_or(0) as f64;
        let done = self.done.unwrap_or(0) as f64;
        let total = todo + done;
        if total == 0.0 { return 0.0; }
        done / total
    }
}
