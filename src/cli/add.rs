use std::{path::PathBuf, env::current_dir};

use super::RunCmd;
use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::manager::{Manager, ProjectData}};

// this looks like shit
#[derive(Args, Debug, Clone)]
pub struct AddStruct{
    name: Option<String>,
    path: Option<PathBuf>,
}

impl RunCmd for AddStruct{
    fn run(&self) -> ProjectResult<()> {
        let mut manager = Manager::load_data_from(Manager::get_path()?)?;
        let cwd = current_dir().unwrap();
        let f_name = cwd.file_name().unwrap().to_str().unwrap().to_string();
        manager.projects.push(ProjectData{
            name: self.name.unwrap_or(f_name),
            path: self.path.unwrap_or(cwd),
            ignore: None,
            subprojects: None,
        });
        Ok(())
    }
}

