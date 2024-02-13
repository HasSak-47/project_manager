#![allow(unused_import_braces)]

use std::{path::PathBuf, fs::File, io::{BufReader, Read, BufWriter, Write}};
use cli::cli;
use project_manager_api::{ProjectsHandler, ProjectLoader, config::manager::Location};
use anyhow::{Result, anyhow};

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
    fn get_manager(&self) -> Result<String> {
        let mut reader = BufReader::new(File::open(&self.manager_path)?);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        
        Ok(buf)
    }

    fn get_project(&self, location: &Location) -> Result<String> {
        if let Location::Path { path } = location{
            let mut file = File::open(path)?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            return Ok(buf);
        }
        Err(anyhow!("Project at {location} was not found!"))
    }

    fn write_manager(&mut self, data: String) -> Result<()> {
        let mut writer = BufWriter::new(File::create(&self.manager_path)?);
        writer.write_all(data.as_bytes())?;

        Ok(())
    }

    fn write_project(&mut self, data: String, location: &Location) -> Result<()> {
        if let Location::Path { path } = location{
            println!("writing to...: {}", path.display());
            let mut writer = BufWriter::new(File::create(&path)?);
            writer.write_all(data.as_bytes())?;
        }

        Ok(()) 
    }

    fn ensure_existance(&mut self) -> Result<()>{
        Ok(())
    }
}

pub type SystemHandler = ProjectsHandler<SystemLoader>;

fn main() -> Result<()>{
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
