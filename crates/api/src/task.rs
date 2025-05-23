<<<<<<< HEAD
use ly::proc::builder;
use serde::{Deserialize, Serialize};

use crate::Description;
use std::time::Duration;

#[builder(Task)]
=======
use crate::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Task{
    desc: Description,
    done: bool,
    min_time: time::Duration,
    parent_task: Option<String>,
    project: Option<String>,
}

impl Task {
    pub const fn new(desc: Description) -> Self{
        return Self{
            desc, done: false,
            min_time: Duration::from_secs(60 * 60),
            parent_task: None,
            project: None,
        }
    }
    pub fn desc(mut self, desc: Description) -> Self { self.desc = desc; self }
    pub fn done(mut self, done: bool) -> Self { self.done = done; self }
    pub fn min_time(mut self, min_time: time::Duration) -> Self { self.min_time = min_time; self }
    pub fn parent_task(mut self, parent_task: String) -> Self { self.parent_task = Some(parent_task); self }
    pub fn project(mut self, parent_project: String) -> Self { self.project = Some(parent_project); self }
}

>>>>>>> afbb097 (pull)
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    pub(crate) desc: Description,
    pub(crate) done: bool,

<<<<<<< HEAD
    // minimun time needed to perform the task min_time   : time::Duration,
    pub(crate) min_time: Duration,
    #[builder(other_type: String)]
    pub(crate) parent_task: Option<usize>,

    #[builder(other_type: String)]
    pub(crate) project    : Option<usize>,

    #[builder(skip)]
    pub(crate) id : usize,
=======
    // minimun time needed to perform the task
    pub(crate) min_time   : time::Duration,

    pub(crate) id : usize,
    pub(crate) parent_task: Option<usize>,
    pub(crate) project    : Option<usize>,
}

impl TaskTable {
    pub(crate) fn from_task(task: Task, pool: &Pool)-> Self{
        let id = pool.tasks.last().and_then(|s| Some(s.id)).unwrap_or(pool.tasks.len());
        let parent_task = task.parent_task.and_then(|p| pool.search_task_id(p).ok() );
        let project = task.project.and_then(|p| pool.search_project_id(p).ok() );

        return Self{
            desc: task.desc,
            done: task.done,
            min_time: task.min_time,
            id, parent_task, project,
        }
    }
>>>>>>> afbb097 (pull)
}
