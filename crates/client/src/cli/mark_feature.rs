use std::{env::current_dir, time::{SystemTime, UNIX_EPOCH}};

use anyhow::Result;
use clap::Args;
use project_manager_api::{Handler, Location};

use super::Arguments;


#[derive(Args, Debug, Clone)]
pub struct MarkFeature {
    feature: String,
}

impl MarkFeature {
    pub fn run(self, _args: Arguments, mut handler: Handler) -> Result<()> {
        let cwd = Location::Path( current_dir().unwrap() );

        handler.load_projects();
        let project = handler.get_project(cwd)?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        project.info.last_update = Some(now.as_secs() as usize);

        handler.commit_project(project.info.name);
        handler.commit_manager();

        Ok(())
    }
}
