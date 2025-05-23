use super::project::*;

use super::Location;
use anyhow::Result;

pub struct Manager{
    pub projects: Vec<ProjectInfo>,
}

pub trait ManagerWriter{
    fn write_manager(&mut self, location: Location, man: &Manager) -> Result<()>;
}

pub trait ManagerReader{
    fn read_manager(&self, location: Location) -> Result<Manager>;
}

pub struct ManagerHandler{
    writer: Option<Box<dyn ManagerWriter>>,
    reader: Option<Box<dyn ManagerReader>>,
}

impl ManagerHandler{
    fn new() -> Self{ ManagerHandler{ writer: None, reader: None, } }
    fn set_writer(&mut self, writer: Box<dyn ManagerWriter>){ self.writer = Some(writer); }
    fn set_reader(&mut self, reader: Box<dyn ManagerReader>){ self.reader = Some(reader); }

    fn write_manager(&mut self, location: Location, manager: &Manager) -> Result<()> {
        match &mut self.writer{
            Some(s) => s.write_manager(location, &manager),
            None => return Err(anyhow::anyhow!("there is no manager writer")),
        }
    }

    fn read_manager(&self, location: Location) -> Result<Manager> {
        match &self.reader {
            Some(s) => s.read_manager(location),
            None => return Err(anyhow::anyhow!("there is no manager reader")),
        }
    }
}

