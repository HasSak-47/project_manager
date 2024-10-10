use ly::proc::builder;
use serde::{Deserialize, Serialize};


#[builder(name = Tag, pass = derive(Debug, Default, Serialize, Deserialize))]
#[derive(Debug, Default,)]
struct TagTable{
    tag: String,
    #[builder(skip)]
    id: usize,
}
