use ly::proc::builder;
use serde::{Deserialize, Serialize};

use crate::Database;
use crate::Result;
use crate::DatabaseError;

const DEFAULT_EDITION : &str = "0.1.0";
const DEFAULT_VERSION : &str = "0.5.2";

fn default_edition() -> String{
    return DEFAULT_EDITION.to_string();
}
fn default_version() -> String{
    return DEFAULT_VERSION.to_string();
}

const fn default_min_time() -> i64 { 30 }

#[builder(name = Descriptor, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Description{
    pub name       : String,

    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub description: String,

    pub priority   : f64,
    pub difficulty : f64,

    #[builder(pass = serde(default = "default_version"))]
    #[builder(init = default_version() )]
    pub version    : String,

    #[builder(pass = serde(default = "default_edition"))]
    #[builder(init = default_edition())]
    pub edition    : String,

    #[builder(ty = String)]
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "String::is_empty"))]
    pub due_date   : Option<chrono::NaiveDate>,
    
    // minimun time in minutes
    #[builder(ty = i64)]
    #[builder(pass = serde(default= "default_min_time"))]
    pub min_time   :chrono::Duration,

    #[builder(skip_table)]
    #[builder(pass = serde(default))]
    #[builder(pass = serde(skip_serializing_if = "Vec::is_empty"))]
    pub tags       : Vec<String>,
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
        return Err(DatabaseError::other("Could not create Description form Descriptor"));
    }
}
impl Description {
    pub fn naive_description(self) -> Descriptor{
        Descriptor {
            name: self.name,
            description: self.description,
            priority: self.priority,
            difficulty: self.difficulty,
            version: self.version,
            edition: self.edition,
            due_date: self.due_date
                .and_then(|p| Some(p.format("%d %m %Y").to_string()))
                .unwrap_or(String::new()),
            min_time: self.min_time.num_minutes(),
            tags: Vec::new()
        }
    }
}
