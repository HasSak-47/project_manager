use serde::{Deserialize, Serialize};

use crate::Description;
use std::time::Duration;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskTree{
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_time: Option<Duration>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) childs : Option<Vec<TaskTree>>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) project: Option<String>,
}

impl TaskTree{
    fn extract_kids(self, buf: &mut Vec<Task>) {
        if let Some(v) = self.childs{
            for child in v{
                buf.push(Task {
                    desc:     self.desc,
                    done:     self.done,
                    min_time: self.min_time,
                    project:  self.project,
                    parent_task: Some(self.desc.name),
                });
                child.extract_kids(buf);
            }
        }
    }

    pub fn unravel(mut self) -> Vec<Task>{
        let mut buf = Vec::new();
        self.extract_kids(&mut buf);
        return buf;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Task{
    desc: Description,
    done: bool,

    // minimun time needed to perform the task
    min_time   : Option<Duration>,

    parent_task: Option<String>,
    project    : Option<String>,
}

#[allow(dead_code)]
impl Task{
    fn new(desc: Description) -> Self{
        return Self {desc,
            done: false,
            min_time: None,
            parent_task: None,
            project: None
        };
    }

    fn min_time(mut self, min_time: Duration) -> Self{
        self.min_time = Some(min_time);
        return self;
    }

    fn parent_task(mut self, parent_task: String) -> Self{
        self.parent_task = Some(parent_task);
        return self;
    }

    fn project(mut self, project: String) -> Self{
        self.project = Some(project);
        return self;
    }

    fn done(mut self) -> Self{
        self.done = true;
        return self;
    }
}

#[derive(Debug, Default, Clone)]
pub struct TaskTable{
    pub(crate) desc: Description,
    pub(crate) done: bool,

    // minimun time needed to perform the task min_time   : time::Duration,
    pub(crate) min_time: Duration,
    pub(crate) id : usize,
    pub(crate) parent_task: Option<usize>,
    pub(crate) project    : Option<usize>,
}
