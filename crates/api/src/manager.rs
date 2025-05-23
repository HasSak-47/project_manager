use std::{collections::HashMap, rc::Rc};

use super::project::*;

use anyhow::Result;

#[derive(Debug, Default)]
pub struct Manager{
    pub projects: HashMap<String, Project>,
}

pub trait Writer{
    fn write(&mut self, man: &Manager) -> Result<()>;
}

pub trait Reader{
    fn read(&self) -> Result<Manager>;
}

#[derive(Default)]
pub struct ManagerHandler{
    writer: Option<Rc<dyn Writer>>,
    reader: Option<Rc<dyn Reader>>,
}

impl std::fmt::Debug for ManagerHandler{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "ManagerHandler{{ writer: {}, reader: {}, }}", self.writer.is_some(), self.reader.is_some())
    }
}

impl ManagerHandler{
    pub fn new() -> Self{ ManagerHandler{ writer: None, reader: None, } }
    pub fn set_writer(&mut self, writer: Rc<dyn Writer>){ self.writer = Some(writer); }
    pub fn set_reader(&mut self, reader: Rc<dyn Reader>){ self.reader = Some(reader); }

    pub fn write(&mut self, manager: &Manager) -> Result<()> {
        match &mut self.writer{
            Some(s) => s.write(&manager),
            None => return Err(anyhow::anyhow!("there is no manager writer")),
        }
    }

    pub fn read(&self) -> Result<Manager> {
        match &self.reader {
            Some(s) => s.read(),
            None => return Err(anyhow::anyhow!("there is no manager reader")),
        }
    }
}

