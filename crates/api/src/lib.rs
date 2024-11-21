pub mod project;
pub mod utils;
pub mod tags;
pub mod task;
pub mod desc;

use std::{marker::PhantomData, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use ly::log::prelude::*;

use project::{Project, ProjectTable};
use tags::{TagOtherTable, TagTable};
use task::{Task, TaskTable};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Git(String),
    #[default]
    None,
}

impl Location {
    pub fn is_path(&self, p: &PathBuf) -> bool{
        if let Location::Path(path) = self{
            return path == p;
        }
        return false;
    }
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

pub trait Idable{
    fn set_id(&mut self);
    fn get_id(&mut self) -> usize;
}

#[allow(dead_code)]
type TagProjectTable = TagOtherTable<ProjectTable>;
#[allow(dead_code)]
type TagTaskTable = TagOtherTable<TaskTable>;

#[derive(Default)]
pub struct Database{
    projects: Vec<ProjectTable>,
    tasks   : Vec<TaskTable>,
    tags    : Vec<TagTable>,

    #[allow(dead_code)]
    tag_pro : Vec<TagOtherTable<ProjectTable>>,
    #[allow(dead_code)]
    tag_task: Vec<TagOtherTable<TaskTable>>,
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

#[derive(Debug, Default)]
struct ProjectTree{
    project: ProjectTable,
    childs: Vec<ProjectTree>,
    tasks : Vec<Task>,
}

#[derive(Debug, Default)]
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
    fn new(project: ProjectTable) -> Self{ Self {
        project,
        childs: Vec::new(),
        tasks: Vec::new()
    }}

    fn into_project(self) -> Project{
        let mut project = self.project.naive_project();
        project.childs = self.childs
            .into_iter()
            .map(Self::into_project)
            .collect();

        project.tasks = self.tasks;

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

    pub fn get_project_id<P>(&self, p: P) -> Result<usize>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
            self.projects
                .iter()
                .find(p)
                .ok_or(DatabaseError::NotFound)
                .and_then(|p| Ok(p.id))
    }

    pub fn get_task_id<P>(&self, p: P) -> Result<usize>
    where
        P: FnMut(&&TaskTable) -> bool,
    {
            self.tasks
                .iter()
                .find(p)
                .ok_or(DatabaseError::NotFound)
                .and_then(|p| Ok(p.id))
    }

    pub fn get_tag_id<P>(&self, p: P) -> Result<usize>
    where
        P: FnMut(&&TagTable) -> bool,
    {
        self.tags.iter()
            .find(p)
            .ok_or(DatabaseError::NotFound)
            .and_then(|p| Ok(p.id))
    }

    pub fn get_project<P>(&self, p: P) -> Result<Manager<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool
    {
        self.get_project_id(p).and_then(|p| Ok(Manager::new(p, self)))
    }

    pub fn get_project_mut<P>(&mut self, p: P) -> Result<ManagerMut<ProjectTable>>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
        self.get_project_id(p).and_then(|p| Ok(ManagerMut::new(p, self)))
    }

    pub fn get_task<P>(&self, p: P) -> Result<Manager<TaskTable>>
    where
        P: FnMut(&&TaskTable) -> bool,
    {
        Ok(Manager::new(self.get_task_id(p)?, self))
    }

    pub fn get_tag<P>(&self, p: P) -> Result<Manager<TagTable>>
    where
        P: FnMut(&&TagTable) -> bool,
    {
        Ok(Manager::new(self.get_tag_id(p)?, self))
    }

    pub fn get_tasks_where<P>(&self, p: P) -> Vec<TaskManager>
    where
        P: FnMut(&&TaskTable) -> bool,
    {
        self.tasks.iter().filter(p).map(|k| TaskManager::new(k.id, self)).collect()
    }

    pub fn get_projects_where<P>(&self, p: P) -> Vec<ProjectManager>
    where
        P: FnMut(&&ProjectTable) -> bool,
    {
        self.projects.iter().filter(p).map(|k| ProjectManager::new(k.id, self)).collect()
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

    fn __take_childs_tasks(table: &mut TaskTree, v: &mut Vec<TaskTree>) {
        // NOTE: this assumes that all childs are after the parent in the vector
        let mut index = 0;
        loop {
            println!("__take_childs_tasks: {index} < {}", v.len());
            if !(index < v.len()){
                break;
            }
            if let Some(parent) = v[index].task.parent{
                if parent == table.task.id{
                    table.childs.push( v.remove(index) );
                }
                else{
                    index += 1;
                }
            }
            else{
                index += 1;
            }
        }
    }

    fn __take_childs_projects(table: &mut ProjectTree, v: &mut Vec<ProjectTree>) {
        // NOTE: this assumes that all childs are after the parent in the vector
        // if it is not the case replace with a search all the thing again lmao
        let mut index = 0;
        loop {
            if !(index < v.len()){
                break;
            }
            if let Some(parent) = v[index].project.parent{
                if parent == table.project.id{
                    table.childs.push( v.remove(index));
                }
                else{
                    index += 1;
                }
            }
            // since a task at i was removed, a new task will appear in i
            // this appears all over the file and it is hideous
            else{
                index += 1;
            }
        }
    }

    fn __build_task_tree(id: usize, buffer: &mut Vec<TaskTree>) -> Result<TaskTree> {
        let root_pos = buffer.iter().position(|p| p.task.id == id).ok_or(DatabaseError::NotFound)?;
        let mut root = buffer.remove(root_pos);

        Self::__take_childs_tasks(&mut root, buffer);
        return Ok(root);
    }

    fn __build_project_tree(id: usize, buffer: &mut Vec<ProjectTree>) -> Result<ProjectTree> {
        let root_pos = buffer.iter().position(|p| p.project.id == id).ok_or(DatabaseError::NotFound)?;
        let mut root = buffer.remove(root_pos);

        Self::__take_childs_projects(&mut root, buffer);
        return Ok(root);
    }

    pub fn build_task_tree(&self, id: usize) -> Result<Task>{
        let mut buffer = self.tasks.clone().into_iter().map(TaskTree::new).collect();
        return Ok(Self::__build_task_tree(id, &mut buffer)?.into_task());
    }

    pub fn build_project_trees(&self) -> Result<Vec<Project>>{
        let mut project_buffer : Vec<_> = self.projects.clone().into_iter().map(ProjectTree::new).collect();
        let mut task_buffer : Vec<_> = self.tasks.clone().into_iter().map(TaskTree::new).collect();

        // first add all tasks for each project
        for project in &mut project_buffer{
            let mut index = 0;
            // NOTE: this assumes that all childs are after the parent in the vector
            loop {
                println!("{index} < {}", task_buffer.len());
                if index >= task_buffer.len(){
                    break;
                }

                if let Some(task_project) = task_buffer[index].task.project{
                    if project.project.id == task_project{
                        let task = Self::__build_task_tree(task_buffer[index].task.id, &mut task_buffer)?.into_task();
                        project.tasks.push(task);
                    }
                }
                else{
                    index += 1;
                }
            }
        }

        // root projects
        let mut roots = Vec::new();
        let mut index = 0;
        loop {
            if !(index < project_buffer.len()){
                break;
            }
            if  project_buffer[index].project.parent.is_none(){
                let new_root = project_buffer.remove(index);
                roots.push(new_root);
            }
            else{
                index += 1;
            }
        }

        for root in &mut roots{
            Self::__take_childs_projects(root, &mut project_buffer);
        }

        return Ok(roots
            .into_iter()
            .map(ProjectTree::into_project)
            .collect()
        );
    }
}

#[derive(Debug, Default)]
pub enum NodeType{
    #[default]
    Project,
    Task,
}

#[derive(Debug)]
pub struct Explorer {
    node_t: NodeType,
    dir: Vec<(NodeType, usize)>,
}

impl Explorer {
    fn root() -> Self{ 
        Self{
            node_t: NodeType::Project,
            dir: Vec::new(),
        }
    }

    /**
     * getting a task: /project_a/project_b/.../!task_a/task_b/.../task_c
     * getting a project: /project_a/project_b/.../project_c
     */
    fn get_desired_target(path: String, database: &Database) -> Option<Vec<usize>>{
        // HACK: I hate you rust
        let direns : Vec<String> = path.split("/").into_iter().map(String::from).collect();
        let mut direns : Vec<String> = direns.into_iter().rev().collect();
        let mut projects = Vec::new();
        // gets all projects
        loop {
            if let Some(data) = direns.pop(){
                assert_ne!(data, "", "this is not well formatted");
                let mut chars = data.chars();
                if chars.next().unwrap() == '!'{
                    break;
                }
                else
                if chars.next().is_none(){
                    assert!(false, "this other stuff is not well formatted")
                }
                projects.push(data.clone());
                continue;
            }
            break;
        }
        return None;
    }
}

#[cfg(test)]
mod exploter_test{
    use crate::{Database, Explorer};

    #[test]
    fn test_target(){
        let d = Database::default();
        Explorer::get_desired_target("projecta/projectb/!task/!task".to_string(), &d);
    }
}

type ProjectManager<'a> = Manager<'a, Project>;
#[allow(dead_code)]
type ProjectManagerMut<'a> = ManagerMut<'a, Project>;

type TaskManager<'a> = Manager<'a, Task>;
#[allow(dead_code)]
type TaskManagerMut<'a> = ManagerMut<'a, Task>;

pub struct Manager<'a, T>{
    pool: &'a Database,
    id: usize,
    t: PhantomData<T>,
}

#[allow(dead_code)]
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
