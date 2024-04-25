use std::collections::HashMap;

use super::project::*;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Manager{
    pub projects: HashMap<String, ProjectInfo>,
}

pub trait IO{
    fn write(&mut self, man: &Manager) -> Result<()>;
    fn read(&self) -> Result<Manager>;
}

#[derive(Default)]
pub struct ManagerHandler{
    io: Option<Box<dyn IO>>,
}

impl std::fmt::Debug for ManagerHandler{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "ManagerHandler{{ writer: {} }}", self.io.is_some())
    }
}

impl ManagerHandler{
    pub fn new() -> Self{ ManagerHandler{ io: None } }
    pub fn set_io(&mut self, writer: Box<dyn IO>){ self.io = Some(writer); }

    pub fn write(&mut self, manager: &Manager) -> Result<()> {
        match &mut self.io{
            Some(s) => s.write(&manager),
            None => return Err(anyhow::anyhow!("there is no manager writer")),
        }
    }

    pub fn read(&self) -> Result<Manager> {
        match &self.io{
            Some(s) => s.read(),
            None => return Err(anyhow::anyhow!("there is no manager reader")),
        }
    }
}

