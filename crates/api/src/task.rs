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

    #[builder(skip)]
    pub(crate) id : usize,

    #[builder(ty = String, init = String::new())]
    pub(crate) project: Option<usize>,

    #[builder(ty = String, init = String::new())]
    pub(crate) parent : Option<usize>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    childs: Vec<Task>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    tags: Vec<Tag>,

}

impl Task{
    fn _flatten(self, parent: String) -> Vec<Task>{
        let mut v = Vec::new();

        v.push(Task::new()
            .parent(parent).clone()
            .desc(self.desc.clone())
            .project(self.project.clone())
            .done(self.done)
            .min_time(self.min_time)
        );
        for child in self.childs{
            v.push(Task::new()
                .parent((&child.desc.name).clone())
                .project(self.project.clone())
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
