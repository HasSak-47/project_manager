use serde::{Serialize, Deserialize};
use toml::{map::Map, value::Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config{
    pub projects: Option<Map<String, Value>>,
    pub force: Force,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Force{
    pub commit_message: String,
    pub push_message: String,
}

pub const DEFAULT_CONFIG: &str =
"[projects]
[force]
commit_message= \"Forced commit\"
push_message= \"Forced push\"";

