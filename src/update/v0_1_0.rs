mod universal{
use serde::{Serialize, Deserialize};
#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}
}

mod prev{
use serde::{Serialize, Deserialize};
use super::universal::ProjectInfo;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FeatureTOML{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub subfeat: Option<Vec<FeatureTOML>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectToml{
    pub project: ProjectInfo,
    pub todo   : Option<Vec<FeatureTOML>>,
    pub done   : Option<Vec<FeatureTOML>>,
}
}

mod next{
use serde::{Serialize, Deserialize};
use super::universal::ProjectInfo;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FeatureTOML{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub done: Option<Vec<FeatureTOML>>,
    pub todo: Option<Vec<FeatureTOML>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectToml{
    pub project: ProjectInfo,
    pub todo   : Option<Vec<FeatureTOML>>,
    pub done   : Option<Vec<FeatureTOML>>,
}
}

fn opt_vec_to_vec(v: Option<Vec<prev::FeatureTOML>>) -> Option<Vec<next::FeatureTOML>>{
    match v {
        Some(v) => Some(
            v.into_iter()
            .map(|x| x.into())
            .collect()),
        None => None,
    }
}

impl From<prev::FeatureTOML> for next::FeatureTOML{
    fn from(value: prev::FeatureTOML) -> Self {
        next::FeatureTOML {
            name: value.name,
            priority: value.priority,
            difficulty: value.difficulty,
            description: value.description,
            todo: opt_vec_to_vec(value.subfeat),
            done: None,
        }
    }
}

impl From<prev::ProjectToml> for next::ProjectToml{
    fn from(value: prev::ProjectToml) -> Self {
        Self{
            project: value.project,
            todo: opt_vec_to_vec(value.todo),
            done: opt_vec_to_vec(value.done),
        }
    }
}
