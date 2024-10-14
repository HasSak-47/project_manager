const VERSION: &str = env!("CARGO_PKG_VERSION");

use cli::cli;
use anyhow::Result;
use ly::log::write::ANSI;
use project_manager_api::{
    project::*, tags::*, task::*, DatabaseBuilder, DatabaseReader, DatabaseWriter, Result as DBResult
};

mod cli;

struct LoaderWriter{
}

impl DatabaseReader for LoaderWriter{
    fn read_all_projects(&self) -> DBResult<Vec<ProjectTable>> { Ok(Vec::new()) }
    fn read_all_tasks(&self) -> DBResult<Vec<TaskTable>> { Ok(Vec::new()) }
    fn read_all_tags(&self) -> DBResult<Vec<TagTable>> { Ok(Vec::new()) }
}
impl DatabaseWriter for LoaderWriter{
    fn write_all_projects(&mut self, _: &mut Vec<ProjectTable>) -> DBResult<()> { Ok(()) }
    fn write_all_tasks(&mut self, _: &mut Vec<TaskTable>) -> DBResult<()> { Ok(()) }
    fn write_all_tags(&mut self, _: &mut Vec<TagTable>) -> DBResult<()> { Ok(()) }
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
