#![allow(unused_imports)]
use ly::log::{self, write::ANSI};
use ly::macro_error;
use ly::macro_log;
use pm_api::project::Project;
use pm_api::task::Task;
use project_manager_api::{Database, DatabaseBuilder, DatabaseReader, DatabaseWriter};
use serde;
use anyhow::Result;

const TEST_PROJECT: &str = include_str!("./test_project.json");
const TEST_TASK: &str = include_str!("./test_task.json");
use ly::log::prelude::*;

struct ReaderWriter{ }
use project_manager_api as pm_api;

impl DatabaseReader for ReaderWriter {
    fn read_all_tags(&self) -> pm_api::Result<Vec<pm_api::tags::TagTable>> { Ok(Vec::new()) }
    fn read_all_tasks(&self) -> pm_api::Result<Vec<pm_api::task::TaskTable>> { Ok(Vec::new()) }
    fn read_all_projects(&self) -> pm_api::Result<Vec<pm_api::project::ProjectTable>> { Ok(Vec::new()) }
}

impl DatabaseWriter for ReaderWriter {
    fn write_all_tags(&mut self, _: &mut Vec<pm_api::tags::TagTable>) -> pm_api::Result<()> { Ok(()) }
    fn write_all_tasks(&mut self, _: &mut Vec<pm_api::task::TaskTable>) -> pm_api::Result<()> { Ok(()) }
    fn write_all_projects(&mut self, _: &mut Vec<pm_api::project::ProjectTable>) -> pm_api::Result<()> { Ok(()) }
}

#[test]
fn test_add_project() -> Result<()>{
    let ansi = ANSI::new();
    log::set_logger(ansi);
    log::set_level(log::Level::Log);

    let tree : Project = serde_json::from_str(TEST_PROJECT)?;
    let mut pool = DatabaseBuilder::new()
        .set_reader(ReaderWriter{})
        .set_writer(ReaderWriter{})
        .build();
    pool.add_full_project(tree)?;

    println!("{pool:?}");
    Ok(())
}

#[test]
fn test_add_task() -> Result<()>{
    let ansi = ANSI::new();
    log::set_logger(ansi);
    log::set_level(log::Level::Log);

    let project : Project = serde_json::from_str(TEST_PROJECT)?;
    let mut pool = DatabaseBuilder::new()
        .set_reader(ReaderWriter{})
        .set_writer(ReaderWriter{})
        .build();

    pool.add_full_project(project)?;

    let task: Task = serde_json::from_str(TEST_TASK)?;
    println!("{task:?}");
    pool.add_full_task(task)?;

    println!("{pool:?}");
    Ok(())
}
