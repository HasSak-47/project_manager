use std::{env::current_dir, path::PathBuf};
use anyhow::Result;


pub fn find_local_status() -> Result<PathBuf> {
    let mut path = current_dir()?;
    path.push("status");
    path.set_extension("toml");
    Ok(path)
}
