
use std::process;

use crate::SystemHandler;

use super::{utils::find_local_status, Arguments};
use clap::Args;
use anyhow::Result;
use project_manager_api::FindCriteria;

#[derive(Args, Debug, Default, Clone)]
#[clap(about = include_str!("abouts/GitStruct.txt").trim_end())]
pub struct GitStruct{
    #[clap(allow_hyphen_values=true)]
    args: Vec<String>,
}

impl GitStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()>{
        let status = find_local_status()?;
        let current_project = handler.find_project_mut(&FindCriteria::path(status))?;

        let mut child = process::Command::new("git").args(self.args).spawn()?;

        child.wait()?;
        current_project.update()?;
        handler.commit_manager()?;
        Ok(())
    }
}


