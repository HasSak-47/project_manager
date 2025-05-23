use std::{path::PathBuf, env::current_dir, fs::File, io::Write};

use super::{RunCmd, Params};
use clap::Args;
use crate::{error::ProjectResult, config::{manager::{Manager, ProjectData}, default::create_project}};

#[derive(Args, Debug, Clone)]
pub struct NewStruct{
    name: String,
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long, default_value = "0.0.0")]
    version: String,
    #[arg(short, long, default_value = "0.1.1")]
    edition: String,
}

impl RunCmd for NewStruct{
    fn run(&self, params: Params) -> ProjectResult<()> {
        let mut manager = Manager::load_data_from(&params.manager_path)?;
        let mut path = current_dir()?;

        path.push("status");
        path.set_extension("toml");
        if path.exists(){
            return Err(crate::error::ProjectError::Other("project already exists!".to_string()));
        }

        let mut file = File::create(path)?;
        file.write(&create_project(&self.name, &self.version, &self.edition).as_bytes())?;

        manager.projects.push(ProjectData{
            name: self.name.clone(),
            path: self.path.clone().unwrap_or(current_dir()?.clone()),
            ignore: None,
            subprojects: None,
        });

        manager.write_data_to(&params.manager_path)?;
        Ok(())
    }
}

