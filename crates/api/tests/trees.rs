#![allow(unused_imports)]
use std::fs::File;
use std::io::{Read, Write};

use project_manager_api as pm_api;

use ly::log::{self, write::ANSI};
use ly::macro_error;
use ly::macro_log;
use pm_api::project::Project;
use pm_api::task::Task;
<<<<<<< HEAD
use project_manager_api::desc::{Description, Descriptor};
use project_manager_api::{Database, DatabaseBuilder, DatabaseReader, DatabaseWriter};
=======
use pm_api::desc::Descriptor;
use pm_api::{Database, DatabaseBuilder, DatabaseReader, DatabaseWriter};
use rand::random;
>>>>>>> dc7eaf5 (tests seems to be fineee)
use serde;
use anyhow::{anyhow, Result};

const TEST_PROJECT: &str = include_str!("./test_project.json");
const TEST_TASK: &str = include_str!("./test_task.json");
use ly::log::prelude::*;

struct ReaderWriter{ }

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

#[test]
fn unfold() -> Result<()>{
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
    pool.add_full_task(task)?;

    let _projects = pool.build_project_trees()?;
    let _task = pool.build_task_tree(0)?;

    println!("{_task:?}");


    Ok(())
}


fn create_random_project(depth: usize) -> Project {
}


#[test]
fn stress_test() -> Result<()>{
    use rand::prelude::*;
    let n_rp : usize = rand::random();
    let mut root_projects = Vec::with_capacity(n_rp);
    for i in 0..n_rp{
        let project = Project::new()
            .desc(Descriptor::new().name(format!("root_{i}")))
            .last_worked(format!("{} {} {}", rand::random::<u32>() % 28, rand::random::<u32>() % 12, 2000 + rand::random::<u32>() % 24));
        root_projects.push(project);
    }



    Ok(())
}

fn proc_task(depth: usize, max: usize) -> Task{
    let mut task = Task::new().desc(Descriptor::new());
    let child = rand10() + depth < 2 * max;
    if child && depth < max{
        let child_count : usize = rand10();
        for _ in 0..child_count{
            task.childs.push(proc_task(depth + 1, max));
        }
    }

    return task;
}

fn proc_project(depth: usize, max: usize) -> Project{
    let mut project = Project::new().desc(
        Descriptor::new()
    );
    let child = rand10() + depth < 2 * max;
    if child && depth < max{
        let child_count : usize = rand10();
        for _ in 0..child_count{
            project.childs.push(proc_project(depth + 1, max));
        }
    }
    let task = rand10() + depth < 2 * max;
    if task && depth < max{
        let task_count : usize = rand10();
        for _ in 0..task_count{
            project.tasks.push(proc_task(depth + 1, max));
        }
    }

    return project;
}

#[test]
fn proc_test() -> Result<()>{
    let project = proc_project(0, 4);
    let mut pool = DatabaseBuilder::new().build();
    pool.add_full_project(project.clone())?;
    let mut tree = pool.build_project_trees()?;
    if tree.len() != 1{
        return Err(anyhow!("project not returned"));
    }
    let returned_project = tree.pop().ok_or(anyhow!("how???"))?;
    if project != returned_project {
        println!("projects are different")
    }


    Ok(())
}

fn rand10() -> usize{
    random::<usize>() % 10
}
