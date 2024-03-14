use std::collections::HashMap;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use project_manager_api::{config::manager::Location, FindCriteria, ProjectLoader, ProjectsHandler};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn fetch(url: &str) -> Promise;
    #[wasm_bindgen(js_namespace = blob)]
    fn text() -> String;
}

#[wasm_bindgen]
pub struct WebReader{
    manager: String,
    projects: HashMap<String, String>,
}

#[wasm_bindgen]
pub struct WebHandler{
    handler: ProjectsHandler<WebReader>,
}

#[wasm_bindgen]
impl WebReader{
    pub fn new() -> Self{
        WebReader{
            manager: String::new(),
            projects: HashMap::new(),
        }
    }

    pub fn set_manager(&mut self, manager: String) {
        self.manager = manager;
    }

    pub fn set_project(&mut self, project: String, toml: String) {
        self.projects.insert(project, toml);
    }
}

#[wasm_bindgen]
impl WebHandler{
    pub fn new(reader: WebReader) -> Self{
        let handler = ProjectsHandler::init(reader).unwrap();
        WebHandler{handler}
    }

    pub fn get_completion(&mut self, name: String) -> f64 {
        self.handler.find_project_mut(&FindCriteria::Name(name)).unwrap().get_completion()
    }
}

impl ProjectLoader for WebReader {
    fn get_manager(&self) -> anyhow::Result<String> {
        Ok(self.manager.clone())
    }
    
    fn get_project(&self, location: &Location) -> anyhow::Result<String> {
        if let Location::Other(ref loc) = location {
            match self.projects.get(loc) {
                Some(toml) => Ok(toml.clone()),
                None => Err(anyhow::anyhow!("Project not found")),
            }
        } else {
            Err(anyhow::anyhow!("Project not found"))
        }
    }

    fn ensure_existance(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn write_manager(&mut self, data: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("You can't write to the web"))
    }

    fn write_project(&mut self, data: String, location: &Location) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("You can't write to the web"))
    }
}
