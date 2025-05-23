use super::config::*;
use crate::errors::Result;
use toml::{Table, Value};

use std::env;
use std::io;

fn catcher() -> Result<()>{
    let mut manager = manager::load_manager();

    let current_dir = env::current_dir()?.to_str().unwrap().to_string();
    println!("set name for ({current_dir})");

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let mut projects = match &manager.projects{
        Some(n) => n.clone(),
        None    => Table::new(),
    };

    projects.insert(name, Value::String(current_dir));
    manager.projects = Some(projects);

    let path = manager::get_config_path();

    let data : String = toml::to_string(&manager).unwrap();
    std::fs::write(path, data.as_bytes())?;

    Ok(())
}

pub fn add(){
    catcher().unwrap();
}
