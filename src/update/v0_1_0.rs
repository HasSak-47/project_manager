use serde::{Deserialize, Serialize};
use toml::{Table, value::Array};

use crate::config::project::{Feature, Project};

mod prev_data{
use serde::{Deserialize, Serialize};
use toml::{Table, value::Array};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub sub_feature: Vec<Feature>,
}

#[derive(Debug, Default, Clone)]
pub struct Project{
    pub info : ProjectInfo,
    pub subproj: Vec<Project>,
    pub todo: Vec<Feature>,
    pub done: Vec<Feature>,
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectToml{
    project : ProjectInfo,
    subproj : Option<Array>,
    todo    : Option<Array>, 
    done    : Option<Array>, 
}
}

impl From<prev_data::Project> for Project{
    fn from(value: prev_data::Project) -> Self {
        let mut p = Project::default();


        p
    }
}
