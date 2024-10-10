use std::error::Error;

use super::Location;
use chrono::Timelike;
use ly::proc::builder;
use serde::{Deserialize, Serialize};
use crate::*;
use crate::desc::{Descriptor, Description};


#[builder(name = Project, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    #[builder(ty = Descriptor)]
    pub(crate) desc: Description,

    #[builder(ty=String)]
    pub(crate) last_worked: Option<Timestamp>,
    pub(crate) location: Option<Location>,

    #[builder(skip)]
    pub(crate) id: usize,
    #[builder(ty = String)]
    pub(crate) parent: Option<usize>,
}

impl ProjectTable {
    pub fn from_project(project: Project, pool: &Pool) -> Result<Self, PoolError>{
        let id = pool.projects.last().and_then(|s| Some(s.id)).unwrap_or(pool.projects.len());
        let parent = if !project.parent.is_empty() {
            Some(pool.search_project_id(&project.parent)?)
        }
        else {
            None
        };
        return Ok(ProjectTable{
            desc: Description::from_descriptor( project.desc, pool )?,
            location: project.location,
            last_worked: Some(chrono::DateTime::parse_from_str(&project.last_worked, "")
                .and_then(|d| Ok( SystemTime::UNIX_EPOCH + Duration::from_secs( d.second() as u64 ) ))
                .map_err(PoolError::from_error)?
            ),
            parent, id,
        });
    }
}
