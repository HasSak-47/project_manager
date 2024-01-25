use std::{path::PathBuf, env::current_dir};

use super::{RunCmd, Params};
use clap::{Subcommand, Parser, Args};
use crate::{error::ProjectResult, config::manager::{Manager, ProjectData, self}};

// this looks like shit
#[derive(Args, Debug, Clone)]
pub struct DelStruct{
    name: String,
}


impl RunCmd for DelStruct{
    fn run(&self, params: Params) -> ProjectResult<()> {
        let mut manager = Manager::load_data_from(&params.manager_path)?;
        let name = self.name.clone();
        let delete = manager.projects.iter().enumerate().find(|p| p.1.name == name);
        if delete.is_none(){
            println!("project not found!");
        }

        let delete = delete.unwrap();
        manager.projects.remove(delete.0);

        manager.write_data_to(&params.manager_path)?;
        Ok(())
    }
}

