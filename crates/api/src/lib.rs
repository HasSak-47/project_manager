pub mod project;
pub mod tags;
pub mod task;
pub mod trees;
pub mod desc;

use std::{marker::PhantomData, path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use ly::{log::prelude::*, proc::builder};

use project::{Project, ProjectTable};
use tags::{Tag, TagTable};
use task::{Task, TaskTable};

pub type Timestamp = SystemTime;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Git(String),
    #[default]
    Other,
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

pub trait Idable{
    fn set_id(&mut self);
    fn get_id(&mut self) -> usize;
}

#[builder(name = DatabaseBuilder)]
pub struct Database{
    #[builder(skip)]
    projects: Vec<ProjectTable>,
    #[builder(skip)]
    tasks   : Vec<TaskTable>,
    #[builder(skip)]
    tags    : Vec<TagTable>,

    #[builder(ty = Box<dyn DatabaseReader>, skip_setter, init = Box::new(UnimplementedReader))]
    reader: Box<dyn DatabaseReader>,
    #[builder(ty = Box<dyn DatabaseWriter>, skip_setter, init = Box::new(UnimplementedWriter))]
    writer: Box<dyn DatabaseWriter>,
}

struct UnimplementedReader;
struct UnimplementedWriter;

impl DatabaseReader for UnimplementedReader {
    fn read_all_projects(&self) -> Result<Vec<ProjectTable>> { Err(DatabaseError::NotImplemented) }
    fn read_all_tasks(&self) -> Result<Vec<TaskTable>> { Err(DatabaseError::NotImplemented) }
    fn read_all_tags(&self) -> Result<Vec<TagTable>> { Err(DatabaseError::NotImplemented) }
}

impl DatabaseWriter for UnimplementedWriter {
    fn write_all_projects(&mut self, v: &mut Vec<ProjectTable>) -> Result<()> {Err(DatabaseError::NotImplemented) }
    fn write_all_tasks(&mut self, v: &mut Vec<TaskTable>) -> Result<()> {Err(DatabaseError::NotImplemented) }
    fn write_all_tags(&mut self, v: &mut Vec<TagTable>) -> Result<()> {Err(DatabaseError::NotImplemented) }
}

impl DatabaseBuilder{
    pub fn build(self) -> Database{
        Database{
            projects: Vec::new(),
            tags: Vec::new(),
            tasks: Vec::new(),
            reader: self.reader,
            writer: self.writer,
        }

    }

    pub fn set_reader<R: DatabaseReader>(mut self, r: R) -> Self{
        self.reader = Box::new(r);
        return self;
    }
    pub fn set_writer<W: DatabaseWriter>(mut self, w: W) -> Self{
        self.writer = Box::new(w);
        return self;
    }
}

impl Database{
    pub fn new_project(&mut self, p: Project) -> Result<()>{ Err(DatabaseError::NotImplemented) }
    pub fn new_task(&mut self, t: Task) -> Result<()>{ Err(DatabaseError::NotImplemented) }
    pub fn new_tag(&mut self, t: Tag) -> Result<()>{ Err(DatabaseError::NotImplemented) }

    pub fn search_project<P>(&self, p: P) -> Result<Manager<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
            self.projects
                .iter()
                .find(p)
                .ok_or(DatabaseError::NotFound)
                .and_then(|p| Ok(Manager::new(p.id, self)))
    }

    pub fn search_project_mut<P>(&mut self, p: P) -> Result<ManagerMut<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
            Ok(ManagerMut::new(self.projects
                .iter()
                .find(p)
                .ok_or(DatabaseError::NotFound)
                .and_then(|p| Ok(p.id))?, self))
    }

    pub fn build_project(&self) -> Result<Project>{ Err(DatabaseError::NotImplemented) }
    pub fn add_full_project(&mut self, _: Project) -> Result<()>{ Err(DatabaseError::NotImplemented) }

    pub fn load_data(&mut self) -> Result<()>{
        self.projects = self.reader.read_all_projects()?;
        self.tasks = self.reader.read_all_tasks()?;
        self.tags = self.reader.read_all_tags()?;
        Ok(())
    }

    pub fn write_data(&mut self) -> Result<()>{
        self.writer.write_all_projects(&mut self.projects)?;
        self.writer.write_all_tasks(&mut self.tasks)?;
        self.writer.write_all_tags(&mut self.tags)?;
        Ok(())
    }

    pub fn get_all_projects(&self) -> Vec<Manager<Project>>{
        self.projects
            .iter()
            .map(|p| Manager::new(p.id, self))
            .collect()
    }
}

pub trait DatabaseReader where Self: 'static {
    fn read_all_projects(&self) -> Result<Vec<ProjectTable>>;
    fn read_all_tasks(&self) -> Result<Vec<TaskTable>>;
    fn read_all_tags(&self) -> Result<Vec<TagTable>>;
}

pub trait DatabaseWriter where Self: 'static {
    fn write_all_projects(&mut self, v: &mut Vec<ProjectTable>) -> Result<()>;
    fn write_all_tasks(&mut self, v: &mut Vec<TaskTable>) -> Result<()>;
    fn write_all_tags(&mut self, v: &mut Vec<TagTable>) -> Result<()>;
}

pub struct Manager<'a, T>{
    pool: &'a Database,
    id: usize,
    t: PhantomData<T>,
}

impl<'a, T> Manager<'a, T> {
    fn new(id: usize, pool: &'a Database) -> Self{
        Self {id, pool, t: PhantomData}
    }
}

pub struct ManagerMut<'a, T>{
    pool: &'a mut Database,
    id: usize,
    t: PhantomData<T>,
}

impl<'a, T> ManagerMut<'a, T> {
    fn new(id: usize, pool: &'a mut Database) -> Self{
        Self {id, pool, t: PhantomData}
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError{
    #[error("Not found ")]
    NotFound,

    #[error("unknown")]
    Unknown,

    #[error("undefined error")]
    Undefined,

    #[error("not implemented")]
    NotImplemented,
}
