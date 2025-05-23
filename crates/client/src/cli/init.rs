use std::{env::current_dir, fs::File, io::{BufReader, Read}, path::PathBuf};

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

        let mut status_path = path.clone();
        status_path.push("status");
        status_path.set_extension("toml");
        let mut file = BufReader::new(File::open(status_path)?);
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let tree: project_manager_api::trees::ProjectTree = toml::from_str(&buf)?;
        db.build_project_tree();

        let project = Project::new()
            .location(Location::Path(path))
            .desc(Descriptor::new().name(name));

        db.new_project(project)?;
        db.write_data()?;

        Ok(())
    }
}

