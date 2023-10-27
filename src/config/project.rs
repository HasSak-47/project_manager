use serde::{Deserialize, Serialize};
use toml::{Table, value::Array};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    name       : String,
    priority   : f32,
    difficulty : f32,
    description: Option<String>,
    sub_feature: Vec<Feature>,
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

fn get_feature(t: &Table) -> ProjectResult<Feature>{
    let mut f = gen_feature(t)?;
    let subfeature = t.get("subfeature");
    if let None = subfeature{
        return Ok(f);
    }

    let arr = subfeature.unwrap().as_array();
    match arr{
        Some(subfeats) => {
            for feat in subfeats {
                f.sub_feature.push(get_feature(feat.as_table().unwrap())?)
            }
        },
        None => return Ok(f),
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

    fn extract_weight(feats: &Vec<Feature>) -> f32{
        let mut f = 0f32;

        for feat in feats{
            f += feat.difficulty;
            f += Self::extract_weight(&feat.sub_feature);
        }

        f
    }

    pub fn get_todo(&self) -> f32{
        Self::extract_weight(&self.todo)
    }

    pub fn get_done(&self) -> f32{
        Self::extract_weight(&self.done)
    }

    pub fn get_completion(&self) -> f32{
        let t = self.get_todo();
        let d = self.get_done();

        let total = t + d;
        if total == 0.{ 0. }
        else{ d  / total }

    }
}
