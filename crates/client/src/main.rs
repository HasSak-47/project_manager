#![allow(unused_import_braces)]

use std::{path::PathBuf, fs::File, io::{BufReader, Read, BufWriter, Write}};
use cli::cli;
use anyhow::{Result, anyhow};
use project_manager_api::{manager, project, Handler};

mod cli;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct LinuxManager{
    manager_path: PathBuf,
}

impl manager::Writer for LinuxManager{
    fn write(&mut self, man: &manager::Manager) -> Result<()> {
        Err(anyhow!("not implemented!"))
    }
}

impl manager::Reader for LinuxManager{
    fn read(&self) -> Result<manager::Manager> {
        Err(anyhow!("not implemented!"))
    }
}

#[derive(Default)]
pub struct LinuxProject{}

impl project::Writer for LinuxProject{
    fn write(&mut self, location: &project_manager_api::Location, prj: &project::ProjectStatus) -> Result<()> {
        Err(anyhow!("not implemented!"))
    }
}

impl project::Reader for LinuxProject {
    fn read(&self, location: &project_manager_api::Location) -> Result<project::ProjectStatus> {
        Err(anyhow!("not implemented!"))
    }
}

fn main() -> Result<()>{

    cli()?;
    Ok(())
}
