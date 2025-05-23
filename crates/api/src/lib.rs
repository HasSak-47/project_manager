pub mod project;
pub mod task;
pub mod trees;

use std::{path::PathBuf, time::{self, Duration}};

use serde::{Deserialize, Serialize};
use task::Task;
use thiserror::Error;
use trees::ProjectTree;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Description {
    name       : String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    priority   : f64,
    difficulty : f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    due_date   : String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags       : Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Git(String),
}

use crate::task::*;
use crate::project::*;

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
    #[error("could not load pool")]
    LoadingError,

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

    pub fn new_project(&mut self, project: Project) -> Result<ProjectManager, PoolError>{
        let entry = ProjectTable::from_project(project, self);
        let project_id = entry.id;
        self.projects.push(entry);

        return Ok(ProjectManager{ pool: self, project_id });
    }

    pub fn new_task(&mut self, task: Task) -> Result<TaskManager, PoolError>{
        let table_entry = TaskTable::from_task(task, self);
        let id = table_entry.id;
        self.tasks.push(table_entry);
        Ok(TaskManager{ pool: self, task_id: id })
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
        let project_id = self.search_project_id(name)?;
        Ok(ProjectManager { pool: self, project_id })
    }

    pub fn search_task(&mut self, name: &String, project: &String) -> Result<TaskManager, PoolError>{
        let task_id = self.search_task_id(name, project)?;
        Ok(TaskManager { pool: self, task_id})
    }
    
    pub fn load() -> Result<Self, PoolError>{
        return Err(PoolError::LoadingError);
    }

    pub fn add_project_tree(&mut self, tree: ProjectTree) -> Result<(), PoolError>{
        let (projects, tasks) = tree.flatten();
        for project in projects{
            let project = ProjectTable::from_project_result(project, self)?;
            self.projects.push(project);
        }
        for task in tasks{
            let task = TaskTable::from_task_result(task, self)?;
            self.tasks.push(task);
        }
        return Ok(());
    }
}

#[allow(dead_code)]
impl<'a> ProjectManager<'a> {
    pub fn get_table_mut(&mut self) -> &mut ProjectTable{
        return &mut self.pool.projects[self.project_id];
    }
    pub fn set_parent(&mut self, name: String) -> Result<(), PoolError>{
        let project_id = self.pool.search_project_id(&name)?;
        self.pool.projects[self.project_id].parent = Some(project_id);
        Ok(())
    }

    pub fn set_description(&mut self, desc: String) -> Result<(), PoolError>{
        self.pool.projects[self.project_id].desc.description = desc;
        Ok(())
    }
}
