const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::{fs::File, io::{BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}};
use cli::cli;
use anyhow::Result;
use project_manager_api::{
    Pool, project,
};

mod cli;

#[derive(Debug, Default)]
struct ProjectTOML{ } 

#[derive(Debug, Default)]
struct ManagerTOML{
    path: PathBuf,
} 

fn main() -> Result<()>{
    let mut pool = Pool();
    cli()?;
    Ok(())
}
