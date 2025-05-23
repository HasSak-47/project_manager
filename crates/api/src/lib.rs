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
use tags::{Tag, TagOtherTable, TagTable};
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

type TagProjectTable = TagOtherTable<ProjectTable>;
type TagTaskTable = TagOtherTable<TaskTable>;

#[builder(name = DatabaseBuilder)]
pub struct Database{
    #[builder(skip)]
    projects: Vec<ProjectTable>,
    #[builder(skip)]
    tasks   : Vec<TaskTable>,
    #[builder(skip)]
    tags    : Vec<TagTable>,

    #[builder(skip)]
    tag_pro : Vec<TagOtherTable<ProjectTable>>,

    #[builder(skip)]
    tag_task: Vec<TagOtherTable<TaskTable>>,

    #[builder(ty = Box<dyn DatabaseReader>, skip_setter, init = Box::new(UnimplementedReader))]
    reader: Box<dyn DatabaseReader>,
    #[builder(ty = Box<dyn DatabaseWriter>, skip_setter, init = Box::new(UnimplementedWriter))]
    writer: Box<dyn DatabaseWriter>,
}

impl std::fmt::Debug for Database{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "projects: {:?} tasks{:?} tags {:?}",    
            self.projects,
            self.tasks   ,
            self.tags    ,
        )
    }
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
            tag_pro: Vec::new(),
            tag_task: Vec::new(),
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
    pub fn search_project_id<P>(&self, p: P) -> Result<usize>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
            self.projects
                .iter()
                .find(p)
                .ok_or(DatabaseError::NotFound)
                .and_then(|p| Ok(p.id))
    }

    pub fn search_task_id<P>(&self, p: P) -> Result<usize>
    where
        P: FnMut(&&TaskTable) -> bool,
    {
            self.tasks
                .iter()
                .find(p)
                .ok_or(DatabaseError::NotFound)
                .and_then(|p| Ok(p.id))
    }

    pub fn search_project<P>(&self, p: P) -> Result<Manager<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool {
        self.search_project_id(p).and_then(|p| Ok(Manager::new(p, self)))
    }

    pub fn search_project_mut<P>(&mut self, p: P) -> Result<ManagerMut<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
        self.search_project_id(p).and_then(|p| Ok(ManagerMut::new(p, self)))
    }

    pub fn build_project(&self) -> Result<Project>{ Err(DatabaseError::NotImplemented) }

    pub fn search_tag_id(&self, name: &String) -> Result<usize>{
        self.tags.iter()
            .find(|p| p.tag == *name)
            .ok_or(DatabaseError::NotFoundOther(format!("Could not found tag {name}")))
            .and_then(|p| Ok(p.id))
    }

    fn create_project(&self, p: Project) -> Result<(ProjectTable, Vec<TagProjectTable>)>{
        let mut entry = ProjectTable::default();
        entry.id = self.projects
            .last()
            .and_then(|l| Some(l.id))
            .unwrap_or(self.projects.len());
        let o_id = entry.id;

        let mut tag_pro = Vec::new();
        for tag in &p.desc.tags{
            let tag_id = self.search_tag_id(tag)?;
            tag_pro.push(TagProjectTable::new(o_id, tag_id));
        }

        entry.desc = p.desc.into();
        
        return Ok((entry, tag_pro));
    }

    fn get_child_project(&self, entries: &mut Vec<ProjectTable>, p: Project, parent: usize) -> Result<()>{
        if !p.parent.is_empty(){
            return Err(DatabaseError::Malformed);
        }

        let childs = p.childs.clone();
        let mut entry = self.create_project(p)?;
        entry.0.parent = Some(parent);
        let id = entries.len();
        entries.push(entry.0);

        for child in childs{
            self.get_child_project(entries, child, id)?;
        }

        Ok(())
    }

    pub fn add_full_project(&mut self, p: Project) -> Result<()>{
        if !p.parent.is_empty(){
            return Err(DatabaseError::NotImplemented);
        }

        let childs = p.childs.clone();
        let entry = self.create_project(p)?;
        let id = 0;
        let mut entries = vec![entry.0];
        for child in childs{
            self.get_child_project(&mut entries, child, id)?;
        }
        let offset = self.projects.last().and_then(|f| Some(f.id)).unwrap_or(0);
        for mut entry in entries{
            entry.id += offset;
            self.projects.push(entry);
        }


        Ok(())
    }

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
    #[error("Not found")]
    NotFound,

    #[error("Not found {0}")]
    NotFoundOther(String),

    #[error("unknown")]
    Unknown,

    #[error("undefined error")]
    Undefined,

    #[error("Malformed non expected field")]
    Malformed,

    #[error("not implemented")]
    NotImplemented,
}
