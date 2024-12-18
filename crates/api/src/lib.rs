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

#[derive(Debug)]
struct ProjectTree{
    project: ProjectTable,
    childs: Vec<ProjectTree>,
}

#[derive(Debug)]
struct TaskTree{
    task: TaskTable,
    childs: Vec<TaskTree>,
}

impl TaskTree{
    fn new(task: TaskTable) -> Self{ Self {task, childs: Vec::new()} }

    fn into_task(self) -> Task{
        let mut task = self.task.naive_task();
        task.childs = self.childs
            .into_iter()
            .map(Self::into_task)
            .collect();

        return task;
    }
}

impl ProjectTree{
    fn new(project: ProjectTable) -> Self{ Self {project, childs: Vec::new()} }

    fn into_project(self) -> Project{
        let mut project = self.project.naive_project();
        project.childs = self.childs
            .into_iter()
            .map(Self::into_project)
            .collect();

        return project;
    }
}

impl Database{
    // [boilerplate start]
    pub fn get_all_projects(&self) -> Vec<ProjectManager>{
        self.projects
            .iter()
            .map(|p| Manager::new(p.id, self))
            .collect()
    }

    pub fn get_all_tasks(&self) -> Vec<TaskManager>{
        self.projects
            .iter()
            .map(|p| Manager::new(p.id, self))
            .collect()
    }

    pub fn get_all_tags(&self) -> Vec<Manager<TagTable>>{
        self.tags
            .iter()
            .map(|p| Manager::new(p.id, self))
            .collect()
    }

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

    pub fn search_tag_id<P>(&self, p: P) -> Result<usize>
    where
        P: FnMut(&&TagTable) -> bool,
    {
        self.tags.iter()
            .find(p)
            .ok_or(DatabaseError::NotFound)
            .and_then(|p| Ok(p.id))
    }

    pub fn search_project<P>(&self, p: P) -> Result<Manager<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool
    {
        self.search_project_id(p).and_then(|p| Ok(Manager::new(p, self)))
    }

    pub fn search_project_mut<P>(&mut self, p: P) -> Result<ManagerMut<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
        self.search_project_id(p).and_then(|p| Ok(ManagerMut::new(p, self)))
    }

    pub fn search_task<P>(&self, p: P) -> Result<Manager<TaskTable>>
    where
        P: FnMut(&&TaskTable) -> bool,
    {
        Ok(Manager::new(self.search_task_id(p)?, self))
    }

    pub fn search_tag<P>(&self, p: P) -> Result<Manager<TagTable>>
    where
        P: FnMut(&&TagTable) -> bool,
    {
        Ok(Manager::new(self.search_tag_id(p)?, self))
    }
    // [boilerplate end]


    pub fn build_project(&self) -> Result<Project>{ Err(DatabaseError::NotImplemented) }

    fn unravel_tasks(&self, pt: &mut Vec<ProjectTable>, tt: &mut Vec<TaskTable>, gt: &mut Vec<TagTable>, t: Task, pa_id: Option<usize>, pr_id: Option<usize>) -> Result<()>{
        let mut entry = TaskTable::default();
        entry.id = tt.len();
        entry.desc = t.desc.into();
        entry.parent = pa_id;
        entry.project = pr_id;
        entry.min_time = chrono::TimeDelta::minutes(t.min_time as i64);
        let id = entry.id;

        tt.push(entry);
        for child in t.childs{
            self.unravel_tasks(pt, tt, gt, child, Some(id), pr_id)?;
        }

        Ok(())
    }

    fn unravel_project(&self, pt: &mut Vec<ProjectTable>, tt: &mut Vec<TaskTable>, gt: &mut Vec<TagTable>, p: Project, p_id : Option<usize>) -> Result<()>{
        let mut entry = ProjectTable::default();
        entry.id = pt.len();
        entry.desc = p.desc.into();
        entry.parent = p_id;
        entry.location = p.location;

        let id = entry.id;
        pt.push(entry);

        for child in p.childs{
            self.unravel_project(pt, tt, gt, child, Some(id))?;
        }

        for task in p.tasks{
            self.unravel_tasks(pt, tt, gt, task, None, Some(id))?;
        }

        return Ok(());
    }

    fn project_offset(&self) -> usize {
        self.projects
            .last()
            .and_then(|p| Some(p.id + 1))
            .unwrap_or(self.projects.len())
    }
    fn task_offset(&self) -> usize {
        self.tasks
            .last()
            .and_then(|p| Some(p.id + 1))
            .unwrap_or(self.tasks.len())
    }

    fn add_buffer(&mut self, pt: &mut Vec<ProjectTable>, tt: &mut Vec<TaskTable>, _: &mut Vec<TagTable>){
        self.projects.append(pt);
        self.tasks.append(tt);
    }

    pub fn add_full_task(&mut self, t: Task) -> Result<()>{
        let mut pt = Vec::new();
        let mut tt = Vec::new();
        let mut gt = Vec::new();

        let pa_id = if t.project.is_empty() {
            None
        } else {
            Some(self.projects
                .iter()
                .find(|p| p.desc.name == t.project)
                .and_then(|p| Some(p.id))
                .ok_or(DatabaseError::NotFound)?
            )
        };

        self.unravel_tasks(&mut pt, &mut tt, &mut gt, t, None, pa_id)?;

        let offset = self.task_offset();
        for task in &mut tt{
            task.parent.as_mut().and_then(|p| Some(*p += offset));
            task.id += offset;
        }

        self.add_buffer(&mut pt, &mut tt, &mut gt);
        Ok(())
    }

    pub fn add_full_project(&mut self, p: Project) -> Result<()>{
        if !p.parent.is_empty(){
            return Err(DatabaseError::NotImplemented);
        }
        let mut pt = Vec::new();
        let mut tt = Vec::new();
        let mut gt = Vec::new();

        self.unravel_project(&mut pt, &mut tt, &mut gt, p, None)?;

        let t_offset = self.task_offset();
        let p_offset = self.project_offset();
        for task in &mut tt{
            task.parent.as_mut().and_then(|p| Some(*p += t_offset));
            task.id += t_offset;
        }
        for project in &mut pt{
            project.parent.as_mut().and_then(|p| Some(*p += p_offset));
            project.id += p_offset;
        }

        self.add_buffer(&mut pt, &mut tt, &mut gt);
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

    pub fn build_task_tree(&self, id: usize) -> Result<Task>{
        let mut root = TaskTree::new(
            self.tasks.iter().find(|t| t.id == id)
            .ok_or(DatabaseError::NotFound)?
            .clone()
        );
        let mut buffer : Vec<_> = self.tasks.clone().into_iter().filter(|t| t.id == id).collect();

        fn search_in(table: &TaskTable, v: &mut Vec<TaskTree>) -> bool{
            for root in v{
                if root.task.id == table.parent.unwrap(){
                    root.childs.push(TaskTree::new(table.clone()));
                    return true;
                }
                let found = search_in(table, &mut root.childs);
                if found {
                    return true;
                }
            }
            return false;
        }

        buffer.sort_by(|a, b| b.id.cmp(&a.id));
        while buffer.len() != 0{
            let top = buffer.pop().unwrap();
            if top.parent.is_none(){
                continue;
            }
            if top.parent.unwrap() == root.task.id{
                root.childs.push(TaskTree::new(top));
                continue;
            }
            if !search_in(&top, &mut root.childs){
                let _ = warn!("task not found in tree");
            }
        }

        return Ok(root.into_task());
    }

    pub fn build_project_trees(&self) -> Result<Vec<Project>>{
        let mut buffer = self.projects.clone();
        let mut roots = Vec::new();

        loop{
            let proj = buffer.iter().enumerate().find(|p| p.1.parent.is_none());
            if let Some((i, _)) = proj {
                roots.push(ProjectTree{
                    project: buffer.remove(i),
                    childs: Vec::new(),
                });
            }
            else{
                break;
            }
        }

        fn search_in(table: &ProjectTable, v: &mut Vec<ProjectTree>) -> bool{
            for root in v{
                if root.project.id == table.parent.unwrap(){
                    root.childs.push(ProjectTree::new(table.clone()));
                    return true;
                }
                let found = search_in(table, &mut root.childs);
                if found {
                    return true;
                }
            }
            return false;
        }

        buffer.sort_by(|a, b| b.id.cmp(&a.id));
        while buffer.len() != 0{
            let top = buffer.pop().unwrap();
            if !search_in(&top, &mut roots) {
                let error = format!("could not find parent ({}) project for {}", top.parent.unwrap(), top.desc.name);
                return Err(DatabaseError::other(error));
            }
        }

        return Ok(roots
            .into_iter()
            .map(ProjectTree::into_project)
            .collect()
        );
    }

    pub fn get_writer_mut(&mut self) -> &mut Box<dyn DatabaseWriter>{
        return &mut self.writer;
    }

    pub fn get_reader_mut(&mut self) -> &mut Box<dyn DatabaseReader>{
        return &mut self.reader;
    }
}

type ProjectManager<'a> = Manager<'a, Project>;
type ProjectManagerMut<'a> = ManagerMut<'a, Project>;

type TaskManager<'a> = Manager<'a, Task>;
type TaskManagerMut<'a> = ManagerMut<'a, Task>;

impl<'a> ProjectManager<'a>{
    pub fn name(&self) -> &String{
        &self.get_table().desc.name
    }

    pub fn location(&self) -> &Location{
        &self.get_table().location
    }

    pub fn get_table(&self) -> &ProjectTable{
        &self.pool.projects[self.id]
    }
}

impl<'a> TaskManager<'a>{
    pub fn name(&self) -> &String{
        &self.get_table().desc.name
    }

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

    #[error("database function not implemented")]
    NotImplemented,
}

impl DatabaseError {
    pub fn other<S: AsRef<str>>(s: S) -> Self{
        return DatabaseError::Other(s.as_ref().to_string());
    }
}
