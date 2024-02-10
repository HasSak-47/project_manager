use std::{path::PathBuf, env::current_dir, fs::File, io::Write};

use crate::SystemHandler;

use super::{Params, Arguments};
use clap::Args;
use project_manager_api::{
    error::{ProjectResult, ProjectError},
    config::{manager::{Manager, ProjectData, Location}, default::create_project}
};

#[derive(Args, Debug, Clone)]
pub struct NewStruct{
    name: String,
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long, default_value = "0.0.0")]
    version: String,
    #[arg(short, long, default_value = "0.1.1")]
    edition: String,
}

impl NewStruct{
    fn get_location(&self) -> ProjectResult<Location>{
        let mut path = current_dir()?;
        path.push("status");
        path.set_extension("toml");

        if path.exists(){
            return Err(ProjectError::Other("status.toml already exists try using init instead!".to_string()));
        };
        Ok(Location::Path{path})
    }

    fn validate_path(&self) -> bool { true }

    pub fn run(self, args: Arguments, mut handler: SystemHandler) -> ProjectResult<()> {

        if self.validate_path() {
            handler.new_project(self.name.clone(), self.get_location()?)?;
        }

        handler.commit_manager()?;
        handler.commit_projects()?;

        Ok(())
    }
}

