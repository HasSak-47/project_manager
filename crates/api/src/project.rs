use super::Location;
use ly::proc::builder;
use serde::{Deserialize, Serialize};
use crate::*;
use crate::desc::{Descriptor, Description};


#[builder(name = Project, pass = derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    #[builder(ty = Descriptor)]
    pub desc: Description,

    #[builder(skip)]
    pub id: usize,

    #[builder(ty=String)]
    pub last_worked: Option<chrono::NaiveDate>,

    #[builder(init = Location::None)]
    #[builder(pass = serde(skip))]
    pub location: Location,

    #[builder(ty = String, init = String::new())]
    #[builder(pass = serde(default = "String::new"))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub parent: Option<usize>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    childs: Vec<Project>,

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    tasks: Vec<Task>,
}


impl ProjectTable {
    pub fn naive_project(self) -> Project{
        Project {
            desc: self.desc.naive_description(),
            last_worked: self.last_worked
                .and_then(|p| Some(p.format("%d %m %Y").to_string()))
                .unwrap_or(String::new()),
            location: self.location,
            ..Default::default()
        }
    }
}

use super::{Manager, ManagerMut};

pub type ProjectManager<'a> = Manager<'a, Project>;
pub type ProjectManagerMut<'a> = ManagerMut<'a, Project>;

impl<'a> ProjectManager<'a>{
    pub fn name(&self) -> &String{
        &self.get_table().desc.name
    }

    pub fn location(&self) -> &Location{
        &self.get_table().location
    }

    pub fn get_table(&self) -> &ProjectTable{
        &self.pool.projects[self.id]
    }

    pub fn get_parent_id(&self) -> Option<usize>{
        self.pool.projects[self.id].parent.clone()
    }

    pub fn get_completion(&self) -> f64{
        let mut todo = 0.;
        let mut done = 0.;
        let tasks = self.pool.get_tasks_where(|p| p.project.is_some_and(|p| p == self.id));
        for task in tasks{
            let (td, tt) = task.get_completion_pairs();
            done += td;
            todo += tt;
        }
        if todo + done == 0.{
            return 0.;
        }
        return done / (todo + done);
    }
}


