use ly::proc::builder;
use crate::*;

use crate::Description;
use std::{error::Error, time::Duration};
use serde::{Deserialize, Serialize};

#[builder(name = Task)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskTable{
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    #[builder(def_val = Duration::new(1000))]
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

        let parent_task = if !task.parent_task.is_empty() {
            pool.search_task_id(&task.parent_task, &task.project).ok()
        }
        else {
            None
        };

        let project = if !task.project.is_empty() {
            pool.search_task_id(&task.project, &task.project).ok()
        }
        else {
            None
        };

        return Self{
            desc: task.desc,
            done: task.done,
            min_time: task.min_time,
            id, parent_task, project,
        }
    }
    
    pub(crate) fn from_task_result(task: Task, pool: &Pool)-> Result<Self, PoolError> {
        let id = pool.tasks.last().and_then(|s| Some(s.id)).unwrap_or(pool.tasks.len());

        let parent_task = if !task.parent_task.is_empty() {
            Some(pool.search_task_id(&task.parent_task, &task.project)?)
        }
        else {
            None
        };

        let project = if !task.project.is_empty() {
            Some(pool.search_task_id(&task.project, &task.project)?)
        }
        else {
            None
        };

        return Ok(Self{
            desc: task.desc,
            done: task.done,
            min_time: task.min_time,
            id, parent_task, project,
        });
    }
}
