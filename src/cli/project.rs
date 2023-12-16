use super::CliUtil;
use crate::error::ProjectResult;
use crate::config::project::Project;

pub struct ManageProject{
    project: Project,
}

impl ManageProject {
    pub fn load_project<S: AsRef<str>>(folder: S) -> ProjectResult<()> {
        let folder = folder.as_ref();
        Ok(())
    }
}

