use std::error::Error;

use super::Location;
use ly::proc::builder;
use serde::{Deserialize, Serialize};
use crate::*;

#[builder(name = Project)]
#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    pub(crate) desc: Description,
    // pass to SystemTime
    pub(crate) last_worked: Option<time::Instant>,
    pub(crate) location: Option<Location>,

    #[builder_skip]
    pub(crate) id: usize,
    #[builder(ty = String)]
    pub(crate) parent: Option<usize>,
}

impl ProjectTable {
    pub fn from_project(project: Project, pool: &Pool) -> Self{
        let id = pool.projects.last().and_then(|s| Some(s.id)).unwrap_or(pool.projects.len());
        let parent = if project.parent.is_empty() {
            None
        }else{
            pool.search_project_id(&project.parent).ok()
        };
        return ProjectTable{
            desc: project.desc,
            location: project.location,
            last_worked: project.last_worked,
            parent, id,
        };
    }

    pub fn from_project_result(project: Project, pool: &Pool) -> Result<Self, PoolError>{
        let id = pool.projects.last().and_then(|s| Some(s.id)).unwrap_or(pool.projects.len());
        let parent = if !project.parent.is_empty() {
            Some(pool.search_project_id(&project.parent)?)
        }
        else {
            None
        };
        return Ok(ProjectTable{
            desc: project.desc,
            location: project.location,
            last_worked: project.last_worked,
            parent, id,
        });
    }
}
