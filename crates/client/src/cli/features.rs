
use crate::utils::*;
use clap::Args;

use anyhow::{anyhow, Result};
use project_manager_api::{desc::Descriptor, task::Task, Database};

use super::Arguments;

#[derive(Args, Debug, Clone)]
pub struct AddFeat{
    name: String,
    priority: f64,
    difficulty: f64,

    #[clap(short, default_value = "todo")]
    r#type : String,
}

impl AddFeat{
    pub fn run(self, _params: Arguments, mut db: Database) -> Result<()> {
        let projects = db.get_all_projects();
        let project = projects.iter()
            .find(|p| p.get_table().desc.name == self.name)
            .ok_or(anyhow!("could not find project"))?
            .get_table();

        let task = Task::new()
            .desc(Descriptor::new()
                .name(self.name)
                .priority(self.priority)
                .difficulty(self.difficulty)
            ).project(project.desc.name.clone());

        Ok(())
    }
}



