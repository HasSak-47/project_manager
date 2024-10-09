use ly::log::{self, write::ANSI};
use ly::macro_error;
use ly::macro_log;
use project_manager_api::{trees::ProjectTree, Pool};
use serde;
use anyhow::Result;

const TEST_TREE: &str = include_str!("./test_tree.json");
use ly::log::prelude::*;

#[test]
fn test_tree_serde() -> Result<()>{
    let ansi = ANSI::new();
    log::set_logger(ansi);
    log::set_level(log::Level::Log);
    let tree : ProjectTree = serde_json::from_str(TEST_TREE)?;
    let mut pool = Pool::new();
    pool.add_project_tree(tree);

    Ok(())
}
