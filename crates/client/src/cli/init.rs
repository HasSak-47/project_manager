use std::{path::PathBuf, env::current_dir};

use super::Arguments;
use clap::Args;

use anyhow::Result;
use project_manager_api::{project::{Project, ProjectStatus}, Handler};

/**
The project already has a status.toml in it's path
it just adds it to the tracker
 */
#[derive(Args, Debug, Clone)]
#[clap(about = include_str!("abouts/InitStruct.txt").trim_end())]
pub struct InitStruct{
    name: Option<String>,
    path: Option<PathBuf>,
}

impl InitStruct{
    pub fn run(self, _args: Arguments, mut handler: Handler) -> Result<()> {
        let path = self.path.unwrap_or(current_dir()?);
        let name = self.name.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());

        let mut status_path = path.clone();
        status_path.push("status");
        status_path.set_extension("toml");
        let mut project = Project::default();
        project.info.name = name.clone();
        project.status = Some(Box::new(
            ProjectStatus::new(name.clone(), String::new())
        ));

        handler.init_project(project)?;
        handler.commit_project(name)?;

        Ok(())
    }
}

