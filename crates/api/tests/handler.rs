use std::{fs::{File, ReadDir}, io::{Read, BufRead, BufReader}, path::{PathBuf, Path}, str::FromStr};

use project_manager_api::{error::*, config::{*, manager::{Location, Manager}}, *};

#[derive(Debug, Default)]
struct TestLoader{
    _inner_manager: String,
}

impl ProjectLoader for TestLoader{
    fn get_manager(&self) -> ProjectResult<String> {
        Ok(self._inner_manager.clone())
    }

    fn get_project(&self, location: &Location) -> ProjectResult<String> {
        let path =
            if let Location::Path{path: p} = location { p }
            else{ return Err(ProjectError::ProjectNotFound { name: None, path: None })};
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

    fn write_project(&mut self, s: String, location: &Location) -> ProjectResult<()> {
        Ok(())
    }
}


#[test]
fn test_add_project(){
    let root_path = PathBuf::from_str("/home/lilith/project_manager/crates/api/tests/test_projects").unwrap();
    let loader = TestLoader::default();
    let mut handler = ProjectsHandler::init(loader).unwrap();

    let entries = root_path.read_dir().unwrap();
    for (i, entry) in entries.into_iter().enumerate(){
        let path = entry.unwrap().path();
        let mut reader = BufReader::new(File::open(&path).unwrap());
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        handler.add_project(format!("project{i}"), Location::path(path));
    }

    handler.commit_manager().unwrap();
    println!("manager: {handler:?}");
    handler.commit_projects().unwrap();
}
