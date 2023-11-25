use serde::{Deserialize, Serialize};
use toml::{Table, value::Array};
use crate::error::*;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub done: Vec<Feature>,
    pub todo: Vec<Feature>,
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
    pub subproj: Option<Vec<Project>>,
    pub todo   : Option<Vec<Feature>>,
    pub done   : Option<Vec<Feature>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Project{
    pub project: ProjectInfo,
    pub subproj: Vec<Project>,
    pub todo   : Vec<Feature>,
    pub done   : Vec<Feature>,
}

impl From<ProjectToml> for Project{
    fn from(value: ProjectToml) -> Self {
        Self {
            project: value.project,
            subproj: value.subproj.unwrap_or(Vec::new()),
            todo: value.todo.unwrap_or(Vec::new()),
            done: value.done.unwrap_or(Vec::new()),
        }
    }
}

fn load_project<S: AsRef<str>>(path : S) -> ProjectResult<Project>{
    let path = path.as_ref();
    let path = format!{"{path}/status.toml"};
    let data = crate::utils::read_file(path)?;

    let project_toml : ProjectToml = toml::from_str(std::str::from_utf8(data.as_bytes())?)?;

    Ok(project_toml.into())
}

impl Project {
    #[allow(unused)]
    pub const fn new() -> Self{
        Project{
            project: ProjectInfo {
                name: String::new(),
                version: String::new(),
                edition: String::new(),
            },
            subproj: Vec::new(),
            
            todo: Vec::new(),
            done: Vec::new(),
        }
    }

    pub fn load_project<S : AsRef<str>>(path: S) -> ProjectResult<Self>{
        load_project(path)
    }

    fn _get_todo(v : &Vec<Feature>) -> f32{
        let mut t = 0.;
        for f in v{
            t += f.difficulty;
            t += Self::_get_todo(&f.todo);
        }
        t
    }

    fn _get_done(v : &Vec<Feature>) -> f32{
        let mut t = 0.;
        for f in v{
            t += f.difficulty;
            t += Self::_get_done(&f.done);
        }
        t
    }

    pub fn get_todo(&self) -> f32{
        Self::_get_todo(&self.todo)
    }

    pub fn get_done(&self) -> f32{
        Self::_get_done(&self.done)
    }

    pub fn get_completion(&self) -> f32{
        let t = self.get_todo();
        let d = self.get_done();

        let total = t + d;
        if total == 0.{ 0. }
        else{ d  / total }
    }
}
