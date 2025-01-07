use std::{env::current_dir, fs::File, io::Write, path::PathBuf};

use super::Arguments;
use clap::Args;

use anyhow::Result;
use thiserror::Error;
use project_manager_api::{desc::Descriptor, project::Project, Database, Location};

use crate::{utils::{exists_name, exists_path, load_database, save_database}, VERSION};
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

#[derive(Debug, Error)]
enum NewError{
    #[error("Project already exist {0}")]
    NameAlreadyUsed(String),
    #[error("Path already used {0} try using init instead")]
    PathAlreadyUsed(String)
}

impl NewStruct{
    pub fn run(self, _args: Arguments, mut db: Database) -> Result<()> {

        let mut path = self.path.unwrap_or(current_dir().unwrap());
        log!("creating project at {}", path.display());

        path.push("status");
        path.set_extension("toml");

        log!("with status at {}", path.display());

        load_database(&mut db)?;

        if exists_path(&db, &path){
            Err( NewError::PathAlreadyUsed(path.clone().to_str().unwrap().to_string()) )?;
        }

        if exists_name(&db, &self.name){
            Err( NewError::NameAlreadyUsed(path.clone().to_str().unwrap().to_string()) )?;
        }


        let p = Project::new()
            .desc(Descriptor::new()
                  .name(self.name)
                  .version(self.version)
                  .edition(self.edition)
            )
            .location(Location::Path(path.clone()));
        debug!("created project {p:?}");

        let mut file = File::create(&path)?;
        log!("created status file");
        let s = toml::to_string(&p).unwrap();

        file.write(s.as_bytes())?;

        db.add_full_project(p)?;
        save_database(&db)?;
        Ok(())
    }
}

