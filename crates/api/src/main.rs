pub mod db;
use anyhow::Result;
use db::DatabaseGraph;

fn main() -> Result<()> {
    let mut db = DatabaseGraph::from_file("./testing/root.db")?;
    return Ok(());
}
