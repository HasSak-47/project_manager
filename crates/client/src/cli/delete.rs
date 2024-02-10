use std::env::current_dir;

use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use project_manager_api::error::{ProjectError, ProjectResult};

#[derive(Args, Debug, Clone)]
pub struct DelStruct{
    name: String,
    #[clap(long, default_value="false")]
    delete_status: bool,
}

impl DelStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> ProjectResult<()> {
        let project = match handler.get_project(self.name){
            Some(s) => s,
            None => {return Err(ProjectError::ProjectNotFound { name: Some(self.name), path: None })},
        };
        if self.delete_status{
        }
        Ok(())
    }
}

