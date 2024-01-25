use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::error::*;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FeatureTOML{
    pub name       : String,
    pub priority   : f64,
    pub difficulty : f64,
    pub description: Option<String>,
    pub done: Option<Vec<FeatureTOML>>,
    pub todo: Option<Vec<FeatureTOML>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    pub name       : String,
    pub priority   : f64,
    pub difficulty : f64,
    pub description: Option<String>,
    pub done: Vec<Feature>,
    pub todo: Vec<Feature>,
}

fn opt_vec_to_vec(v: Option<Vec<FeatureTOML>>) -> Vec<Feature>{
    let v = v.unwrap_or(Vec::new());
    v.into_iter()
        .map(|x| x.into())
        .collect()
}

impl From<FeatureTOML> for Feature{
    fn from(value: FeatureTOML) -> Self {
        Feature {
            name: value.name,
            priority: value.priority,
            difficulty: value.difficulty,
            description: value.description,
            todo: opt_vec_to_vec(value.todo),
            done: opt_vec_to_vec(value.done),
        }
    }
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectToml{
    pub project: ProjectInfo,
    pub todo   : Option<Vec<FeatureTOML>>,
    pub done   : Option<Vec<FeatureTOML>>,
}

#[derive(Serialize, Debug)]
pub struct Project{
    pub project: ProjectInfo,
    pub todo   : Vec<Feature>,
    pub done   : Vec<Feature>,
}

impl From<ProjectToml> for Project{
    fn from(value: ProjectToml) -> Self {
        Self {
            project: value.project,
            todo: opt_vec_to_vec(value.todo),
            done: opt_vec_to_vec(value.done),
        }
    }
}

#[allow(dead_code)]
impl Project {
    #[allow(unused)]
    pub const fn new() -> Self{
        Project{
            project: ProjectInfo {
                name: String::new(),
                version: String::new(),
                edition: String::new(),
            },
            
            todo: Vec::new(),
            done: Vec::new(),
        }
    }

    pub fn load_project<P : AsRef<Path>>(path: P) -> ProjectResult<Self>{
        let mut path = path.as_ref().to_path_buf();
        path.push("status");
        path.set_extension("toml");
        let data = crate::utils::read_file(path)?;

        let project_toml : ProjectToml = toml::from_str(std::str::from_utf8(data.as_bytes())?)?;

        Ok(project_toml.into())
    }

    fn _get_todo(v : &Vec<Feature>) -> f64{
        let mut t = 0.;
        for f in v{
            t += f.difficulty;
            t += Self::_get_todo(&f.todo);
        }
        t
    }

    fn _get_done(v : &Vec<Feature>) -> f64{
        let mut t = 0.;
        for f in v{
            t += f.difficulty;
            t += Self::_get_done(&f.done);
        }
        t
    }
    

    pub fn get_todo(&self) -> f64{
        Self::_get_todo(&self.todo)
    }

    pub fn get_done(&self) -> f64{
        Self::_get_done(&self.done)
    }

    pub fn get_completion(&self) -> f64{
        let t = self.get_todo();
        let d = self.get_done();

        let total = t + d;
        if total == 0.{ 0. }
        else{ d  / total }
    }
}
