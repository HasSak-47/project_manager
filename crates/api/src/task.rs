use ly::proc::builder;
use serde::{Deserialize, Serialize};

use crate::Description;
use std::time::Duration;

#[builder(Task)]
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    pub(crate) min_time: Duration,
    #[builder(other_type: String)]
    pub(crate) parent_task: Option<usize>,

    #[builder(other_type: String)]
    pub(crate) project    : Option<usize>,

    #[builder(skip)]
    pub(crate) id : usize,
}
