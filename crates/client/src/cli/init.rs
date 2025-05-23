use std::{path::PathBuf, env::current_dir};

use super::{Params};
use clap::Args;
use project_manager_api::{
    error::{ProjectResult, ProjectError},
    config::manager::{Manager, ProjectData}
};

#[derive(Args, Debug, Clone)]
pub struct InitStruct{
    name: Option<String>,
    path: Option<PathBuf>,
}

impl InitStruct{
    pub fn run(&self, params: Params) -> ProjectResult<()> {
        let mut manager = Manager::load_data_from(&params.manager_path)?;
        let cwd = current_dir().unwrap();
        let f_name = cwd.file_name().unwrap().to_str().unwrap().to_string();
        for p in &manager.projects{
            if cwd == p.path || f_name == p.name{
                return Err(ProjectError::Other("Name or Path already exists!!".to_string()));
            }
        }

        manager.projects.push(ProjectData{
            name: self.name.clone().unwrap_or(f_name),
            path: self.path.clone().unwrap_or(cwd),
            ..Default::default()
        });
        
        manager.write_data_to(&params.manager_path)?;


        Ok(())
    }
}

