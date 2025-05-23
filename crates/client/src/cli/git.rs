
use std::{env::current_dir, process};

use super::Params;
use clap::Args;
use project_manager_api::{error::ProjectResult, config::manager::Manager};

#[derive(Args, Debug, Default, Clone)]
pub struct GitStruct{
    #[clap(allow_hyphen_values=true)]
    args: Vec<String>,
}

impl GitStruct{
    pub fn run(&self, _params: Params) -> ProjectResult<()>{
        let man_path = Manager::get_path()?;
        let mut man = Manager::load_data_from(&man_path)?;
        let cwd = current_dir()?;
        let current_project = man.find_project(|p| p.path == cwd)?.name.clone();
        man.update_project(current_project)?;
        man.write_data_to(man_path)?;
        let _child = process::Command::new("git")
            .args(self.args.as_slice())
            .spawn().unwrap()
            .wait();
        Ok(())
    }
}


