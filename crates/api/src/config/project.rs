use std::fs::File;
use std::io::{Write, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use crate::error::*;
use crate::utils::{optvec_vec, vec_optvec};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FeatureTOML{
    pub name       : String,
    pub priority   : f64,
    pub difficulty : f64,
    pub description: Option<String>,
    pub done: Option<Vec<FeatureTOML>>,
    pub todo: Option<Vec<FeatureTOML>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    pub name       : String,
    pub priority   : f64,
    pub difficulty : f64,
    pub description: Option<String>,
    pub done: Vec<Feature>,
    pub todo: Vec<Feature>,
}

impl Feature{
    #[allow(unused)]
    fn new(name: String, priority: f64, difficulty: f64) -> Self {
        Feature {name, priority, difficulty, ..Default::default()}
    }
}

impl From<FeatureTOML> for Feature{
    fn from(value: FeatureTOML) -> Self {
        Feature {
            name: value.name,
            priority: value.priority,
            difficulty: value.difficulty,
            description: value.description,
            todo: optvec_vec(value.todo),
            done: optvec_vec(value.done),
        }
    }
}

impl From<Feature> for FeatureTOML {
    fn from(value: Feature) -> Self {
        FeatureTOML {
            name: value.name,
            priority: value.priority,
            difficulty: value.difficulty,
            description: value.description,
            todo: vec_optvec(value.todo),
            done: vec_optvec(value.done),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ProjectToml{
    pub project: ProjectInfo,
    pub todo   : Option<Vec<FeatureTOML>>,
    pub done   : Option<Vec<FeatureTOML>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Project{
    pub project: ProjectInfo,
    pub todo   : Vec<Feature>,
    pub done   : Vec<Feature>,
}

impl From<ProjectToml> for Project{
    fn from(value: ProjectToml) -> Self {
        Self {
            project: value.project,
            todo: optvec_vec(value.todo),
            done: optvec_vec(value.done),
        }
    }
}

impl From<Project> for ProjectToml{
    fn from(value: Project) -> Self {
        Self {
            project: value.project,
            todo: vec_optvec(value.todo),
            done: vec_optvec(value.done),
        }
    }
}

#[allow(dead_code)]
impl Project {
    #[allow(unused)]
    pub const fn new() -> Self{
        Project{
            project: ProjectInfo {
                name: String::new(),
                version: String::new(),
                edition: String::new(),
            },
            
            todo: Vec::new(),
            done: Vec::new(),
        }
    }

    fn get_status<P : AsRef<Path>>(path: P) -> PathBuf{
        let mut path = path.as_ref().to_path_buf();
        path.push("status");
        path.set_extension("toml");
        
        path
    }

    pub fn read_project<R: BufRead>(reader: &mut BufReader<R>) -> ProjectResult<Self>{
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(toml::from_str(&buffer)?)
    }

    pub fn write_project<R: BufRead>(reader: &mut BufReader<R>) -> ProjectResult<Self>{
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(toml::from_str(&buffer)?)
    }

    pub fn read_project_from_dir<P : AsRef<Path>>(path: P) -> ProjectResult<Self>{
        let path = Self::get_status(path.as_ref());
        let data = crate::utils::read_file(path)?;

        let project_toml : ProjectToml = toml::from_str(std::str::from_utf8(data.as_bytes())?)?;

        Ok(project_toml.into())
    }

    pub fn write_project_to_dir<P: AsRef<Path>>(&self, path: P) -> ProjectResult<()>{
        let project_toml = ProjectToml::from(self.clone());

        let buffer = toml::to_string(&project_toml)?;
        let path = Self::get_status(path.as_ref());
        let mut file = File::create(path)?;
        file.write(&buffer.as_bytes())?;

        Ok(())
    }

    fn _get_act<F>(v: &Vec<Feature>, sel: F) -> f64
    where 
        F: Fn(&Feature) -> &Vec<Feature> + Copy,
    {
        let mut t = 0.;
        for f in v{
            t += f.difficulty;
            t += Self::_get_act(sel(f), sel);
        }

        t
    }

    pub fn get_todo(&self) -> f64{ Self::_get_act(&self.todo, |feat : &Feature| &feat.todo) }
    pub fn get_done(&self) -> f64{ Self::_get_act(&self.done, |feat : &Feature| &feat.done) }

    pub fn add_todo(&mut self, f: Feature){ self.todo.push(f); }
    pub fn add_done(&mut self, f: Feature){ self.done.push(f); }

    pub fn get_completion(&self) -> f64{
        let t = self.get_todo();
        let d = self.get_done();

        let total = t + d;
        if total == 0.{ 0. }
        else{ d  / total }
    }
}
