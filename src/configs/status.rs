use serde::{Serialize, Deserialize};
use toml::{map::Map, value::Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct Status{
    pub done: Option<Map<String, Value>>,
    pub todo: Option<Map<String, Value>>,
}

fn explote(a: &Option<Map<String, Value>>) -> f64{
        match a{
            Some(v) => { v.iter().fold(0., |a, opt| a + opt.1.as_float().unwrap_or(0.))},
            None => {0.},
        }
}

impl Status{
    pub fn analyze(&self) -> f64{
        let done = explote(&self.done);
        let todo = explote(&self.todo);

        let total = done + todo;
        if total == 0.0{ 
            return f64::NAN;
        }

        done / total
    }
}

pub const DEFAULT_STATUS : &str = 
"[done]
[todo]";
