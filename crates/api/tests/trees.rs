use ly::log::{self, write::ANSI};
use ly::macro_error;
use ly::macro_log;
use pm_api::project::Project;
use project_manager_api::{Database, DatabaseBuilder, DatabaseReader, DatabaseWriter};
use serde;
use anyhow::Result;

const TEST_TREE: &str = include_str!("./test_tree.json");
use ly::log::prelude::*;

struct ReaderWriter{ }
use project_manager_api as pm_api;

impl DatabaseReader for ReaderWriter {
    fn read_all_tags(&self) -> pm_api::Result<Vec<pm_api::tags::TagTable>> { Ok(Vec::new()) }
    fn read_all_tasks(&self) -> pm_api::Result<Vec<pm_api::task::TaskTable>> { Ok(Vec::new()) }
    fn read_all_projects(&self) -> pm_api::Result<Vec<pm_api::project::ProjectTable>> { Ok(Vec::new()) }
}

impl DatabaseWriter for ReaderWriter {
    fn write_all_tags(&mut self, v: &mut Vec<pm_api::tags::TagTable>) -> pm_api::Result<()> { Ok(()) }
    fn write_all_tasks(&mut self, v: &mut Vec<pm_api::task::TaskTable>) -> pm_api::Result<()> { Ok(()) }
    fn write_all_projects(&mut self, v: &mut Vec<pm_api::project::ProjectTable>) -> pm_api::Result<()> { Ok(()) }
}

#[test]
fn test_tree_serde() -> Result<()>{
    let ansi = ANSI::new();
    log::set_logger(ansi);
    log::set_level(log::Level::Log);

    let tree : Project = serde_json::from_str(TEST_TREE)?;
    let mut pool = DatabaseBuilder::new()
        .set_reader(ReaderWriter{})
        .set_writer(ReaderWriter{})
        .build();
    pool.add_full_project(tree)?;

    println!("{pool:?}");

    Ok(())
}
