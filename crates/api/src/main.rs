pub mod db;
pub mod utils;
use anyhow::Result;
use db::DatabaseGraph;

fn main() -> Result<()> {
    let mut db = DatabaseGraph::from_file("./testing/root.db")?;
    utils::print_db( &db.root.con )?;
    return Ok(());
}
