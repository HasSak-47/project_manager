use crate::SystemHandler;

use super::{utils::find_local_status, Arguments};
use clap::Args;
use project_manager_api::{
    config::project::Feature,
    FindCriteria
};
use anyhow::{anyhow, Result};

#[derive(Args, Debug, Clone)]
pub struct AddFeat{
    name: String,
    priority: f64,
    difficulty: f64,

    #[clap(short, default_value = "todo")]
    r#type : String,
}

impl AddFeat{
    pub fn run(self, _params: Arguments, mut handler: SystemHandler) -> Result<()> {
        let feat = Feature::new(self.name.clone(), self.priority, self.difficulty);
        let path = find_local_status()?;


        handler.load_projects();
        let project = handler.find_project_mut(&FindCriteria::path(path))?;
        let name = project.get_name().clone();
        if _params.debug{
            if project.broken().unwrap_or( true ) {
                return Err(anyhow!("Project is broken!"));
            }
            println!("Adding feature {feat:?} to project: {}", name);
        }


        if self.r#type == "done"{
            project.add_done(feat);
        }
        else{
            project.add_todo(feat);
        }
        project.update()?;
        handler.commit_project(&name)?;

        Ok(())
    }
}



