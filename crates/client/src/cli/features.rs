use libc::STDOUT_FILENO;
use project_manager_api::{project::{Feature, ProjectInfo}, CachedProject, Handler, Location};
use super::{utils::current_project_mut, Arguments};
use clap::{Subcommand, Args, ValueEnum};

use anyhow::{anyhow, Result};

#[derive(Args, Debug, Clone)]
pub struct AddFeat{
    name: String,
    priority: u8,
    difficulty: u8,

    #[clap(short, default_value = "todo")]
    r#type : String,
}

impl AddFeat{
    pub fn run(self, _params: Arguments, mut handler: Handler) -> Result<()> {
        let feature = Feature::new(self.name, "".to_string(), self.priority, self.difficulty);
        let status = current_project_mut(&mut handler)?.status.as_mut().unwrap();
        if self.r#type == "todo"{
            status.add_todo(feature);
        }
        else{
            status.add_done(feature);
        }
        
        Ok(())
    }
}



