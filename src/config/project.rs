use toml::Table;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Id{
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoTable{
    pub done: Table,
    pub todo: Table,
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct Project{
    pub id: Id,
    pub features: TodoTable,
    pub  front: TodoTable,
    pub middle: TodoTable,
    pub   back: TodoTable,
}
