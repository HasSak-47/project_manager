use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
mod manager;
mod project;

pub enum Location{
    Path(PathBuf),
    Url(String),
}

impl Location{
    pub fn path<P : AsRef<Path>>(path: P) -> Self{ Location::Path(path.as_ref().to_path_buf()) }
    pub fn url<S: AsRef<str>>(url: S) -> Self{ Location::Url(url.as_ref().to_string()) }

    pub fn get_path(&self) -> Result<&Path>{
        match self{
            Location::Path(path) => Ok(path.as_path()),
            _ => Err(anyhow!("Location is not a path!!")),
        }
    }

    pub fn get_url(&self) -> Result<&str>{
        match self{
            Location::Url(url) => Ok(url.as_str()),
            _ => Err(anyhow!("Location is not a url!!")),
        }
    }
}
