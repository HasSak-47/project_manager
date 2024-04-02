#![allow(unused_import_braces)]

use std::{path::PathBuf, fs::File, io::{BufReader, Read, BufWriter, Write}};
use cli::cli;
use project_manager_api::{ProjectsHandler, ProjectLoader, config::manager::Location};
use anyhow::{Result, anyhow};

mod cli;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct SystemLoader{
    manager_path: PathBuf,
}

impl SystemLoader {
    pub fn new() -> Self{ Self{manager_path: PathBuf::new()} }
    pub fn set_path(&mut self, path: PathBuf)  {self.manager_path = path}

    pub fn get_path(path: &PathBuf) -> PathBuf {
        let mut p = path.clone();
        p.push("status");
        p.set_extension("toml");
        p
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
            let path = Self::get_path(path);
            let mut file = File::open(path)?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            return Ok(buf);
        }
        else{}
        Err(anyhow!("Project at {location} was not found!"))
    }

    fn write_manager(&mut self, data: String) -> Result<()> {
        let mut writer = BufWriter::new(File::create(&self.manager_path)?);
        writer.write_all(data.as_bytes())?;

        Ok(())
    }

    fn write_project(&mut self, data: String, location: &Location) -> Result<()> {
        if let Location::Path { path } = location{
            let path = Self::get_path(path);
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
    cli()?;
    Ok(())
}
