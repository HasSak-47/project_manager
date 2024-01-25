use std::{path::PathBuf, env::current_dir};

use super::RunCmd;
use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::manager::{Manager, ProjectData, self}};

// this looks like shit
#[derive(Args, Debug, Clone)]
pub struct DelStruct{
    name: Option<String>,
}

impl RunCmd for DelStruct{
    fn run(&self) -> ProjectResult<()> {
        if self.name.is_none(){
            return Err(crate::error::ProjectError::Other("proj not found".to_string()));
        }
        let man_path = Manager::get_path()?;
        let mut manager = Manager::load_data_from(&man_path)?;
        let name = self.name.clone().unwrap();
        let delete = manager.projects.iter().enumerate().find(|p| p.1.name == name);
        if delete.is_none(){
            println!("project not found!");
        }

        let delete = delete.unwrap();
        manager.projects.remove(delete.0);

        manager.write_data_to(man_path)?;
        Ok(())
    }
}

