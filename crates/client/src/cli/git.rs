
use std::env::current_dir;

use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use project_manager_api::error::ProjectResult;

#[derive(Args, Debug, Default, Clone)]
pub struct GitStruct{
    #[clap(allow_hyphen_values=true)]
    args: Vec<String>,
}

impl GitStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> ProjectResult<()>{
        let cwd = current_dir()?;
        let _current_project = handler.find_via_path(cwd);
        handler.commit_manager()?;
        Ok(())
    }
}


