use super::Location;
use anyhow::Result;

#[allow(dead_code)]
pub struct Feature{
    name: String,
    description: String,
    status: String,
    priority: u8,
    difficulty: u8,

    todo: Vec<Feature>,
    done: Vec<Feature>,
}

// the status of the project
#[allow(dead_code)]
pub struct ProjectStatus{
    pub name: String,
    pub description: String,
    pub todo: Vec<Feature>,
    pub done: Vec<Feature>,
}

// info on the project
#[allow(dead_code)]
pub struct ProjectInfo{
    pub name: String,
    pub location: Location,
    pub status: Option<Location>, // if None, status is in the project folder
    pub last_update: Option<usize>, // timestamp
}

// the project inside the manager
pub struct Project{
    pub info: ProjectInfo,
    pub status: Option<Box<ProjectStatus>>, // the project may not be loaded
}

pub trait Writer{
    fn write_status(&mut self, location: &Location, man: &ProjectStatus) -> Result<()>;
    fn write_info  (&mut self, location: &Location, man: &ProjectInfo) -> Result<()>;
}

pub trait Reader{
    fn read_status(&self, location: &Location) -> Result<ProjectStatus>;
    fn read_info  (&self, location: &Location) -> Result<ProjectInfo>;
}

pub struct ProjectHandler{
    writer: Option<Box<dyn Writer>>,
    reader: Option<Box<dyn Reader>>,
}

impl ProjectHandler{
    pub fn new() -> Self{ ProjectHandler{ writer: None, reader: None, } }
    pub fn set_writer(&mut self, writer: Box<dyn Writer>){ self.writer = Some(writer); }
    pub fn set_reader(&mut self, reader: Box<dyn Reader>){ self.reader = Some(reader); }

    pub fn write_status(&mut self, project: &mut Project) -> Result<()> {
        match &mut self.writer{
            Some(s) => {
                let status = match &project.status{
                    Some(s) => s,
                    None => return Err(anyhow::anyhow!("there is no project status")),
                };
                s.write_status(&project.info.location, &status)?;
            },
            None => return Err(anyhow::anyhow!("there is no project writer")),
        }
        Ok(())
    }

    pub fn read_status(&self, location: Location) -> Result<ProjectStatus> {
        match &self.reader {
            Some(s) => s.read_status(&location),
            None => return Err(anyhow::anyhow!("there is no project reader")),
        }
    }

    pub fn write_info(&mut self, project: &mut Project) -> Result<()> {
        match &mut self.writer{
            Some(s) => s.write_info(&project.info.location, &project.info),
            None => return Err(anyhow::anyhow!("there is no project writer")),
        }
    }

    pub fn read_info(&self, location: Location) -> Result<ProjectInfo> {
        match &self.reader {
            Some(s) => s.read_info(&location),
            None => return Err(anyhow::anyhow!("there is no project reader")),
        }
    }
}
