use std::env::current_dir;

use crate::SystemHandler;

use super::Arguments;
use clap::Args;
use project_manager_api::config::project::Feature;
use anyhow::Result;

#[derive(Args, Debug, Clone)]
pub struct AddFeat{
    name: String,
    priority: f64,
    difficulty: f64,

    #[clap(short, default_value = "todo")]
    r#type : String,
}

impl AddFeat{
    pub fn run(self, _params: Arguments, handler: SystemHandler) -> Result<()> {
        // let f = Feature::new(self.name, self.priority, self.difficulty);
        let feat = Feature::new(self.name.clone(), self.priority, self.difficulty);

        let path = current_dir()?;

        /*
        if self.r#type == "done"{
            project.add_done(feat);
        }
        else{
            project.add_todo(feat);
        }

        project.write_project_to_dir(path)?;
        */

        Ok(())
    }
}



