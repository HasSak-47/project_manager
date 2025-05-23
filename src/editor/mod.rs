use std::io::{self, prelude::*};
use std::fs;
use std::env;
use dirs;
use super::config::Config;

use super::project::{self, *};

fn ask<Type>(reader: &mut io::BufReader<std::io::Stdin>, name: &str) -> Option<Type>
where
    Type: std::fmt::Display + std::str::FromStr + std::fmt::Debug, 
    <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    println!("write {}: ", name);
    reader.read_line(&mut buffer).unwrap();
    buffer.pop();

    Some(Type::from_str(buffer.as_str()).unwrap())

}

pub fn make_project(mut c: Config){
    let mut p = Project::new();
    let mut reader = io::BufReader::new(io::stdin());
    p.project.name    = ask(&mut reader, "name").unwrap();
    p.project.folder  = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    p.project.version = ask(&mut reader, "version").unwrap();

    let name = p.project.name.clone().to_lowercase().replace(" ", "_");

    let pm_path = format!("{}/project_manager/", dirs::config_dir().unwrap().to_str().unwrap());
    let config_path  = format!("{}config", pm_path);
    let project_path = format!("{}{}.toml", pm_path, name);


    let mut projects = c.projects.unwrap();
    projects.insert(name, toml::Value::String(project_path.clone()));
    c.projects = Some(projects);


    let mut project_file = fs::File::create(project_path).unwrap();
    project_file.write(toml::to_string(&p).unwrap().as_bytes()).unwrap();

    let mut config_file = fs::File::create(config_path).unwrap();
    config_file.write(toml::to_string(&c).unwrap().as_bytes()).unwrap();
}

/*
pub fn ask_for_project() -> (Project, String){
    let mut p : Project = Project::new();
    let mut reader = io::BufReader::new(io::stdin());

    p.project.folder = env::current_dir().unwrap().to_str().unwrap().to_string();

    // god this is disguisting
    ask(&mut reader, "folder", &mut p.project.folder);
    ask(&mut reader, "name", &mut p.project.name);
    ask(&mut reader, "version", &mut p.project.version);

    ask(&mut reader, "countdown", &mut p.local.countdown);
    ask(&mut reader, "add all", &mut p.local.add_all);
    ask(&mut reader, "force commit", &mut p.local.force_commit);

    ask(&mut reader, "push", &mut p.remote.push);
    ask(&mut reader, "remote", &mut p.remote.remote);
    ask(&mut reader, "countdown", &mut p.remote.countdown);
    ask(&mut reader, "local_commit", &mut p.remote.local_commit);
    ask(&mut reader, "force_push", &mut p.remote.force_push);

    let mut file_path = format!("{}/project_manager/{}.toml", dirs::config_dir().unwrap().to_str().unwrap(), p.project.name);

    ask(&mut reader, "file path", &mut file_path);
    (p , file_path)
}*/

pub fn create_project(config: Config, project : Project, path: String){
}
