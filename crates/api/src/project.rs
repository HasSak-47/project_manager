use std::{fmt::Debug, rc::Rc};

use super::Location;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};


#[allow(dead_code)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Feature{
    name: String,
    #[serde(default)] 
    status: String,
    #[serde(default)] 
    description: String,
    priority: u8,
    difficulty: u8,

    #[serde(default)] 
    #[serde(skip_serializing_if = "Vec::is_empty")]
    todo: Vec<Feature>,
    #[serde(default)] 
    #[serde(skip_serializing_if = "Vec::is_empty")]
    done: Vec<Feature>,
}

impl Feature{
    pub fn new(name: String, description: String, status: String, priority: u8, difficulty: u8) -> Self {
        Feature{
            name,
            priority, difficulty,
            todo: Vec::new(), done: Vec::new(),
            description,
            status,
        }
    }
}

use crate::current_edition;

/**
 * this data is just for decoration
 */
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectData{
    pub name: String,
    #[serde(default)] 
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default)] 
    #[serde(skip_serializing_if = "String::is_empty")]
    pub version: String,
    #[serde(default = "current_edition")] 
    pub edition: String,
}


// the status of the project
// this is what the file will contain
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectStatus{
    pub project: ProjectData,
    #[serde(default)] 
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub todo: Vec<Feature>,
    #[serde(default)] 
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub done: Vec<Feature>,
}

impl ProjectStatus{
    pub fn new(name: String, description: String) -> Self {
        ProjectStatus{
            project: ProjectData{
                name, description,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn add_todo(&mut self, feature: Feature){ self.todo.push(feature); }
    pub fn add_done(&mut self, feature: Feature){ self.done.push(feature); }

    fn remove_from(features: &mut Vec<Feature>, feature_name: String) -> Result<Feature>{
        let i = features.iter().position(|f| f.name == feature_name);
        match i{
            Some(i) => Ok(features.remove(i)),
            None => Err(anyhow::anyhow!("Feature not found")),
        }
    }
    
    pub fn mark_done(&mut self, feature_name: String) -> Result<()>{
        let p = ProjectStatus::remove_from(&mut self.todo, feature_name);
        self.done.push(p?);
        Ok(())
    }

    pub fn mark_todo(&mut self, feature_name: String) -> Result<()>{
        let p = ProjectStatus::remove_from(&mut self.done, feature_name);
        self.todo.push(p?);
        Ok(())
    }

    fn get_features_difficulty<S>(v: Option<&Vec<Feature>>, selector: &S) -> usize
    where
        S: Fn(&Feature) -> Option<&Vec<Feature>>
    {
        if v.is_none(){
            return 0;
        }

        let v = v.unwrap();
        let mut d = 0;
        for f in v{
            d += f.difficulty as usize;
            d += ProjectStatus::get_features_difficulty(selector(f), selector);
        }

        d
    }

    pub fn get_todo_difficulty(&self) -> usize{ ProjectStatus::get_features_difficulty(Some(&self.todo), &|f| Some(&f.todo)) }
    pub fn get_done_difficulty(&self) -> usize{ ProjectStatus::get_features_difficulty(Some(&self.done), &|f| Some(&f.done)) }

    pub fn get_completion(&self) -> f64{
        let todo = self.get_todo_difficulty();
        let done = self.get_done_difficulty();
        let total = todo + done;
        if total == 0{
            return 0.;
        }
        done as f64 / total as f64
    }

    // fn add_to(v: &mut Vec<Feature>, f: Feature){ v.push(f); }
}


// info on the project that is stored in the manager and project
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectInfo{
    pub name: String,
    pub location: Location,

    /** if None, status is in the project folder */
    #[serde(default)] 
    pub status: Option<Location>, 
    #[serde(default)] 
    pub last_update: Option<usize>, // timestamp
}

// the project that is loaded in memory
// it has an info and a status
// the status may not be loaded
// and may be too big to be in the stack
// therefore it is a box
#[derive(Debug, Default, Clone)]
pub struct Project{
    pub info: ProjectInfo,
    pub status: Option<Box<ProjectStatus>>, // the project may not be loaded
}

impl Project{
    pub fn unload(&mut self) {
        self.status = None;
    }

    pub fn load<I: IO>(&mut self, reader: &I) -> Result<()>{
        if !self.is_loaded(){
            self.status = Some(Box::new(reader.read(&self.info.location)?));
        }
        Ok(())
    }

    pub fn is_loaded(&self) -> bool{ self.status.is_some() }
}

pub trait IO{
    fn read(&self, location: &Location) -> Result<ProjectStatus>;
    fn write(&mut self, location: &Location, prj: &ProjectStatus) -> Result<()>;
}

#[derive(Default)]
pub struct ProjectHandler{
    io: Option<Box<dyn IO>>,
}

impl Debug for ProjectHandler{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "ProjectHandler{{ io: {}, }}", self.io.is_some())
    }
}

impl ProjectHandler{
    pub fn new() -> Self{ ProjectHandler{ io: None, } }
    pub fn set_io(&mut self, writer: Box<dyn IO>){ self.io = Some(writer); }

    pub fn write(&mut self, project: &Project) -> Result<()> {
        match &mut self.io{
            Some(s) => {
                let status = match &project.status{
                    Some(s) => s,
                    None => return Err(anyhow::anyhow!("there is no project status")),
                };
                s.write(&project.info.location, &status)?;
            },
            None => return Err(anyhow::anyhow!("there is no project writer")),
        }
        Ok(())
    }

    pub fn read(&self, project: &mut Project) -> Result<()> {
        match &self.io {
            Some(s) => {
                project.status = Some(Box::new(s.read(&project.info.location)?));

                Ok(())
            },
            None => return Err(anyhow::anyhow!("there is no project reader")),
        }
    }
}
