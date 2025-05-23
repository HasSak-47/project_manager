use std::env::current_dir;

use anyhow::Result;
use clap::Args;
use project_manager_api::FindCriteria;
use super::Arguments;

use crate::SystemHandler;

#[derive(Args, Debug, Clone)]
pub struct MarkFeature {
    feature: String,
}

impl MarkFeature {
    pub fn run(self, _params: Arguments, mut handler: SystemHandler) -> Result<()> {
        let mut path = current_dir()?;
        path.push("status");
        path.set_extension("toml");

        handler.load_projects();
        let project = handler.find_project_mut(&FindCriteria::path(path))?;

        let name = project.get_name().clone();
        project.mark_todo_done(self.feature);
        handler.commit_project(&name)?;

        Ok(())
    }
}
