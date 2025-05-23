pub mod project;
pub mod tags;
pub mod task;
pub mod desc;

use std::{marker::PhantomData, path::PathBuf, time::SystemTime};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use ly::{log::prelude::*, proc::builder};

use project::{Project, ProjectTable};
use tags::{Tag, TagOtherTable, TagTable};
use task::{Task, TaskTable};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
        write!(f, "projects: {:#?} tasks{:#?} tags {:#?}",    
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
    fn write_all_projects(&mut self, _: &mut Vec<ProjectTable>) -> Result<()> {Err(DatabaseError::NotImplemented) }
    fn write_all_tasks(&mut self, _: &mut Vec<TaskTable>) -> Result<()> {Err(DatabaseError::NotImplemented) }
    fn write_all_tags(&mut self, _: &mut Vec<TagTable>) -> Result<()> {Err(DatabaseError::NotImplemented) }
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

    pub fn search_tag_id(&self, name: &String) -> Result<usize>{
        self.tags.iter()
            .find(|p| p.tag == *name)
            .ok_or(DatabaseError::NotFoundOther(format!("Could not found tag {name}")))
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

    fn unravel_tasks(&self, pt: &mut Vec<ProjectTable>, tt: &mut Vec<TaskTable>, gt: &mut Vec<TagTable>, t: Task, pa_id: Option<usize>, pr_id: Option<usize>) -> Result<()>{
        let mut entry = TaskTable::default();
        entry.id = tt.len();
        entry.desc = t.desc.into();
        entry.parent = pa_id;
        entry.project = pr_id;
        entry.min_time = chrono::TimeDelta::minutes(t.min_time as i64);

        for child in t.childs{
            self.unravel_tasks(pt, tt, gt, child, Some(entry.id), pr_id)?;
        }

        tt.push(entry);
        Ok(())
    }

    fn unravel_project(&self, pt: &mut Vec<ProjectTable>, tt: &mut Vec<TaskTable>, gt: &mut Vec<TagTable>, p: Project, p_id : Option<usize>) -> Result<()>{
        let mut entry = ProjectTable::default();
        entry.id = pt.len();
        entry.desc = p.desc.into();
        entry.parent = p_id;
        entry.location = p.location;

        for child in p.childs{
            self.unravel_project(pt, tt, gt, child, Some(entry.id))?;
        }

        for task in p.tasks{
            self.unravel_tasks(pt, tt, gt, task, None, Some(entry.id))?;
        }
        pt.push(entry);

        return Ok(());
    }

    pub fn add_full_project(&mut self, p: Project) -> Result<()>{
        if !p.parent.is_empty(){
            return Err(DatabaseError::NotImplemented);
        }
        let mut pt = Vec::new();
        let mut tt = Vec::new();
        let mut gt = Vec::new();

        self.unravel_project(&mut pt, &mut tt, &mut gt, p, None)?;
        self.projects.append(&mut pt);
        self.tasks.append(&mut tt);
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

    pub fn get_all_projects(&self) -> Vec<ProjectManager>{
        self.projects
            .iter()
            .map(|p| Manager::new(p.id, self))
            .collect()
    }
}

type ProjectManager<'a> = Manager<'a, Project>;

impl<'a> ProjectManager<'a>{
    pub fn get_table(&self) -> &ProjectTable{
        &self.pool.projects[self.id]
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

pub struct ManagerMut<'a, T>{
    pool: &'a mut Database,
    id: usize,
    t: PhantomData<T>,
}

impl<'a, T> Manager<'a, T> {
    fn new(id: usize, pool: &'a Database) -> Self{
        Self {id, pool, t: PhantomData}
    }
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

    #[error("Root project already exists")]
    ProjectExists,

    #[error("Not found {0}")]
    NotFoundOther(String),

    #[error("unknown")]
    Unknown,

    #[error("Other: {0}")]
    Other(String),

    #[error("undefined error")]
    Undefined,

    #[error("Malformed non expected field")]
    Malformed,

    #[error("not implemented")]
    NotImplemented,
}

impl DatabaseError {
    pub fn other<S: AsRef<str>>(s: S) -> Self{
        return DatabaseError::Other(s.as_ref().to_string());
    }
}
