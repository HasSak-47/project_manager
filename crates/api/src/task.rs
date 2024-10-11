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

