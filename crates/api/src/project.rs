use super::Location;
use serde::{Deserialize, Serialize};
use crate::*;

#[derive(Debug, Default, Clone)]
pub struct Project{
    desc: Description,
    last_worked: Option<time::Instant>,
    location: Option<Location>,
    parent: Option<String>,
}

impl Project {
    pub const fn new(desc: Description) -> Self{
        Self {desc, last_worked: None, location: None, parent: None}
    }
    pub fn last_worked(mut self, last_worked : time::Instant) -> Self{ self.last_worked = Some(last_worked); self }
    pub fn location(mut self, location : Location) -> Self{ self.location = Some(location); self }
    pub fn parent(mut self, parent : String) -> Self{ self.parent = Some(parent); self }
}

#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    pub(crate) desc: Description,
    pub(crate) last_worked: Option<time::Instant>,
    pub(crate) location: Option<Location>,

    pub(crate) id    : usize,
    pub(crate) parent: Option<usize>,
}

impl ProjectTable {
    pub fn from_project(project: Project, pool: &Pool) -> Self{
        let id = pool.projects.last().and_then(|s| Some(s.id)).unwrap_or(pool.projects.len());
        let parent = project.parent.and_then( |p| pool.search_project_id(p).ok()) ;
        return ProjectTable{
            desc: project.desc,
            location: project.location,
            last_worked: project.last_worked,
            parent, id,
        };
    }

    pub fn from_project_result(project: Project, pool: &Pool) -> Result<Self, PoolError>{
        let id = pool.projects.last().and_then(|s| Some(s.id)).unwrap_or(pool.projects.len());
        let parent = if let Some(parent) = project.parent {
            Some(pool.search_project_id(parent)?)
        }
        else { None };
        return Ok(ProjectTable{
            desc: project.desc,
            location: project.location,
            last_worked: project.last_worked,
            parent, id,
        });
    }
}
