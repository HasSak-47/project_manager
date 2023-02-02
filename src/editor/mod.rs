use std::fmt::Debug;
use std::io;
use std::env;
use serde;

use super::project::{self, *};

pub fn ask<Type>(reader: &io::Stdin, name: &str, option: &mut Type) 
where
    Type: std::fmt::Display + std::str::FromStr + std::fmt::Debug, 
    <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    println!("is this option ({}: {}) okay? Y/n", name, option);
    reader.read_line(&mut buffer).unwrap();

    if buffer.chars().nth(0).is_none() || buffer.chars().nth(0).unwrap() != 'y'{
        print!("write your answer: ");
        reader.read_line(&mut buffer).unwrap();
      
        *option = Type::from_str(buffer.as_str()).unwrap();
    }

}

pub fn add_project(){
    let mut p : Project = Project::new();
    let reader = io::stdin();

    p.project.folder = env::current_dir().unwrap().to_str().unwrap().to_string();

    ask(&reader, "folder", &mut p.project.folder);
    ask(&reader, "name", &mut p.project.name);
    ask(&reader, "version", &mut p.project.version);

    ask(&reader, "countdown", &mut p.local.countdown);
    ask(&reader, "add all", &mut p.local.add_all);
    ask(&reader, "force commit", &mut p.local.force_commit);

    ask(&reader, "push", &mut p.remote.push);
    ask(&reader, "remote", &mut p.remote.remote);
    ask(&reader, "countdown", &mut p.remote.countdown);
    ask(&reader, "local_commit", &mut p.remote.local_commit);
    ask(&reader, "force_push", &mut p.remote.force_push);


    println!("{:?}", p);
}
