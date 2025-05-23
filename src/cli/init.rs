use std::{path::PathBuf, env::current_dir};

use super::{RunCmd, Params};
use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::manager::{Manager, ProjectData, self}};

// this looks like shit
#[derive(Args, Debug, Clone)]
pub struct InitStruct{
    name: Option<String>,
    path: Option<PathBuf>,
}

impl RunCmd for InitStruct{
    fn run(&self, params: Params) -> ProjectResult<()> {
        let man_path = Manager::get_path()?;
        let mut manager = Manager::load_data_from(&man_path)?;
        let cwd = current_dir().unwrap();
        let f_name = cwd.file_name().unwrap().to_str().unwrap().to_string();
        for p in &manager.projects{
            if cwd == p.path || f_name == p.name{
                return Err(crate::error::ProjectError::Other("Name or Path already exists!!".to_string()));
            }
        }

        manager.projects.push(ProjectData{
            name: self.name.clone().unwrap_or(f_name),
            path: self.path.clone().unwrap_or(cwd),
            ignore: None,
            subprojects: None,
        });
        
        manager.write_data_to(man_path)?;


        Ok(())
    }
}

