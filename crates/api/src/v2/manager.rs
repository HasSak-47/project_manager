use std::collections::HashMap;

use super::project::*;

use anyhow::Result;

#[derive(Debug, Default)]
pub struct Manager{
    pub projects: HashMap<String, Project>,
}

pub trait ManagerWriter{
    fn write_manager(&mut self, man: &Manager) -> Result<()>;
}

pub trait ManagerReader{
    fn read_manager(&self) -> Result<Manager>;
}

#[derive(Default)]
pub struct ManagerHandler{
    writer: Option<Box<dyn ManagerWriter>>,
    reader: Option<Box<dyn ManagerReader>>,
}

impl std::fmt::Debug for ManagerHandler{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "ManagerHandler{{ writer: {}, reader: {}, }}", self.writer.is_some(), self.reader.is_some())
    }
}

impl ManagerHandler{
    pub fn new() -> Self{ ManagerHandler{ writer: None, reader: None, } }
    pub fn set_writer(&mut self, writer: Box<dyn ManagerWriter>){ self.writer = Some(writer); }
    pub fn set_reader(&mut self, reader: Box<dyn ManagerReader>){ self.reader = Some(reader); }

    pub fn write_manager(&mut self, manager: &Manager) -> Result<()> {
        match &mut self.writer{
            Some(s) => s.write_manager(&manager),
            None => return Err(anyhow::anyhow!("there is no manager writer")),
        }
    }

    pub fn read_manager(&self) -> Result<Manager> {
        match &self.reader {
            Some(s) => s.read_manager(),
            None => return Err(anyhow::anyhow!("there is no manager reader")),
        }
    }
}

