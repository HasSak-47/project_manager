
use std::env::current_dir;

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
        let man = Manager::load_data_from(Manager::get_path()?)?;
        let p = man.find_project_path(current_dir()?);
        for arg in &self.args{
            println!("{arg}");
        }
        Ok(())
    }
}


