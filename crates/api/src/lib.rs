pub mod project;
pub mod task;
pub mod trees;
pub mod desc;

use std::{marker::PhantomData, path::PathBuf, time::{self, Duration, SystemTime}};

use crate::task::*;
use crate::project::*;
use crate::desc::*;

use serde::{Deserialize, Serialize};
use task::{Task, TaskTable};
use thiserror::Error;
use trees::ProjectTree;
use ly::log::prelude::*;

type Timestamp = SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Git(String),
}

#[derive(Debug, )]
pub struct Manager<'a, T>{
    pool: &'a mut Pool,
    id: usize,
    t: PhantomData<T>,
}

#[derive(Debug, Default, Clone)]
pub struct Pool{
    tasks: Vec<TaskTable>,
    projects: Vec<ProjectTable>,
}

impl<'a, T> Manager<'a, T> {
    fn new(id: usize, pool: &'a mut Pool) -> Self{
        Self {id, pool, t: PhantomData}
    }
}

pub type ProjectManager<'a> = Manager<'a, ProjectTable>;
pub type TaskManager<'a> = Manager<'a, TaskTable>;

#[derive(Debug, Error)]
pub enum PoolError{
    #[error("Passed project was empty")]
    PassedEmptyProject,

    #[error("could not load pool")]
    LoadingError,

    #[error("project \"{name}\" was not found ")]
    ProjectNotFound{ name: String },

    #[error("task \"{name}\" was not found ")]
    TaskNotFound{ name: String },

    #[error("{other}")]
    Other{other: String},

    #[error("unknown")]
    Unknown,
}

impl PoolError{
    pub fn from_error<E: std::error::Error>(e: E) -> Self{
        Self::Other {other: e.to_string()}
    }
}

#[allow(dead_code)]
impl Pool{
    pub fn new() -> Self{
        return Self {
            tasks: Vec::new(),
            projects: Vec::new(),
        }
    }

    pub fn new_project(&mut self, project: Project) -> Result<ProjectManager, PoolError>{
        let entry = ProjectTable::from_project(project, self)?;
        let id = entry.id;
        self.projects.push(entry);

        Ok(Manager::new(id, self))
    }

    pub fn new_task(&mut self, task: Task) -> Result<TaskManager, PoolError>{
        let table_entry = TaskTable::from_task(task, self)?;
        let id = table_entry.id;
        self.tasks.push(table_entry);
        Ok(Manager::new(id, self))
    }

    fn search_project_id(&self, name: &String) -> Result<usize, PoolError>{
        return self.projects
            .iter()
            .find(|p| p.desc.name == *name)
            .and_then(|p| Some(p.id))
            .ok_or(PoolError::ProjectNotFound { name: name.clone() });
    }

    fn search_task_id(&self, name: &String, project: &String) -> Result<usize, PoolError>{
        let project = if project.is_empty(){
            None
        }
        else{
            Some(self.search_project_id(project)?)
        };
        return self.tasks
            .iter()
            .find(|t| t.project == project && t.desc.name == *name)
            .and_then(|t| Some(t.id))
            .ok_or(PoolError::TaskNotFound { name: name.clone() });
    }

    pub fn search_project(&mut self, name: &String) -> Result<ProjectManager, PoolError>{
        let id = self.search_project_id(name)?;
        Ok(Manager::new( id, self))
    }

    pub fn search_task(&mut self, name: &String, project: &String) -> Result<TaskManager, PoolError>{
        let id = self.search_task_id(name, project)?;
        Ok(Manager::new( id, self))
    }
    
    pub fn add_project_tree(&mut self, tree: ProjectTree) -> Result<(), PoolError>{
        let (projects, tasks) = tree.flatten();
        if projects.is_empty(){
            return Err(PoolError::PassedEmptyProject);
        }
        for project in projects{
            let project = ProjectTable::from_project(project, self)?;
            self.projects.push(project);
        }
        for task in tasks{
            let task = TaskTable::from_task(task, self)?;
            self.tasks.push(task);
        }
        return Ok(());
    }
}

#[allow(dead_code)]
impl<'a> ProjectManager<'a> {
    pub fn get_table_mut(&mut self) -> &mut ProjectTable{
        return &mut self.pool.projects[self.id];
    }
    pub fn set_parent(&mut self, name: String) -> Result<(), PoolError>{
        let project_id = self.pool.search_project_id(&name)?;
        self.pool.projects[self.id].parent = Some(project_id);
        Ok(())
    }

    pub fn set_description(&mut self, desc: String) -> Result<(), PoolError>{
        self.pool.projects[self.id].desc.description = desc;
        Ok(())
    }
}
