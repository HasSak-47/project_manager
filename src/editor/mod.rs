use std::io::{self, prelude::*};
use std::fs;
use std::env;
use dirs;

use super::project::{self, *};

fn ask<Type, ReaderT>(reader: &mut io::BufReader<ReaderT>, name: &str, option: &mut Type) 
where
    Type: std::fmt::Display + std::str::FromStr + std::fmt::Debug, 
    <Type as std::str::FromStr>::Err: std::fmt::Debug,
    ReaderT: io::Read,
{
    let mut buffer = String::new();
    println!("is this option ({}: {}) okay? Y/n", name, option);
    reader.read_line(&mut buffer).unwrap();

    if buffer.len() == 0 {return;}

    let nth = buffer.chars().nth(0).unwrap();
    if nth.to_uppercase().nth(0).unwrap() != 'Y'{
        print!("write the new one: ");
        reader.read_line(&mut buffer).unwrap();
      
        *option = Type::from_str(buffer.as_str()).unwrap();
    }

}

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
}

pub fn create_project(project: Project, path: String){
    let mut file = fs::File::open(&path).unwrap();
    file.write(toml::to_string(&project).unwrap().as_bytes()).unwrap();
}
