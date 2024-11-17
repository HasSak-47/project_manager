use std::{env::current_dir, fs::File, io::{BufReader, Read}, path::PathBuf};

use super::Arguments;
use clap::Args;

use anyhow::{anyhow, Result};
use project_manager_api::{project::Project, Database, Location};

use crate::
    utils::{exists_path, load_database, save_database};

use ly::log::prelude::*;

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
        let _ = log!("path of the new project: {}", path.display());

        let mut status_path = path.clone();
        status_path.push("status");
        status_path.set_extension("toml");

        let _ = log!("path of the status file : {}", status_path.display());
        let mut file = BufReader::new(File::open(&status_path)?);
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let proj = toml::from_str::<Project>(&buf)?
            .location(Location::Path(status_path.clone()));

        load_database(&mut db)?;
        if exists_path(&db, &status_path) {
            return Err(anyhow!("Project path ({}) already exists", status_path.display()));
        }
        db.add_full_project(proj)?;
        save_database(&db)?;

        Ok(())
    }
}

