use std::time::SystemTime;
use ly::proc::builder;
use serde::{Deserialize, Serialize};



#[builder(name = Descriptor, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Description{
    pub(crate) name       : String,
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) escription: String,
    pub(crate) riority   : f64,
    pub(crate) ifficulty : f64,

    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) due_date   : String,

    #[builder(skip_table)]
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    pub(crate) tags       : Vec<String>,
}

use crate::{Pool, PoolError};

impl Description {
    pub fn from_descriptor(des: Descriptor, pool: &Pool) -> Result<Self, PoolError>{
        return Err(PoolError::Unknown);
    }
}


pub struct TagTask{
    tag: usize,
    task: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TagProject{
    tag: usize,
    proj: usize,
}
