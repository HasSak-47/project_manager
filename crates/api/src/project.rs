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

    #[builder(init = Location::Other)]
    #[builder(pass = serde(default = "Location::default"))]
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
}

use super::Result;
