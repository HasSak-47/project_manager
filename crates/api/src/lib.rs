pub mod project;
pub mod task;

use std::{path::PathBuf, time::{self, Duration}};

use serde::{Deserialize, Serialize};
use task::Task;
use thiserror::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Description {
    name       : String,
    description: String,
    priority   : f64,
    difficulty : f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date   : Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags       : Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
enum Location{
    Path(PathBuf),
    Git(String),
}

#[derive(Debug, Default, Clone)]
pub struct Project{
    desc: Description,
    last_worked: Option<time::Instant>,
    location: Option<Location>,
    parent: Option<String>,
}

#[allow(dead_code)]
impl Project {
    fn new(desc: Description) -> Self{
        Self {desc, last_worked: None, location: None, parent: None}
    }

    fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        return self;
    }

    fn parent(mut self, parent: String) -> Self {
        self.parent = Some(parent);
        return self;
    }
    
    fn last_worked(mut self, last_worked: time::Instant) -> Self {
        self.last_worked = Some(last_worked);
        return self;
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    desc: Description,
    last_worked: Option<time::Instant>,
    location: Option<Location>,

    id    : usize,
    parent: Option<usize>,
}

use crate::task::TaskTable;

#[derive(Debug, Default, Clone)]
pub struct Pool{
    tasks: Vec<TaskTable>,
    projects: Vec<ProjectTable>,
}

#[derive(Debug, )]
pub struct ProjectManager<'a>{
    pool: &'a mut Pool,
    project_id: usize,
}

#[derive(Debug, )]
pub struct TaskManager<'a>{
    pool: &'a mut Pool,
    task_id: usize,
}

#[derive(Debug, Error)]
pub enum PoolError{
    #[error("project \"{name}\" was not found ")]
    ProjectNotFound{ name: String },

    #[error("task \"{name}\" was not found ")]
    TaskNotFound{ name: String },
}

#[allow(dead_code)]
impl Pool{
    pub fn new() -> Self{
        return Self {
            tasks: Vec::new(),
            projects: Vec::new(),
        }
    }

    pub fn new_project(&mut self, proj: Project) -> Result<ProjectManager, PoolError>{
        let id = self.projects.last().and_then(|s| Some(s.id)).unwrap_or(self.projects.len());
        let project = ProjectTable{
            desc: proj.desc,
            parent: proj.parent.and_then(|p| self.search_project_id(p).ok() ),
            location: proj.location,
            last_worked: proj.last_worked,
            id,
        };
        self.projects.push(project);

        Ok(ProjectManager { pool: self, project_id: id })
    }

    pub fn new_task(&mut self, task: Task) -> Result<TaskManager, PoolError>{
        todo!("do this well mate");
        Ok(TaskManager{ pool: self, task_id: 0 })
    }

    fn search_project_id(&mut self, name: String) -> Result<usize, PoolError>{
        return self.projects
            .iter()
            .find(|p| p.desc.name == name)
            .and_then(|p| Some(p.id))
            .ok_or(PoolError::ProjectNotFound { name });
    }

    fn search_task_id(&mut self, name: String) -> Result<usize, PoolError>{
        return self.tasks
            .iter()
            .find(|p| p.desc.name == name)
            .and_then(|p| Some(p.id))
            .ok_or(PoolError::TaskNotFound { name });
    }

    pub fn search_project(&mut self, name: String) -> Result<ProjectManager, PoolError>{
        let project_id = self.search_project_id(name)?;
        Ok(ProjectManager { pool: self, project_id })
    }

    pub fn search_task(&mut self, name: String) -> Result<TaskManager, PoolError>{
        let task_id = self.search_project_id(name)?;
        Ok(TaskManager { pool: self, task_id })
    }
}

#[allow(dead_code)]
impl<'a> ProjectManager<'a> {
    pub fn set_parent(&mut self, name: String) -> Result<(), PoolError>{
        let project_id = self.pool.search_project_id(name)?;
        self.pool.projects[self.project_id].parent = Some(project_id);
        Ok(())
    }

    pub fn set_description(&mut self, desc: String) -> Result<(), PoolError>{
        self.pool.projects[self.project_id].desc.description = desc;
        Ok(())
    }
}
