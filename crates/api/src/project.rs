use std::{fmt::Debug, rc::Rc};

use super::Location;
use anyhow::{anyhow, Result};

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Feature{
    name: String,
    description: String,
    status: String,
    priority: u8,
    difficulty: u8,

    todo: Vec<Feature>,
    done: Vec<Feature>,
}

impl Feature{
    pub fn new(name: String, description: String, status: String, priority: u8, difficulty: u8) -> Self {
        Feature{name, description, status, priority, difficulty, todo: Vec::new(), done: Vec::new()}
    }
}

// the status of the project
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ProjectStatus{
    pub name: String,
    pub description: String,
    pub todo: Vec<Feature>,
    pub done: Vec<Feature>,
}

impl ProjectStatus{
    pub fn new(name: String, description: String) -> Self {
        ProjectStatus{name, description, todo: Vec::new(), done: Vec::new()}
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


// info on the project
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ProjectInfo{
    pub name: String,
    pub location: Location,
    pub status: Option<Location>, // if None, status is in the project folder
    pub last_update: Option<usize>, // timestamp
}

// the project inside the manager
#[derive(Debug, Default)]
pub struct Project{
    pub info: ProjectInfo,
    pub status: Option<Box<ProjectStatus>>, // the project may not be loaded
}

impl Project{
    pub fn unload(&mut self) {
        self.status = None;
    }

    pub fn load<R: Reader>(&mut self, reader: &R) -> Result<()>{
        if !self.is_loaded(){
            self.status = Some(Box::new(reader.read(&self.info.location)?));
        }
        Ok(())
    }

    pub fn is_loaded(&self) -> bool{ self.status.is_some() }
}

pub trait Writer{
    fn write(&mut self, location: &Location, prj: &ProjectStatus) -> Result<()>;
}

pub trait Reader{
    fn read(&self, location: &Location) -> Result<ProjectStatus>;
}

#[derive(Default)]
pub struct ProjectHandler{
    writer: Option<Rc<dyn Writer>>,
    reader: Option<Rc<dyn Reader>>,
}

impl Debug for ProjectHandler{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "ProjectHandler{{ writer: {}, reader: {}, }}", self.writer.is_some(), self.reader.is_some())
    }
}

impl ProjectHandler{
    pub fn new() -> Self{ ProjectHandler{ writer: None, reader: None, } }
    pub fn set_writer(&mut self, writer: Rc<dyn Writer>){ self.writer = Some(writer); }
    pub fn set_reader(&mut self, reader: Rc<dyn Reader>){ self.reader = Some(reader); }

    pub fn write(&mut self, project: &Project) -> Result<()> {
        match &mut self.writer{
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
        match &self.reader {
            Some(s) => {
                project.status = Some(Box::new(s.read(&project.info.location)?));

                Ok(())
            },
            None => return Err(anyhow::anyhow!("there is no project reader")),
        }
    }
}
