use std::time::SystemTime;

use ly::proc::builder;
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Clone)]
pub struct SanatizedDescription {
    name       : String,
    description: String,
    priority   : f64,
    difficulty : f64,
    due_date   : Option<SystemTime>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Description {
    name       : String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    priority   : f64,
    difficulty : f64,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    due_date   : String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags       : Vec<String>,
}


#[builder(name = Tag)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TagTable{
    name       : String,
    #[builder_pass(serde(skip_serializing_if = "String::is_empty"))]
    description: String,
    #[builder(def_val = 1.0)]
    priority   : f64,
    #[builder(def_val = 1.0)]
    difficulty : f64,
    #[builder(ty = String)]
    due_date   : Option<SystemTime>,
    #[builder_skip]
    id : usize,
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
