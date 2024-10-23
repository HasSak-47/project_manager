use ly::proc::builder;
use crate::desc::{Descriptor, Description};
use serde::{Deserialize, Serialize};

#[builder(name = Task, pass = derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    #[builder(ty = Descriptor)]
    pub desc: Description,
    pub done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    #[builder(ty = u64, init = 10)]
    pub min_time: chrono::Duration,

    #[builder(skip)]
    pub id : usize,

    #[builder(skip)]
    pub parent : Option<usize>,

    #[builder(ty = String, init = String::new())]
    #[builder(pass = serde(default = "String::new"))]
    pub project: Option<usize>,

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
    pub fn naive_task(self) -> Task{
        Task {
            desc: self.desc.naive_description(),
            done: self.done,
            min_time: self.min_time.num_minutes() as u64,
            project: String::new(),
            childs: Vec::new(),
            tags: Vec::new(),
        }
    }    
}
