use std::{env::current_dir, time::{SystemTime, UNIX_EPOCH}};
use clap::Args;
use anyhow::{anyhow, Result};
use project_manager_api::{desc::Descriptor, task::Task, Database, Location};
use ly::log::prelude::*;

use crate::utils::save_database;

use super::Arguments;


#[derive(Args, Debug, Clone)]
pub struct MarkFeature {
    feature: String,
}

impl MarkFeature {
    pub fn run(self, _params: Arguments, mut db: Database) -> Result<()> {
        let mut cwd = current_dir()?;

        cwd.push("status");
        cwd.set_extension("toml");
        let path = Location::Path(cwd);

        let project_id = db.get_project(|p| p.location == path)?.id();

        let mut task = db.get_task_of_project_mut(project_id, |task| task.desc.name == self.feature)?;
        task.mark_as_done();

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();



        Ok(())
    }
}
