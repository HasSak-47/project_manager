use super::Location;

#[derive(Debug, Default)]
pub struct Manager{
}

pub trait ManagerWriter{
    fn write_manager(&mut self, str: Manager) -> Result<()>;
}

pub trait MangerReader{
    fn read_manager(&self, location: Location) -> Result<Manager>;
}

pub struct ManagerHandler{
    writer: Option<Box<dyn ManagerWriter>>,
    reader: Option<Box<dyn ManagerReader>>,
    manager: Manager,
}

impl ManagerHandler{
    pub fn new(){
        ManagerHandler{
            writer: None,
            reader: None,
            manager: Manager::default(),
        }
    }

    pub fn set_writer(&mut self, writer: Box<dyn ManagerWriter>){ self.writer = Some(writer); }
    pub fn set_reader(&mut self, reader: Box<dyn ManagerReader>){ self.reader = Some(reader); }


}

