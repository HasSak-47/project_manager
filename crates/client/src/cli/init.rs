use std::{path::PathBuf, env::current_dir};

use crate::SystemHandler;

use super::{Params, Arguments};
use clap::Args;
use project_manager_api::{
    config::manager::{Manager, ProjectData},
    CachedProject,
};

use anyhow::Result;

#[derive(Args, Debug, Clone)]
pub struct InitStruct{
    name: Option<String>,
    path: Option<PathBuf>,
}

impl InitStruct{
    pub fn run(self, args: Arguments, handler: SystemHandler) -> Result<()> {
        let path = self.path.clone().unwrap_or(current_dir().unwrap());

        Ok(())
    }
}

