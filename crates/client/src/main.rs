#![allow(unused_import_braces)]

use std::{path::PathBuf, fs::File, io::{BufReader, BufRead, Read, BufWriter, Write}};

use cli::cli;
use project_manager_api::{error::ProjectResult, ProjectsHandler, ProjectLoader, config::manager::Location};

mod cli;

#[derive(Debug)]
pub struct SystemLoader{
    manager_path: PathBuf,
}

impl SystemLoader {
    fn new() -> Self{
        Self{manager_path: PathBuf::new()}
    }
}

impl ProjectLoader for SystemLoader{
    fn get_manager(&self) -> ProjectResult<String> {
        let mut reader = BufReader::new(File::open(&self.manager_path)?);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        
        Ok(buf)
    }
    fn get_project(&self, location: &Location) -> ProjectResult<String> {
        Ok(String::new())
    }

    fn write_manager(&mut self, data: String) -> ProjectResult<()> {
        let mut writer = BufWriter::new(File::create(&self.manager_path)?);
        writer.write_all(data.as_bytes())?;

        Ok(())
    }

    fn write_project(&mut self, data: String, location: &Location) -> ProjectResult<()> {
        if let Location::Path { path } = location{
            println!("writing to...: {}", path.display());
            let mut writer = BufWriter::new(File::create(&path)?);
            writer.write_all(data.as_bytes())?;
        }

        Ok(()) 
    }

    fn ensure_existance(&mut self) -> ProjectResult<()>{
        Ok(())
    }
}

pub type SystemHandler = ProjectsHandler<SystemLoader>;

fn main() -> ProjectResult<()>{
    let mut loader = SystemLoader::new();
    let mut dir = dirs::data_dir().unwrap();
    dir.push("project_manager");
    dir.push("projects");
    dir.set_extension("toml");
    loader.manager_path = dir;
    let handler = SystemHandler::init(loader)?;
    cli(handler)?;
    Ok(())
}
