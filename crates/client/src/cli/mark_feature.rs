use std::env::current_dir;
use clap::Args;
use anyhow::Result;
use project_manager_api::{Database, Location};
// use ly::log::prelude::*;

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

        let mut project = db.get_project_mut(|p| p.location == path)?;

        let now = chrono::Local::now();
        project.update(now.date_naive())?;

        let project_id = project.as_ref()?.id();

        let mut task = db.get_task_of_project_mut(project_id, |task| task.desc.name == self.feature)?;
        task.mark_as_done();

        save_database(&db)?;


        Ok(())
    }
}
