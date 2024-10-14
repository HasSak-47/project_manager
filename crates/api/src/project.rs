use super::Location;
use ly::proc::builder;
use serde::{Deserialize, Serialize};
use crate::*;
use crate::desc::{Descriptor, Description};


#[builder(name = Project, pass = derive(Debug, Default, Clone, Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct ProjectTable{
    #[builder(ty = Descriptor)]
    pub(crate) desc: Description,

    #[builder(ty=String)]
    pub(crate) last_worked: Option<Timestamp>,

    #[builder(init = Location::Other)]
    #[builder(pass = serde(default = "Location::default"))]
    pub(crate) location: Location,

    #[builder(skip)]
    pub(crate) id: usize,
    #[builder(ty = String)]
    pub(crate) parent: Option<usize>,
}

use crate::Result;
