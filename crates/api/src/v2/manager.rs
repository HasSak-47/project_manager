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
