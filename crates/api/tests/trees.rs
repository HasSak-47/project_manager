use project_manager_api::{trees::ProjectTree, Pool};
use serde;
use anyhow::Result;

const TEST_TREE: &str = include_str!("./test_tree.json");

#[test]
fn test_tree_serde() -> Result<()>{
    let tree : ProjectTree = serde_json::from_str(TEST_TREE)?;
    println!("tree: {tree:#?}");
    let mut pool = Pool::new();
    pool.add_project_tree(tree);
    println!("pool: {pool:#?}");

    Ok(())
}
