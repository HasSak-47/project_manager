const VERSION: &str = env!("CARGO_PKG_VERSION");


use project_manager_api::Location;
use cli::cli;
use anyhow::Result;

mod cli;
mod utils;
use serde::{Deserialize, Serialize};

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
    cli()?;

    Ok(())
}
