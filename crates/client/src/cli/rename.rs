use std::path::PathBuf;

use crate::SystemHandler;

use super::{utils::find_local_status, Arguments};
use clap::Args;
use project_manager_api::config::manager::Location;
use anyhow::{Result, anyhow};

use crate::VERSION;

#[derive(Args, Debug, Clone)]
pub struct RenameStruct{
    old_name: String,
    new_name: String,
}

impl RenameStruct{
    pub fn run(self, _args: Arguments, mut handler: SystemHandler) -> Result<()> {
        Ok(())
    }
}

