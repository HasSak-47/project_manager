pub mod project;
pub mod tags;
pub mod task;
pub mod trees;
pub mod desc;

use std::{collections::HashMap, error::Error, marker::PhantomData, path::PathBuf, time::{self, Duration, SystemTime}};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use ly::log::prelude::*;

use project::ProjectTable;
use tags::TagTable;
use task::TaskTable;


type Timestamp = SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Git(String),
}

type Result<T> = std::result::Result<T, DatabaseError>;

pub trait FromSchema<T> where Self: Sized{
    fn from_schema(schema: T, db: &Database) -> Result<Self>;
}

pub trait Idable{
    fn set_id(&mut self);
    fn get_id(&mut self) -> usize;
}

#[derive(Debug, Default)]
struct Database{
    pub projects: Vec<ProjectTable>,
    pub tasks   : Vec<TaskTable>,
    pub tags    : Vec<TagTable>,
}

impl Database{
}

trait Find<T> where
Self: Sized{
    fn find(&self, db: &Database) -> Result<T>{
        return Err(DatabaseError::NotImplemented);
    }
}

pub struct Manager<'a, T>{
    pool: &'a mut Database,
    id: usize,
    t: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum DatabaseError{
    #[error("Ident {ident:?} not found ")]
    IdentNotFound{ident: String},
    #[error("{id} not found ")]
    IdNotFound{id: usize},

    #[error("Other error: {0}")]
    Other(#[from] Box<dyn Error>),

    #[error("unknown")]
    Unknown,

    #[error("undefined error")]
    Undefined,

    #[error("not implemented")]
    NotImplemented,
}

impl<'a, T> Manager<'a, T> {
    fn new(id: usize, pool: &'a mut Database) -> Self{
        Self {id, pool, t: PhantomData}
    }
}
