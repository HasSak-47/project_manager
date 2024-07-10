
use std::{env::current_dir, os::unix::process::CommandExt, path::PathBuf, process::{Child, Command}, time::{SystemTime, UNIX_EPOCH}};

use anyhow::Result;
use project_manager_api::{manager::Manager, project::{ProjectInfo, ProjectStatus}, CachedProject, Handler, Location};
use super::Arguments;
use clap::Args;


#[derive(Args, Debug, Default, Clone)]
#[clap(about = include_str!("abouts/GitStruct.txt").trim_end())]
pub struct GitStruct{
    #[clap(allow_hyphen_values=true)]
    args: Vec<String>,
}

impl GitStruct{
    pub fn run(self, _args: Arguments, mut handler: Handler) -> Result<()>{
        let cwd = Location::Path( current_dir().unwrap() );
        let project = handler.get_project_mut(cwd)?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        project.info.last_update = Some(now.as_secs() as usize);

        Command::new("git").args(self.args).spawn()?.wait()?;

        Ok(())
    }
}


