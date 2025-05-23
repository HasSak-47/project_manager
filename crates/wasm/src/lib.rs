use async_std::task::block_on;

use anyhow::anyhow;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use project_manager_api::{config::manager::Location, FindCriteria, ProjectLoader, ProjectsHandler};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn fetch(url: &str) -> Promise;
}

#[wasm_bindgen]
pub struct WebReader{
    manager_url: String,
}

fn blocked_fetch(url: &str) -> String{
    let promise = fetch(url);
    let future = JsFuture::from(promise);
    let mut result = String::new();
    let ptr = &mut result as *mut String;
    // this may be a bad idea
    block_on( unsafe { async move {
        match future.await{
            Ok(value) => {
                let value = value.as_string().unwrap_throw();
                *ptr = value;
            },
            Err(err) => {
                log(&format!("Error: {:?}", err));
            }
        }
    }});
    result
}

#[wasm_bindgen]
impl WebReader{
    pub fn new(manager_url: String) -> Self{
        Self{ manager_url }
    }
}

impl ProjectLoader for WebReader{
    fn write_project(&mut self, _data: String, _location: &Location) -> anyhow::Result<()> {
        Err(anyhow!("Cannot write project"))
    }

    fn write_manager(&mut self, _data: String) -> anyhow::Result<()> {
        Err(anyhow!("Cannot write manager"))
    }

    fn get_project(&self, location: &Location) -> anyhow::Result<String> {
        Ok(blocked_fetch(&location.to_string()))
    }

    fn get_manager(&self) -> anyhow::Result<String> {
        Ok(blocked_fetch(&self.manager_url))
    }

    fn ensure_existance(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen]
pub struct WebProjectsHandler{
    _handler: ProjectsHandler<WebReader>,
}

#[wasm_bindgen]
impl WebProjectsHandler{
    pub fn init(reader: WebReader) -> Option<WebProjectsHandler> {
        let _handler = ProjectsHandler::init(reader).unwrap_throw();
        Some(Self{ _handler })
    }

    pub fn load_projects(&mut self){
        self._handler.load_projects();
    }

    pub fn get_project_completion(&mut self, name: String) -> f64{
        let proj = self._handler.find_project_mut(&FindCriteria::name(name)).unwrap_throw();
        proj.get_completion_mut()
    }
}

