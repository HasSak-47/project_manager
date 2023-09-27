use serde::{Deserialize, Serialize};
use toml::{ Value, map::Map};

pub struct Feature{
    name: String,
    difficulty: f32,
    priority: f32,
    sub_feature: Vec<Feature>,
}

pub struct Project{
    name : String,
    features: Vec<Feature>,
    todo: Vec<usize>,
    done: Vec<usize>,
}
