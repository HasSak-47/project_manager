use ly::proc::builder;
use serde::{Deserialize, Serialize};

use crate::Description;
use std::time::Duration;

#[builder(name = Task)]
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    pub(crate) min_time: Duration,
    #[builder(ty = String)]
    pub(crate) parent_task: Option<usize>,

    #[builder(ty = String)]
    pub(crate) project    : Option<usize>,

    #[builder_skip]
    pub(crate) id : usize,
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
}
