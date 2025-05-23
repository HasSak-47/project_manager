/*
 * this is just the toml parser thingy
 */
use std::{
    path::PathBuf,
    collections::HashMap, fmt::Display,
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum Location{
    Path{path: PathBuf},
    Other(String),
}

impl Display for Location{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path{path} => write!(f, "location::path {}", path.display()),
            Self::Other(o) => write!(f, "location::other {o}"),
        }
    }
}

impl Location{
    pub fn path(path: PathBuf) -> Self{ Self::Path {path} }
    pub fn to_string(&self) -> String { 
        match self {
            Self::Path{path} => path.to_string_lossy().to_string(),
            Self::Other(o) => o.clone(),
        }
    }
}

impl Default for Location{ fn default() -> Self { Self::Other(String::new()) } }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProjectData{
    pub last_updated: Option<u64>,
    pub location   : Location,
    pub description: Option<String>,
    pub subprojects: Option<Vec<String>>, // Vector of the name of the projects
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ManagerInfo{
    pub version : String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manager{
    pub manager: ManagerInfo,
    pub projects: HashMap<String, ProjectData>,
}
