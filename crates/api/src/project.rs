use super::Location;
use ly::proc::builder;
use serde::{Deserialize, Serialize};
use crate::*;
use crate::desc::{Descriptor, Description};

#[builder(name = Project, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    #[builder(ty = Descriptor)]
    pub(crate) desc: Description,

    #[builder(ty=String)]
    pub(crate) last_worked: Option<Timestamp>,

    #[builder(init = Location::Other)]
    #[builder(pass = serde(default = "Location::default"))]
    pub(crate) location: Location,

    #[builder(skip)]
    pub(crate) id: usize,
    #[builder(skip)]
    pub(crate) parent: Option<usize>,

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

pub struct FlatProject{
    desc: Descriptor,
    last_worked: String,
    location: Location,
    parent: Option<String>,
    tags: Vec<Tag>,
}

impl Project{
    fn _flatten(self, parent: String) -> Result<Vec<Project>>{
        let mut projects = Vec::new();
        let name = self.desc.name.clone();

        projects.push(Project::new()
            .desc(self.desc)
            .last_worked(self.last_worked)
            .location(self.location.clone())
        );
        return Ok(projects);
    }

    pub fn flatten(self) -> Result<Vec<Project>>{
        return self._flatten(String::new());
    }
}
