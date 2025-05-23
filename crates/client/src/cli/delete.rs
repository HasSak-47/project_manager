use clap::Args;
use anyhow::Result;
use project_manager_api::Handler;

use super::Arguments;

#[derive(Args, Debug, Clone)]
#[clap(about = include_str!("abouts/DelStruct.txt").trim_end())]
pub struct DelStruct{
    name: String,
    #[clap(long, default_value="false")]
    delete_status: bool,
}

impl DelStruct{
    pub fn run(self, _args: Arguments, mut handler: Handler) -> Result<()> {
        handler.load_projects()?;
        handler.projects.remove(&self.name);
        handler.commit_manager()?;
        Ok(())
    }
}

