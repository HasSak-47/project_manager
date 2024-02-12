use std::io::{BufRead, BufReader, Read};
use serde::{Deserialize, Serialize};
use anyhow::Result;

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
    pub fn new(name: String, priority: f64, difficulty: f64) -> Self {
        Feature {name, priority, difficulty, ..Default::default()}
    }
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Project{
    pub project: ProjectInfo,
    pub todo   : Vec<Feature>,
    pub done   : Vec<Feature>,
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

    pub fn read_project<R: BufRead>(reader: &mut BufReader<R>) -> Result<Self>{
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(toml::from_str(&buffer)?)
    }

    pub fn write_project<R: BufRead>(reader: &mut BufReader<R>) -> Result<Self>{
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(toml::from_str(&buffer)?)
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
