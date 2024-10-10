pub mod project;
pub mod tags;
pub mod task;
pub mod trees;
pub mod desc;

use std::{error::Error, marker::PhantomData, path::PathBuf, time::{self, Duration, SystemTime}};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use ly::log::prelude::*;

type Timestamp = SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Location{
    Path(PathBuf),
    Git(String),
}

type Result<T> = std::result::Result<T, DatabaseError>;

struct Database{}

impl Database
where
{
    pub fn search_entry_id<T>(&self, ident: T) -> Result<usize>
    where
        T: std::fmt::Debug,
    {
        let ident = format!("{:?}", ident);
        return Err(DatabaseError::IdentNotFound{ ident });
    }

    pub fn get_entry<T>(&self, id: usize) -> Result<Manager<'static, T>>{
        return Err(DatabaseError::IdNotFound { id });
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
}

impl<'a, T> Manager<'a, T> {
    fn new(id: usize, pool: &'a mut Database) -> Self{
        Self {id, pool, t: PhantomData}
    }
}
