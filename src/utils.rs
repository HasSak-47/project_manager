use std::{fs::File, io::{Read, BufReader}};
use crate::ProjectResult;
use std::path::Path;

pub fn read_file<S: AsRef<Path>>(path: S) -> ProjectResult<String>{
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut data = Vec::new();
    buf_reader.read_to_end(&mut data)?;

    Ok(String::from_utf8(data)?)
}
