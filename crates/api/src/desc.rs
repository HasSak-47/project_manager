use ly::proc::builder;
use serde::{Deserialize, Serialize};

use crate::Database;
use crate::Result;
use crate::DatabaseError;

#[builder(name = Descriptor, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Description{
    pub(crate) name       : String,
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) description: String,
    pub(crate) priority   : f64,
    pub(crate) difficulty : f64,

    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) due_date   : String,

    #[builder(skip_table)]
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    pub(crate) tags       : Vec<String>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TagTask{
    tag: usize,
    task: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TagProject{
    tag: usize,
    proj: usize,
}

impl Description {
    pub fn from_descriptor(des: Descriptor, db: &Database) -> Result<Self>{
        return Err(DatabaseError::Unknown);
    }
}
