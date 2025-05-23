use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::{project::Project, task::Task, desc::{Description, Descriptor}, Location, Result};

// how at worst tasks and projects will look like
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskTree{
    pub desc: Descriptor,
    pub done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    pub min_time: Duration,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub childs: Vec<TaskTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub project: String,

}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectTree{
    pub desc: Descriptor,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_worked: String,

    #[serde(default = "Location::default")]
    pub location: Location,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub childs: Vec<ProjectTree>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<TaskTree>,
}

impl TaskTree{
    fn _flatten(self, parent: String) -> Vec<Task>{
        let mut v = Vec::new();

        v.push(Task::new()
            .parent_task(parent).clone()
            .desc(self.desc.clone())
            .done(self.done)
            .min_time(self.min_time)
        );
        for child in self.childs{
            v.push(Task::new()
                .parent_task((&child.desc.name).clone())
                .desc(child.desc.clone())
                .done(child.done)
                .min_time(child.min_time)
            );

            let mut c = child._flatten(self.desc.name.clone());
            v.append(&mut c);
        }
        
        return v;
    }

    pub fn flatten(self) -> Vec<Task>{
        return self._flatten(String::new());
    }
}
impl ProjectTree {
    fn _flatten(self, parent: String) -> Result<(Vec<Project>, Vec<Task>)>{
        let mut projects = Vec::new();
        let mut tasks = Vec::new();
        let name = self.desc.name.clone();

        projects.push(Project::new()
            .parent(parent)
            .desc(self.desc)
            .last_worked(self.last_worked)
            .location(self.location.clone())
        );
        for task in self.tasks{
            let mut tt = task.flatten();
            tasks.append(&mut tt);
        }

        for child in self.childs{
            let (mut cp, mut cv) = child._flatten(name.clone())?;
            projects.append(&mut cp);
            tasks.append(&mut cv);
        }
        return Ok((projects, tasks));
    }

    pub fn flatten(self) -> Result<(Vec<Project>, Vec<Task>)>{
        return self._flatten(String::new());
    }
}
