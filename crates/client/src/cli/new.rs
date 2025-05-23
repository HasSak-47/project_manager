use std::{env::current_dir, fs::File, io::Write, path::PathBuf};

use super::Arguments;
use clap::Args;

use anyhow::Result;
use project_manager_api::{desc::Descriptor, project::Project, Database, Location};

use crate::VERSION;
use ly::log::prelude::*;

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

        let _ = log!("creating project at {}", path.display());

        let mut status_path = path.clone();
        status_path.push("status");
        status_path.set_extension("toml");

        let _ = log!("with status at {}", status_path.display());


        let p = Project::new()
            .desc(Descriptor::new()
                  .name(self.name)
                  .version(self.version)
                  .edition(self.edition)
            )
            .location(Location::Path(path));
        let _ = log!("created project {p:?}");

        let mut file = File::create(&status_path)?;
        let _ = log!("created status file");
        let s = toml::to_string(&p).unwrap();
        let _ = log!("parsed project data");

        file.write(s.as_bytes())?;

        db.add_full_project(p)?;
        db.write_data()?;



        Ok(())
    }
}

