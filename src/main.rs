mod config;
mod error;
mod utils;
mod baked;

use error::*;
#[allow(unused_imports)]
use config::{manager::Manager, project};

use std::env::args;

fn main() -> ProjectResult<()>{
    let project = project::Project::load_project(".")?;
    for f in project.done{
        println!("{}", f.name);
    }
    println!("TODO!!!:");
    for f in project.todo{
        println!("{}", f.name);
    }
    Ok(())
}
