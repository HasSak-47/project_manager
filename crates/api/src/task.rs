use ly::proc::builder;
use crate::*;

use crate::desc::{Descriptor, Description};

use std::time::Duration;
use serde::{Deserialize, Serialize};

#[builder(name = Task, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    #[builder(ty = Descriptor)]
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    #[builder(init = Duration::new(1000, 0))]
    pub(crate) min_time: Duration,

    #[builder(ty = String)]
    pub(crate) parent_task: Option<usize>,

    #[builder(ty = String)]
    pub(crate) project    : Option<usize>,

    #[builder(skip)]
    pub(crate) id : usize,
}

impl TaskTable {
    pub(crate) fn from_task(task: Task, db: &Database)-> Result<Self> {
        let id = db.search_entry_id(&task)?;

        let parent_task = if !task.parent_task.is_empty() {
            Some(db.search_entry_id(&task.parent_task)?)
        }
        else {
            None
        };

        let project = if !task.project.is_empty() {
            Some(db.search_entry_id(&task.project)?)
        }
        else {
            None
        };

        return Ok(Self{
            desc: Description::from_descriptor( task.desc, db )?,
            done: task.done,
            min_time: task.min_time,
            id, parent_task, project,
        });
    }
}
