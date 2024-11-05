const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::{cell::RefCell, fs::{self, File, OpenOptions}, io::{Read, Write}, path::PathBuf, sync::Arc};

use cli::cli;
use anyhow::Result;
use ly::log::write::ANSI;
use project_manager_api::{
    project::*, tags::*, task::*, DatabaseBuilder, DatabaseError, DatabaseReader, DatabaseWriter, Location, Result as DBResult
};

mod cli;
mod utils;
use serde::{Deserialize, Serialize};
use ly::log::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Pair{
    name: String,
    loc: Location,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ManagerData{
    #[serde(default = "Vec::new")]
    projects: Vec<Pair>,
    #[serde(default = "Vec::new")]
    orphan_tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
struct Manager{
    arc: Arc< RefCell< ManagerData > >,
}
impl DatabaseReader for Manager{
    // hack: this creates a temporal database then adds all the projects and then copies the
    // tables and returns them
    fn read_all_projects(&self) -> DBResult<Vec<ProjectTable>> {
        let projects = &(self.arc.borrow()).projects;
        let mut t_db = DatabaseBuilder::new().build();
        for pair in projects{
            let _name = pair.name.clone();
            let loc = pair.loc.clone();
            if let Location::Path(p) = loc{
                let mut file = File::open(&p).map_err(|_| DatabaseError::other("could not open file"))?;
                let mut buf = String::new();
                file.read_to_string(&mut buf).map_err(|_| DatabaseError::other(format!("could not read file {}", p.display())))?;

                t_db.add_full_project(toml::from_str(&buf).map_err(|_| DatabaseError::other("could not parse project"))?)?;
            }
        }
        let k : Vec<_> = t_db
            .get_all_projects()
            .into_iter()
            .map(|k| {
                k.get_table().clone()
            })
            .collect();

        return Ok(k);
    }

    fn read_all_tasks(&self) -> DBResult<Vec<TaskTable>> {
        let mut t_db = DatabaseBuilder::new().build();
        let tasks = &(self.arc.borrow()).orphan_tasks;
        for task in tasks{
            t_db.add_full_task(task.clone())?;
        }
        Ok(Vec::new())
    }
    fn read_all_tags(&self) -> DBResult<Vec<TagTable>> { Ok(Vec::new()) }
}
impl DatabaseWriter for Manager{
    fn write_all_projects(&mut self, p: &mut Vec<ProjectTable>) -> DBResult<()> { 
        let data = &mut *(*self.arc).borrow_mut();
        data.projects.clear();

        // hack: create temporal database that converts Vec<ProjectTable> into Vec<Project>
        struct FakeReader{
            tables: Vec<ProjectTable>,
        }
        impl DatabaseReader for FakeReader{
            fn read_all_projects(&self) -> DBResult<Vec<ProjectTable>> {
                Ok(self.tables.clone())
            }

            fn read_all_tags(&self) -> DBResult<Vec<TagTable>> {
                Ok(Vec::new())
            }
            fn read_all_tasks(&self) -> DBResult<Vec<TaskTable>> {
                Ok(Vec::new())
            }
        }

        let fake = FakeReader {tables : p.clone()};
        let mut temp_db = DatabaseBuilder::new().set_reader(fake).build();
        temp_db.load_data()?;
        let _ = log!("database: {temp_db:#?}");

        let projects = temp_db.build_project_trees()?;

        for p in projects{
            let name = p.desc.name.clone();
            let loc  = p.location.clone();
            data.projects.push(Pair {name, loc: loc.clone()});
            if let Location::Path(path) = loc{
                let mut file = File::create(&path).map_err(|other| DatabaseError::other(other.to_string()))?;
                let toml = toml::to_string_pretty(&p).map_err(|other| DatabaseError::other(other.to_string()))?;
                file.write_all(toml.as_bytes()).map_err(|other| DatabaseError::other(other.to_string()))?;
            }
        }

        let _ = log!("database: {data:?}");
        let mut file = File::create(db_file()).unwrap();
        let tml = toml::to_string(data).unwrap();
        let _ = file.write_all(tml.as_bytes());

        Ok(())
    }
    fn write_all_tasks(&mut self, _: &mut Vec<TaskTable>) -> DBResult<()> { Ok(()) }
    fn write_all_tags(&mut self, _: &mut Vec<TagTable>) -> DBResult<()> { Ok(()) }
}

fn db_dir() -> PathBuf{
    let mut path = dirs::data_local_dir().unwrap();
    path.push("project_manager");
    return path;
}

fn db_file() -> PathBuf{
    let mut path = db_dir();
    path.push("projects");
    path.set_extension("toml");
    return path;
}

fn main() -> Result<()>{
    ly::log::set_logger(ANSI::new());
    ly::log::set_level(ly::log::Level::Error);
    
    let mut path = dirs::data_local_dir().unwrap();
    path.push("project_manager");

    let _ = log!("data path: {}", path.display());
    if !path.exists(){
        let _ = warn!("path does not exits creating...");
        fs::create_dir_all(&path)?;
    }

    path.push("projects");
    path.set_extension("toml");

    let _ = log!("opening file at {}", path.display());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let _ = log!("reading file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let _ = log!("read file");

    let buf = String::from_utf8(buf)?;
    let manager = toml::from_str(&buf)?;

    let manager = Manager{
        arc: Arc::new( RefCell::new( manager ) )
    };

    let db = DatabaseBuilder::new()
        .set_writer(manager.clone())
        .set_reader(manager.clone())
        .build();

    cli(db)?;
    Ok(())
}
