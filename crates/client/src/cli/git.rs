
use std::env::current_dir;

use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use anyhow::Result;
use project_manager_api::FindCriteria;

#[derive(Args, Debug, Default, Clone)]
pub struct GitStruct{
    #[clap(allow_hyphen_values=true)]
    args: Vec<String>,
}

impl GitStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()>{
        let cwd = current_dir()?;
        let current_project = handler.find_project_mut(&FindCriteria::path(cwd))?;
        current_project.update();
        handler.commit_manager()?;
        Ok(())
    }
}


