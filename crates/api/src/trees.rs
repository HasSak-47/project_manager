use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::{task::Task, Description, Location};

// how at worst tasks and projects will look like
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskTree{
    pub desc: Description,
    pub done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    pub min_time: Duration,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub childs: Vec<TaskTree>,

}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectTree{
    pub desc: Description,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub last_worked: String,

    pub location: Option<Location>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub childs: Vec<ProjectTree>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<TaskTree>,
}

impl TaskTree{
    fn _flattern(&mut self, parent: String) -> Vec<Task>{
        let mut v = Vec::new();

        for child in self.childs{
            v.push(Task{
                desc: self.desc,
                done: self.done,
                min_time: self.min_time,
                parent: self.desc.name.to_string(),
            });
        }
        
        return v;
    }

    pub fn flattern(&mut self) -> Vec<Task>{
        let mut v = Vec::new();

        
        return v;
    }
}
impl ProjectTree {
}
