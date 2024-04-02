use core::panic;
use std::{collections::HashMap, rc::Rc};

use js_sys::Promise;
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use project_manager_api::{config::manager::Location, FindCriteria, ProjectLoader, ProjectsHandler};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn fetch(url: &str) -> Promise;

    #[wasm_bindgen(js_namespace = blob)]
    fn text() -> String;
}

#[derive(Debug, Clone)]
pub struct _WebReader{
    manager: String,
    projects: HashMap<String, String>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WebReader{
    reader: Rc< _WebReader>
}

#[wasm_bindgen]
pub struct WebHandler{
    handler: ProjectsHandler<WebReader>,
}

#[wasm_bindgen]
impl WebReader{
    pub fn new() -> Self{
        WebReader{
            reader: Rc::new( _WebReader {
                manager: String::new(),
                projects: HashMap::new(),
            }),
        }
    }

    pub fn set_manager(&mut self, _manager: &str) {
        // Rc::make_mut(&mut self.reader).manager = manager.to_string();
    }

    pub fn set_project(&mut self, project: &str, toml: &str) {
        log(format!("Setting project: {project} to {toml}").as_str());
        // let project = project.to_string();
        // let toml = toml.to_string();
        // // Rc::make_mut(&mut self.reader).projects.insert(project, toml);
    }
}

#[wasm_bindgen]
impl WebHandler{
    pub fn new(reader: WebReader) -> Self{
        let handler = match ProjectsHandler::init(reader.clone()){
            Ok(h) => h,
            Err(e) => {
                log(&format!("Error: {:?}", e));
                panic!("Error: {:?}", e);
            },
        };
        WebHandler{handler}
    }

    pub fn get_completion(&mut self, name: &str) -> f64 {
        let name = name.to_string();
        self.handler.find_project_mut(&FindCriteria::Name(name)).unwrap().get_completion()
    }

    pub fn get_projects(&mut self) -> Vec<String>{
        self.handler.get_projects().into_iter().map(|p| p.get_name().clone()).collect()
    }

    pub fn get_project_path(&mut self, name: &str) -> String {
        let name = name.to_string();
        match self.handler.find_project_mut(&FindCriteria::Name(name)).unwrap().get_location() {
            Location::Url{ url } => url.clone(),
            _ => String::new(),
        }
    }
}

impl ProjectLoader for WebReader {
    fn get_manager(&self) -> anyhow::Result<String> {
        Ok(self.reader.manager.clone())
    }
    
    fn get_project(&self, location: &Location) -> anyhow::Result<String> {
        if let Location::Other(ref loc) = location {
            match self.reader.projects.get(loc) {
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

    fn write_manager(&mut self, _: String) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("You can't write to the web"))
    }

    fn write_project(&mut self, _: String, _: &Location) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("You can't write to the web"))
    }
}
