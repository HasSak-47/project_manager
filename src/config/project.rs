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
    info : ProjectInfo,
    todo: Vec<Feature>,
    done: Vec<Feature>,
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct ProjectInfo{
    name: String,
    version: String,
    edition: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectToml{
    project : ProjectInfo,
    todo    : Option<Array>, 
    done    : Option<Array>, 
}

use crate::error::ProjectResult;

fn gen_feature(t: &Table) -> ProjectResult<Feature>{
    let mut f = Feature::default();
    use crate::error::to_res;
    
    f.name = to_res(t["name"].as_str())?.to_string();
    f.description = t["name"].as_str().map(str::to_string);
    f.difficulty = to_res(t["difficulty"].as_float())? as f32;
    f.priority = to_res(t["priority"].as_float())? as f32;

    Ok(f)
}

fn get_feature(t: &Table) -> ProjectResult<Feature>{
    let mut f = gen_feature(t);
    let subfeature = t.get("subfeature");
    if let None = subfeature{
        return f;
    }

    let arr = subfeature.unwrap().as_array();
    match arr{
        Some(subfeats) => {
            for feat in subfeats {
                f?.sub_feature.push(get_feature(feat.as_table().unwrap())?)
            }
        },
        None => return f,
    }

    f
}

pub fn load_project<S: std::fmt::Display>(path : S) -> ProjectResult<Project>{
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
