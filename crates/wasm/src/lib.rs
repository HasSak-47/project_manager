use core::panic;
use anyhow::anyhow;
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use project_manager_api::*;

#[wasm_bindgen]
struct WasmHandler{
    handler: Handler,
}

struct WasmManagerIO{ }
struct WasmProjectIO{ }

impl manager::IO for WasmManagerIO {
    fn write(&mut self, _man: &manager::Manager) -> anyhow::Result<()> {
        Err(anyhow!("not supported"))
    }

    fn read(&self) -> anyhow::Result<manager::Manager> {
        Err(anyhow!("not supported"))
    }
}

impl project::IO for WasmProjectIO{
    fn write(&mut self, _location: &Location, _man: &project::ProjectStatus) -> anyhow::Result<()> {
        Err(anyhow!("not supported"))
    }

    fn read(&self, _location: &Location) -> anyhow::Result<project::ProjectStatus> {
        Err(anyhow!("not supported"))
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
impl WasmHandler{
    fn init() -> Self{
        let mut handler = Self{
            handler: Handler::new()
        };
        handler.handler.set_manager_io(WasmManagerIO{});
        handler.handler.set_project_io(WasmProjectIO{});

        return handler;
    }
}
