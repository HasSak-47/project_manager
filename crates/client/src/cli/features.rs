

use super::{Params};
use clap::Args;
use project_manager_api::{
    error::{ProjectResult}
};

#[derive(Args, Debug, Clone)]
pub struct InitStruct{
    name: String,
    priority: f64,
    difficulty: f64,
}

impl InitStruct{
    pub fn run(&self, _params: Params) -> ProjectResult<()> {
        Ok(())
    }
}



