#![allow(unused_import_braces)]

const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::{fs::File, io::{BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}};
use cli::cli;
use anyhow::{anyhow, Result};
use project_manager_api::{
    Location,
    project,
    manager,
};


mod cli;

#[derive(Debug, Default)]
struct ProjectTOML{ } 

#[derive(Debug, Default)]
struct ManagerTOML{
    path: PathBuf,
} 

impl ProjectTOML{
    fn get_status(path: &Path) -> PathBuf{
        path.to_path_buf().join("status.toml")
    }
}

impl project::IO for ProjectTOML{
    fn read(&self, location: &Location) -> Result<project::ProjectStatus>{
        let path = Self::get_status(location.get_path()?);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }

    fn write(&mut self, location: &Location, prj: &project::ProjectStatus) -> Result<()>{
        let path = Self::get_status(location.get_path()?);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(toml::to_string(prj)?.as_bytes())?;
        Ok(())
    }
}

impl manager::IO for ManagerTOML {
    fn write(&mut self, manger: &manager::Manager) -> Result<()>{
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);
        let tomldata = toml::to_string(manger).unwrap();
        writer.write_all(tomldata.as_bytes())?;
        Ok(())
    }

    fn read(&self) -> Result<manager::Manager> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }
}

fn main() -> Result<()>{
    cli()?;
    Ok(())
}
