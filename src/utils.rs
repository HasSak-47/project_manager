use std::{fs::File, io::{Read, BufReader}};
use crate::ProjectResult;
use std::path::{Path, PathBuf};
use crate::error::ProjectError;

pub fn read_file<P: AsRef<Path>>(path: P) -> ProjectResult<String>{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = Vec::new();
    buf_reader.read_to_end(&mut data)?;

    Ok(String::from_utf8(data)?)
}

pub fn get_dir(a: fn() -> Option<PathBuf>) -> ProjectResult<PathBuf>{
    use ProjectError as PE;
    Ok(a().ok_or(PE::DirNotFound)?)
}

#[allow(dead_code)]
pub fn to_res<T>(o: Option<T>) -> ProjectResult<T>{
    match o{
        Some(s) => Ok(s),
        None => Err(ProjectError::Option)
    }
}

#[allow(dead_code)]
pub fn to_res_err<T>(o: Option<T>, e: ProjectError) -> ProjectResult<T>{
    match o{
        Some(s) => Ok(s),
        None => Err(e)
    }
}
