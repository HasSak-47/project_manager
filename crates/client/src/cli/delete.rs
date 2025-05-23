use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use anyhow::Result;

#[derive(Args, Debug, Clone)]
#[clap(about = include_str!("abouts/DelStruct.txt").trim_end())]
pub struct DelStruct{
    name: String,
    #[clap(long, default_value="false")]
    delete_status: bool,
}

impl DelStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()> {
        handler.remove_project(self.name)?;
        handler.commit_manager()?;
        Ok(())
    }
}

