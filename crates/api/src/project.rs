use super::Location;
use ly::proc::builder;
use serde::{Deserialize, Serialize};
use crate::*;
use crate::desc::{Descriptor, Description};

#[builder(name = Project, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
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

    #[builder(skip_table)]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    #[builder(pass = serde(default = "Vec::new"))]
    tags: Vec<Tag>,
}

