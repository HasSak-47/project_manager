use std::marker::PhantomData;

use ly::proc::builder;
use serde::{Deserialize, Serialize};


#[builder(name = Tag, pass = derive(Debug, Default, Serialize, Deserialize, Clone))]
#[derive(Debug, Default,)]
pub struct TagTable{
    pub tag: String,
    #[builder(skip)]
    pub id: usize,
}

#[derive(Debug, Default, Clone)]
pub struct TagOtherTable<Other>{
    pub o_id: usize,
    pub tag_id: usize,
    p: PhantomData<Other>,
}

impl<O> TagOtherTable<O>{
    pub fn new(o_id: usize, tag_id: usize) -> Self{Self{
        o_id, tag_id,
        p: PhantomData::default(),
    }}
}
