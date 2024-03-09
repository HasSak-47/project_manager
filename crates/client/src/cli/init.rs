use std::{path::PathBuf, env::current_dir};

use crate::SystemHandler;

use super::Arguments;
use clap::Args;

use anyhow::Result;
use project_manager_api::config::manager::Location;

#[derive(Args, Debug, Clone)]
#[clap(about = include_str!("abouts/InitStruct.txt").trim_end())]
pub struct InitStruct{
    name: Option<String>,
    path: Option<PathBuf>,
}

impl InitStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()> {
        let mut path = self.path.unwrap_or(current_dir()?);
        let name = self.name.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());
        path.push("status");
        path.set_extension("toml");

        handler.add_project(name, Location::path(path));
        handler.commit_manager()?;
        Ok(())
    }
}

