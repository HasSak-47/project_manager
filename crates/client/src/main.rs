const VERSION: &str = env!("CARGO_PKG_VERSION");

use cli::cli;
use anyhow::Result;
use ly::log::write::ANSI;
use project_manager_api::{
    project::*, tags::*, task::*, DatabaseBuilder, DatabaseError, DatabaseReader, DatabaseWriter, Result as DBResult
};

mod cli;

struct LoaderWriter{}

impl DatabaseReader for LoaderWriter{
    fn read_all_projects(&self) -> DBResult<Vec<ProjectTable>> { Err(DatabaseError::NotImplemented) }
    fn read_all_tasks(&self) -> DBResult<Vec<TaskTable>> { Err(DatabaseError::NotImplemented) }
    fn read_all_tags(&self) -> DBResult<Vec<TagTable>> { Err(DatabaseError::NotImplemented) }

    fn read_project(&self) -> DBResult<ProjectTable> { Err(DatabaseError::NotImplemented) }
    fn read_task(&self) -> DBResult<TaskTable> { Err(DatabaseError::NotImplemented) }
    fn read_tag(&self) -> DBResult<TagTable> { Err(DatabaseError::NotImplemented) }
}
impl DatabaseWriter for LoaderWriter{
    fn write_all_projects(&self) -> DBResult<Vec<ProjectTable>> { Err(DatabaseError::NotImplemented) }
    fn write_all_tasks(&self) -> DBResult<Vec<TaskTable>> { Err(DatabaseError::NotImplemented) }
    fn write_all_tags(&self) -> DBResult<Vec<TagTable>> { Err(DatabaseError::NotImplemented) }

    fn write_project(&mut self) -> DBResult<()> { Err(DatabaseError::NotImplemented) }
    fn write_task(&mut self) -> DBResult<()> { Err(DatabaseError::NotImplemented) }
    fn write_tag(&mut self) -> DBResult<()> { Err(DatabaseError::NotImplemented) }
}

fn main() -> Result<()>{

    ly::log::set_logger(ANSI::new());
    ly::log::set_level(ly::log::Level::Error);

    let db = DatabaseBuilder::new()
        .set_writer(LoaderWriter{})
        .set_reader(LoaderWriter{})
        .build();
    cli(db)?;
    Ok(())
}
