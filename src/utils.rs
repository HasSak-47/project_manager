use std::{fs::File, io::{Read, BufReader}, default};
use crate::ProjectResult;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> ProjectResult<String>{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = Vec::new();
    buf_reader.read_to_end(&mut data)?;

    Ok(String::from_utf8(data)?)
}

/*
 * config: version asked
 * current: version compared to
 */
pub fn version_cmp(config: &str, current: &str) -> bool{
    if config == "*" { return true; }
    let config_parts  : Vec<&str> = config.split('.').collect();
    let current_parts : Vec<&str> = current.split('.').collect();

    // lmao what the fuck
    if config_parts[0] != current_parts[0] {
        return false;
    }
    if config_parts[1] == "*"{
        return true
    }
    if config_parts[1] != current_parts[1] {
        return false;
    }
    if config_parts[2] == "*"{
        return true
    }
    if config_parts[2] != current_parts[2] {
        return false;
    }
    true
}

use crate::error::ProjectError;
use std::path::PathBuf;

pub fn get_dir_str(a: fn() -> Option<PathBuf>) -> ProjectResult<String>{
    use ProjectError as PE;
    let str = a().ok_or(PE::DirNotFound)?.to_str().ok_or(PE::DirToStr)?.to_string();
    Ok(str)
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
