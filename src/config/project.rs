use serde::{Deserialize, Serialize};
use toml::{Table, value::Array};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub done: Vec<Feature>,
    pub todo: Vec<Feature>,
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

use crate::error::ProjectResult;

fn gen_feature(t: &Table) -> ProjectResult<Feature>{
    let mut f = Feature::default();
    use crate::error::to_res;
    
    f.name = to_res(t["name"].as_str())?.to_string();
    f.description = t.get("description").map(|x| x.as_str().unwrap_or("").to_string());
    f.difficulty = to_res(t["difficulty"].as_float())? as f32;
    f.priority = to_res(t["priority"].as_float())? as f32;

    Ok(f)
}

fn get_subfeatures(value: &toml::Value) -> ProjectResult<Vec<Feature>>{
    let mut v = Vec::new();
    match value.as_array(){
        Some(feats) => {
            for feat in feats{
                v.push(get_feature(feat.as_table().unwrap())?);
            }
        },
        None => {}
    };

    Ok(v)
}

fn get_feature(t: &Table) -> ProjectResult<Feature>{
    let mut f = gen_feature(t)?;
    let done = t.get("done");
    let todo = t.get("todo");
    if let Some(done_feats_val) = done {
        f.done.append(&mut get_subfeatures(done_feats_val)?);
    }

    if let Some(todo_feats_val) = todo {
        f.todo.append(&mut get_subfeatures(todo_feats_val)?);
    }

    Ok(f)
}

fn load_project<S: AsRef<str>>(path : S) -> ProjectResult<Project>{
    let path = path.as_ref();
    let path = format!{"{path}/status.toml"};
    let data = crate::utils::read_file(path)?;

    let project_toml : ProjectToml = toml::from_str(std::str::from_utf8(data.as_bytes())?)?;
    let mut project = Project::default();

    project.info = project_toml.project;
    for f in project_toml.todo.unwrap_or(Vec::new()){
        project.todo.push(get_feature(&f.as_table().unwrap())?);
    }
    for f in project_toml.done.unwrap_or(Vec::new()){
        project.done.push(get_feature(&f.as_table().unwrap())?);
    }

    Ok(project)
}

impl Project {
    #[allow(unused)]
    pub const fn new() -> Self{
        Project{
            info: ProjectInfo {
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
