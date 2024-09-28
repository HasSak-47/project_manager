use std::{collections::HashMap, path::PathBuf, time::{self, Duration}};

use thiserror::Error;

#[derive(Debug, Default, Clone)]
struct Description {
    name       : String,
    description: String,
    priority   : f64,
    difficulty : f64,
    due_date   : Option<time::Instant>,

    tags       : Vec<String>,
}

#[derive(Debug, Default, Clone)]
struct TaskTable{
    desc: Description,
    done: bool,

    // minimun time needed to perform the task
    min_time   : time::Duration,

    id : usize,
    parent_task: Option<usize>,
    project    : Option<usize>,
}

#[derive(Debug, Clone)]
enum Location{
    Path(PathBuf),
    Git(String),
}

#[derive(Debug, Default, Clone)]
struct Project{
    desc: Description,
    last_worked: Option<time::Instant>,
    location: Option<Location>,
    parent: Option<usize>,
}

impl Project {
    fn new(desc: Description) -> Self{
        Self {desc, last_worked: None, location: None, parent: None}
    }
}

#[derive(Debug, Default, Clone)]
struct ProjectTable{
    desc: Description,
    last_worked: Option<time::Instant>,
    location: Option<Location>,

    id    : usize,
    parent: Option<usize>,
}

#[derive(Debug, Default, Clone)]
struct Pool{
    tasks: Vec<TaskTable>,
    projects: Vec<ProjectTable>,
}

#[derive(Debug, )]
struct ProjectManager<'a>{
    pool: &'a mut Pool,
    project_id: usize,
}

#[derive(Debug, )]
struct TaskManager<'a>{
    pool: &'a mut Pool,
    task_id: usize,
}

#[derive(Debug, Error)]
enum PoolError{
    #[error("project \"{name}\" was not found ")]
    ProjectNotFound{ name: String }
}

#[allow(dead_code)]
impl Pool{
    pub fn new() -> Self{
        return Self {
            tasks: Vec::new(),
            projects: Vec::new(),
        }
    }

    pub fn new_project(&mut self, desc: Description) -> Result<ProjectManager, PoolError>{
        let id = self.projects.last().and_then(|s| Some(s.id)).unwrap_or(self.projects.len());
        let project = ProjectTable{
            desc, id,
            parent: None,
            location: None,
            last_worked: None,
        };
        self.projects.push(project);

        Ok(ProjectManager { pool: self, project_id: id })
    }

    pub fn new_task(&mut self, desc: Description, done: bool) -> Result<TaskManager, PoolError>{
        let id = self.tasks.last().and_then(|s| Some(s.id)).unwrap_or(self.tasks.len());
        let task = TaskTable{
            desc, id, done,
            min_time: Duration::from_secs(60 * 60),
            parent_task: None,
            project: None,
        };
        self.tasks.push(task);

        Ok(TaskManager{ pool: self, task_id: id })
    }

    fn search_project_id(&mut self, name: String) -> Result<usize, PoolError>{
        return self.projects
            .iter()
            .find(|p| p.desc.name == name)
            .and_then(|p| Some(p.id))
            .ok_or(PoolError::ProjectNotFound { name });
    }

    pub fn search_project(&mut self, name: String) -> Result<ProjectManager, PoolError>{
        let project_id = self.search_project_id(name)?;
        Ok(ProjectManager { pool: self, project_id })
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
