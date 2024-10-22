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
    #[builder(ty = u64, init = 10)]
    pub(crate) min_time: chrono::Duration,

    #[builder(skip)]
    pub(crate) id : usize,

    #[builder(skip)]
    pub(crate) parent : Option<usize>,

    #[builder(ty = String, init = String::new())]
    #[builder(pass = serde(default = "String::new"))]
    pub(crate) project: Option<usize>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    childs: Vec<Task>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    tags: Vec<Tag>,

}

impl TaskTable {
    
}
