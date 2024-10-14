use std::{path::PathBuf, env::current_dir};

use super::Arguments;
use clap::Args;

use anyhow::Result;
use project_manager_api::{desc::Descriptor, project::Project, Database, Location};

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
    pub fn run(self, _args: Arguments, mut db: Database) -> Result<()> {
        let path = self.path.unwrap_or(current_dir().unwrap());
        let p = Project::new()
            .desc(Descriptor::new()
                  .name(self.name)
                  .version(self.version))
            .location(Location::Path(path));


        db.new_project(p)?;

        Ok(())
    }
}

