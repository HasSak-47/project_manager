use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};

pub enum Location{
    Path(PathBuf),
    Url(String),
}

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
}

pub trait ProjectIO{
    fn write_project<S: AsRef<str>>(&mut self, str: S) -> Result<()>;
    fn read_project(&self, location: Location) -> Result<String>;
}

pub trait ManagerIO{
    fn write_manager<S: AsRef<str>>(&mut self, str: S) -> Result<()>;
    fn read_manager(&self, location: Location) -> Result<String>;
}

pub trait Loader: ProjectIO + ManagerIO {}

#[allow(dead_code)]
pub struct Feature{
    name: String,
    description: String,
    status: String,
    priority: u8,
    difficulty: u8,

    todo: Vec<Feature>,
    done: Vec<Feature>,
}

// the status of the project
#[allow(dead_code)]
pub struct ProjectStatus{
    pub name: String,
    pub description: String,
    pub todo: Vec<Feature>,
    pub done: Vec<Feature>,
}

// info on the project
#[allow(dead_code)]
pub struct ProjectInfo{
    pub name: String,
    pub project_location: Location,
    pub status_location: Option<Location>, // if None, status is in the project folder
    pub last_update: Option<usize>, // timestamp
}

// the project inside the manager
pub struct Project{
    pub info: ProjectInfo,
    pub status: Option<Box<ProjectStatus>>, // the project may not be loaded
}

#[allow(dead_code)]
pub struct Manager{
    pub projects: Vec<ProjectInfo>,
}

#[allow(dead_code)]
pub struct ProjectHandler<'a, 'b, Handler: ProjectIO>{
    handler: &'a mut Handler,
    project: &'b mut Project,
}

#[allow(dead_code)]
pub struct ManagerHandler<'a, 'b, Handler: ManagerIO>{
    handler: &'a mut Handler,
    manager: &'b mut Manager,
}

#[allow(dead_code)]
pub struct Handler<L: Loader>{
    handler: L,
    manager: Manager,
}

impl<L : Loader> Handler<L>{
    pub fn new(loader: L) -> Self{
        Handler{
            handler: loader,
            manager: Manager{projects: Vec::new()},
        }
    }

    pub fn get_manager(&mut self) -> ManagerHandler<L>{
        ManagerHandler{
            handler: &mut self.handler,
            manager: &mut self.manager,
        }
    }
}
