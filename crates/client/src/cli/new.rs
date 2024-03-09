use std::{path::PathBuf, env::current_dir};

use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use project_manager_api::config::manager::Location;
use anyhow::{Result, anyhow};

use crate::VERSION;

#[derive(Args, Debug, Clone)]
#[clap(about = include_str!("abouts/NewStruct.txt").trim_end())]
pub struct NewStruct{
    name: String,
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long, default_value = "0.0.0")]
    version: String,
    #[arg(short, long, default_value = VERSION)]
    edition: String,
}

impl NewStruct{
    fn get_location(&self) -> Result<Location>{
        let mut path = current_dir()?;
        path.push("status");
        path.set_extension("toml");

        if path.exists(){
            return Err(anyhow!("status.toml already exists try using init instead!"));
        }
        Ok(Location::Path{path})
    }

    fn validate_path(&self) -> bool { true }

    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()> {

        if self.validate_path() {
            handler.new_project(self.name.clone(), self.get_location()?)?;
        }

        handler.commit_manager()?;
        handler.commit_projects()?;

        Ok(())
    }
}

