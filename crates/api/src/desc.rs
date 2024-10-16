use std::time::Duration;

use ly::proc::builder;
use serde::{Deserialize, Serialize};

use crate::Database;
use crate::Result;
use crate::DatabaseError;
use crate::Timestamp;

const DEFAULT_EDITION : &str = "0.1.0";
const DEFAULT_VERSION : &str = "0.5.2";

fn default_edition() -> String{
    return DEFAULT_EDITION.to_string();
}
fn default_version() -> String{
    return DEFAULT_VERSION.to_string();
}

const fn default_min_time() -> i64 {
    30
}

#[builder(name = Descriptor, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Description{
    pub(crate) name       : String,

    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) description: String,

    pub(crate) priority   : f64,
    pub(crate) difficulty : f64,

    #[builder(pass = serde(default = "default_version"))]
    #[builder(init = default_version() )]
    pub(crate) version    : String,

    #[builder(pass = serde(default = "default_edition"))]
    #[builder(init = default_edition())]
    pub(crate) edition    : String,

    #[builder(ty = String)]
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) due_date   : Option<chrono::NaiveDate>,
    
    #[builder(ty = i64)]
    #[builder(pass = serde(default= "default_min_time"))]
    pub(crate) min_time   :chrono::Duration,

    #[builder(skip_table)]
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    pub(crate) tags       : Vec<String>,
}

impl From<Descriptor> for Description{
    fn from(value: Descriptor) -> Self {
        let due_date = if value.due_date.is_empty(){
            None
        } else {
            Some(chrono::NaiveDate::parse_from_str(&value.due_date, "%d %m %Y").unwrap())
        };
        Self{
            name        : value.name       ,
            description : value.description,
            priority    : value.priority   ,
            difficulty  : value.difficulty ,
            version     : value.version    ,
            edition     : value.edition    ,
            min_time    : chrono::Duration::minutes(value.min_time),
            due_date,
        }
    }
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
