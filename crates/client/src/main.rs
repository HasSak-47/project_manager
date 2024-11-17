const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::{fs::OpenOptions, io::Read};

use cli::cli;
use anyhow::Result;
use ly::log::write::ANSI;
use project_manager_api::{
    Database, Location,
};

mod cli;
mod utils;
use serde::{Deserialize, Serialize};
use ly::log::prelude::*;
use utils::{database_path, load_database};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Pair{
    name: String,
    loc: Location,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Manager{
    #[serde(default = "Vec::new")]
    projects : Vec<Pair>,
}

fn main() {
    panic_main().unwrap();
}

fn panic_main() -> Result<()>{
    ly::log::set_logger(ANSI::new());
    ly::log::set_level(ly::log::Level::Error);

    let path = database_path()?;
    let _ = log!("opening file at {}", path.display());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let _ = log!("reading file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let _ = log!("read file");
    

    let mut db = Database::default();

    load_database(&mut db)?;
    cli(db)?;
    Ok(())
}
