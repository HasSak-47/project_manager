use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
mod manager;
mod project;

#[derive(Debug, Clone)]
pub enum Location{
    Path(PathBuf),
    Url(String),
}

impl Default for Location{ fn default() -> Self{ Location::Path(PathBuf::new()) } }

impl Location{
    pub fn path<P : AsRef<Path>>(path: P) -> Self{ Location::Path(path.as_ref().to_path_buf()) }
    pub fn url<S: AsRef<str>>(url: S) -> Self{ Location::Url(url.as_ref().to_string()) }

    pub fn get_path(&self) -> Result<&Path>{
        match self{
            Location::Path(path) => Ok(path.as_path()),
            _ => Err(anyhow!("Location is not a path!!")),
        }
    }

    pub fn get_url(&self) -> Result<&str>{
        match self{
            Location::Url(url) => Ok(url.as_str()),
            _ => Err(anyhow!("Location is not a url!!")),
        }
    }
}
use project::*;
use manager::*;

#[derive(Debug, Default)]
pub struct Handler{
    pub project_handler: ProjectHandler,
    pub manager_handler: ManagerHandler,

    pub manager: Manager,
    _inited: bool,
}

impl Handler{
    pub fn new() -> Self{ Self::default() }

    pub fn set_project_writer<Writer>(&mut self, writer: Writer)
    where
        Writer: project::Writer + 'static
    { self.project_handler.set_writer(Box::new(writer)); }

    pub fn set_project_reader<Reader: project::Reader>(&mut self, reader: Reader)
    where
        Reader: project::Writer + 'static
    { self.project_handler.set_reader(Box::new(reader)); }


    pub fn set_manager_writer<Writer: manager::ManagerWriter>(&mut self, writer: Writer)
    where
        Writer: project::Writer + 'static
    { self.manager_handler.set_writer(Box::new(writer)); }

    pub fn set_manager_reader<Reader: manager::ManagerReader>(&mut self, reader: Reader)
    where
        Reader: project::Writer + 'static
    { self.manager_handler.set_reader(Box::new(reader)); }

    pub fn init(&mut self) -> Result<()>{
        self.manager = self.manager_handler.read_manager()?;
        self._inited = true;
        Ok(())
    }

    pub fn load_projects(&mut self) -> Result<()>{
        for (_, p) in &mut self.manager.projects{
            self.project_handler.read(p)?;
        } Ok(())
    }

    pub fn commit_projects(&mut self) -> Result<()>{
        for (_, p) in &mut self.manager.projects{
            self.project_handler.write(p)?;
        } Ok(())
        
    }

    pub fn get_project<S: AsRef<String>>(&mut self, name: S) -> Result<&mut Project> {
        let project = self.manager.projects
            .get_mut(name.as_ref())
            .ok_or(anyhow!("Project Not found!"))?;
        self.project_handler.read(project)?;
        Ok(project)
    }

    pub fn commit_project<S: AsRef<String>>(&mut self, name: S) -> Result<&mut Project> {
        let project = self.manager.projects
            .get_mut(name.as_ref())
            .ok_or(anyhow!("Project Not found!"))?;
        self.project_handler.write(project)?;
        Ok(project)
    }

}
