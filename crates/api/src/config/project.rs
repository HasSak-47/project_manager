use std::{fmt::Display, io::{BufRead, BufReader, Read}};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Feature{
    pub name       : String,
    pub priority   : f64,
    pub difficulty : f64,
    pub description: Option<String>,
    pub done: Option<Vec<Feature>>,
    pub todo: Option<Vec<Feature>>,
}

impl Feature{
    pub fn new(name: String, priority: f64, difficulty: f64) -> Self {
        Feature {name, priority, difficulty, ..Default::default()}
    }

    // writes in the formatter the feature and its children 
    // the deep paramenter contains how many spaces to add to the output
    fn format(&self, deep: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        for _ in 0..deep{
            write!(f, " ")?;
        }

        write!(f, "{}: {:.2}\n", self.name, self.difficulty)?;
        if let Some(ref d) = self.done{
            for feat in d{
                feat.format(deep + 1, f)?;
            }
        }

        Ok(())
    }
}

impl Display for Feature{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(0, f)
    }
}


#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}

type OptVec<T> = Option<Vec<T>>;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Project{
    pub project: ProjectInfo,
    pub todo   : OptVec<Feature>,
    pub done   : OptVec<Feature>,
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
            
            todo: Some(Vec::new()),
            done: Some(Vec::new()),
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


    fn _get_act<F>(v: &Option<Vec<Feature>>, sel: F) -> f64
    where 
        F: Fn(&Feature) -> &Option<Vec<Feature>> + Copy,
    {
        if v.is_none(){
            return 0.;
        }
        let mut t = 0.;
        for f in v.as_ref().unwrap(){
            t += f.difficulty;
            t += Self::_get_act(sel(&f), sel);
        }

        t
    }

    pub fn get_todo(&self) -> f64{ Self::_get_act(&self.todo, |feat : &Feature| &feat.todo )}
    pub fn get_done(&self) -> f64{ Self::_get_act(&self.done, |feat : &Feature| &feat.done )}

    fn __add_to(ov : &mut Option<Vec<Feature>>, f : Feature){ 
        let mut v = ov.take().unwrap_or(Vec::new());
        v.push(f);
        *ov = Some(v);
    }

    pub fn add_todo(&mut self, f: Feature){ Self::__add_to(&mut self.todo, f); }
    pub fn add_done(&mut self, f: Feature){ Self::__add_to(&mut self.done, f); }

    pub fn mark_done(&mut self, name: String){
        let i = self.todo.as_ref().and_then(|v| v.iter().position(|f| f.name == name));
        let todo = self.todo
            .as_mut()
            .and_then(|v| i.and_then(|i| Some(v.remove(i))));
        self.done
            .as_mut()
            .and_then(|v| todo.and_then(|t| Some(v.push(t))));
    }

    pub fn get_completion(&self) -> f64{
        let t = self.get_todo();
        let d = self.get_done();

        let total = t + d;
        if total == 0.{ 0. }
        else{ d  / total }
    }
}
