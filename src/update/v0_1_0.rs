use serde::{Deserialize, Serialize};
use toml::{Table, value::Array};

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct ProjectInfo{
    pub name: String,
    pub version: String,
    pub edition: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectToml{
    project : ProjectInfo,
    subproj : Option<Array>,
    todo    : Option<Array>, 
    done    : Option<Array>, 
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FeatureV0_1_0{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub done: Vec<FeatureV0_1_0>,
    pub todo: Vec<FeatureV0_1_0>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FeatureV0_0_0{
    pub name       : String,
    pub priority   : f32,
    pub difficulty : f32,
    pub description: Option<String>,
    pub subfeature : Vec<FeatureV0_0_0>,
}
