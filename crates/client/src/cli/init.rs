use std::{path::PathBuf, env::current_dir};

use super::Arguments;
use clap::Args;

use anyhow::Result;
use project_manager_api::{desc::Descriptor, project::Project, Database, Location};

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
    pub fn run(self, _args: Arguments, mut db: Database) -> Result<()> {
        let path = self.path.unwrap_or(current_dir().unwrap());
        let name = self.name.unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string());

        let project = Project::new()
            .location(Location::Path(path))
            .desc(Descriptor::new().name(name));

        db.new_project(project)?;

        Ok(())
    }
}

