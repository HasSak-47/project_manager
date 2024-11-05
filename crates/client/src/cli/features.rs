use std::env::current_dir;
use clap::Args;
use anyhow::{anyhow, Result};
use project_manager_api::{desc::Descriptor, task::Task, Database, Location};
use ly::log::prelude::*;

use super::Arguments;

#[derive(Args, Debug, Clone)]
pub struct AddFeat{
    name: String,
    priority: f64,
    difficulty: f64,

    #[clap(short, default_value = "todo")]
    r#type : String,

    #[clap(long)]
    orphan: bool,
}

impl AddFeat{
    pub fn run(self, _params: Arguments, mut db: Database) -> Result<()> {
        let mut cwd = current_dir()?;
        db.load_data()?;

        cwd.push("status");
        cwd.set_extension("toml");
        let cwd = Location::Path(cwd);
        let project = if !self.orphan{
            db.get_all_projects()
                .iter()
                .find(|p| *p.location() == cwd)
                .ok_or(anyhow!("project not found"))
                .and_then(|p| Ok(p.name()))?
                .clone()
        }else{
             "".to_string()
        };

        let _ = log!("task project: {project}");
        let task = Task::new()
            .desc(Descriptor::new()
                .name(self.name)
                .priority(self.priority)
                .difficulty(self.difficulty)
            ).project(project);

        let task_id = db.add_full_task(task)?;
        let _ = log!("task id: {task_id}");
        let task_manager = db.search_task(|p| p.id == task_id)?;
        let task = task_manager.get_table();
        let _ = log!("task table: {task:#?}");

        let _ = log!("database before calling write: {db:#?}");
        db.write_data()?;
        Ok(())
    }
}



