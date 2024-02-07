#![allow(unused_import_braces)]

use std::path::PathBuf;

use cli::cli;
use project_manager_api::{error::ProjectResult, ProjectsHandler, ProjectLoader, config::manager::{Location, Manager}};

mod cli;

struct SystemLoader{
    manager_path: PathBuf,
}

impl SystemLoader {
    fn new() -> Self{
        Self{manager_path: PathBuf::new()}
    }
}

impl ProjectLoader for SystemLoader{
    fn get_manager(&self) -> ProjectResult<String> {
        Ok(String::new())
    }
    fn get_project(&self, location: &Location) -> ProjectResult<String> {
        Ok(String::new())
    }

    fn ensure_existance(&mut self) -> ProjectResult<()>{ Ok(())}
}

pub type SystemHandler=  ProjectsHandler<SystemLoader>;

fn main() -> ProjectResult<()>{
    let mut loader = SystemLoader::new();
    loader.manager_path = Manager::get_path()?;
    let handler = SystemHandler::init(loader)?;
    cli(handler)?;
    Ok(())
}
