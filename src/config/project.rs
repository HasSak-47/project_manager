use toml::{Value, map::Map};
use serde::{Deserialize, Serialize};
use std::fs::read;

use super::super::errors::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData{
    pub name: String,
    pub path: String,
    pub todo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile{
    project: ProjectData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectTodo{
    pub done: Option<Map<String, Value>>,
    pub todo: Option<Map<String, Value>>,
}

#[derive(Debug)]
pub struct ParsedTodo{
    pub done: Vec<(String, f64)>,
    pub todo: Vec<(String, f64)>,
}

#[derive(Debug)]
pub struct Project{
    pub data: ProjectData,
    pub todo: ParsedTodo,
}

pub fn read_project(data: ProjectData) -> Result<Project>{
    let file_data = String::from_utf8(read(data.todo.clone())?)?;
    let todo_toml: ProjectTodo = toml::from_str(file_data.as_str())?;


    let parse_map = |map: toml::Table|{
        let mut todos: Vec<(String, f64)> = vec![];
        for todo in map{
            if todo.1.is_float(){
                todos.push((todo.0, todo.1.as_float().unwrap()));
            }
        }
        todos
    };

    Ok(Project{data, todo: ParsedTodo{
        todo: parse_map(todo_toml.todo.unwrap()),
        done: parse_map(todo_toml.done.unwrap()),
    }})
}

pub fn get_projects() -> Result<Vec<Project>> {
    let projects_paths = match super::manager::load_manager().projects{
        Some(k) => k,
        None => return Err(Error::new()),
    };


    let mut projects = Vec::<Project>::new();
    for path in projects_paths{
        if !path.1.is_str(){
            println!("project {} has a non str val({})", path.0, path.1);
            continue;
        }

        let project_toml_str = String::from_utf8(match read(path.1.as_str().unwrap()){
            Ok(k) => k,
            Err(_) => continue,
        })?;
        let project_toml : ProjectFile = toml::from_str(project_toml_str.as_str())?;

        projects.push(read_project(project_toml.project)?);
    }

    Ok(projects)
}
