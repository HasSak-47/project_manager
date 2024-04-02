use std::{fs::{File, ReadDir}, io::{Read, BufRead, BufReader, Write}, path::{PathBuf, Path}, str::FromStr};

use project_manager_api::{error::*, config::{*, manager::{Location, Manager}}, *};

#[derive(Debug, Default)]
struct TestLoader{
    _inner_manager: String,
}

fn get_path(location: &Location) -> ProjectResult<&PathBuf> {
    if let Location::Path{path: p} = location { Ok(p) }
    else{ Err(ProjectError::ProjectNotFound { name: None, path: None })}
}

impl ProjectLoader for TestLoader{
    fn get_manager(&self) -> ProjectResult<String> {
        Ok(self._inner_manager.clone())
    }

    fn get_project(&self, location: &Location) -> ProjectResult<String> {
        let mut path : PathBuf = get_path(location)?;
        path.push("status");
        path.set_extension("toml");
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf);

        Ok(buf)
    }

    fn ensure_existance(&mut self) ->ProjectResult<()> {
        if self._inner_manager.is_empty() {
            return Err(ProjectError::DirNotFound)
        }
        return Ok(())
    }
    
    fn write_manager(&mut self, s: String) -> ProjectResult<()> {
        self._inner_manager = s;
        Ok(())
    }

    fn write_project(&mut self, data: String, location: &Location) -> ProjectResult<()> {
        let path = get_path(location)?;
        let mut file = File::create(path).unwrap();
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

#[test]
fn test_add_project(){
    let root_path = PathBuf::from_str("tests/test_projects").unwrap();
    let loader = TestLoader::default();
    let mut handler = ProjectsHandler::init(loader).unwrap();

    let entries = root_path.read_dir().unwrap();

    for (i, entry) in entries.into_iter().enumerate(){
        let path = entry.unwrap().path();
        handler.new_project(format!("project{i}"), Location::path(path)).unwrap();
    }

    handler.commit_projects().unwrap();
    handler.commit_manager().unwrap();
}
