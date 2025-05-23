use ly::proc::builder;
use serde::{Deserialize, Serialize};


#[builder(name = Tag, pass = derive(Debug, Default, Serialize, Deserialize, Clone))]
#[derive(Debug, Default,)]
pub struct TagTable{
    tag: String,
    #[builder(skip)]
    id: usize,
}
