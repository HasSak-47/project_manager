use std::env::current_dir;
use clap::Args;
use anyhow::{anyhow, Result};
use project_manager_api::{desc::Descriptor, task::Task, Database, Location};
use ly::log::prelude::*;

use crate::utils::save_database;

use super::Arguments;

#[derive(Args, Debug, Clone)]
pub struct AddFeat{
    name: String,
    priority: f64,
    difficulty: f64,

    #[clap(short, default_value = "false")]
    done: bool,

    #[clap(long)]
    orphan: bool,
}

impl AddFeat{
    pub fn run(self, _params: Arguments, mut db: Database) -> Result<()> {
        let mut cwd = current_dir()?;

        cwd.push("status");
        cwd.set_extension("toml");
        let cwd = Location::Path(cwd);
        let project = if !self.orphan{
            db.get_all_projects()
                .iter()
                .find(|p| *p.location() == cwd)
                .ok_or(anyhow!("Project not found in database"))
                .and_then(|p| Ok(p.name()))?
                .clone()
        }else{
             "".to_string()
        };

        let task = Task::new()
            .desc(Descriptor::new()
                .name(self.name)
                .priority(self.priority)
                .difficulty(self.difficulty)
            ).project(project);

        log!("task added: {task:?}");

        db.add_full_task(task)?;
        save_database(&db)?;
        Ok(())
    }
}



