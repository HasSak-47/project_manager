use std::{path::PathBuf, env::current_dir};

use super::Arguments;
use clap::Args;

use anyhow::Result;
use project_manager_api::{project::{Project, ProjectStatus}, Handler, Location};

use crate::VERSION;

#[derive(Args, Debug, Clone)]
#[clap(about = include_str!("abouts/NewStruct.txt").trim_end())]
pub struct NewStruct{
    name: String,
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long, default_value = "0.1.0")]
    version: String,
    #[arg(short, long, default_value = VERSION)]
    edition: String,
}

impl NewStruct{
    pub fn run(self, _args: Arguments, mut handler: Handler) -> Result<()> {
        let name = self.name;
        let path = self.path.unwrap_or(current_dir().unwrap());

        let mut status_path = path.clone();
        status_path.push("status");
        status_path.set_extension("toml");

        let mut project = Project::default();
        project.info.name = name.clone();
        project.info.location = Location::Path(path);
        project.status = Some(Box::new(
            ProjectStatus::new(name.clone(), String::new())
        ));

        handler.new_project(project).unwrap();
        handler.commit_project(name).unwrap();
        handler.commit_manager().unwrap();

        Ok(())
    }
}

