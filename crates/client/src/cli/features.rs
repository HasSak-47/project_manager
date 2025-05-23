
use std::env::current_dir;

use crate::utils::*;
use clap::Args;

use anyhow::{anyhow, Result};
use project_manager_api::{desc::Descriptor, task::Task, Database, DatabaseError, Location};

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
        let task = Task::new()
            .desc(Descriptor::new()
                .name(self.name)
                .priority(self.priority)
                .difficulty(self.difficulty)
            ).project(project);

        db.add_full_task(task)?;
        db.write_data()?;
        Ok(())
    }
}



