use std::{fs::File, path::PathBuf};
use std::fs;
use anyhow::Result;
use project_manager_api::project;
use std::io::Read;
use std::io::Write;
use project_manager_api::{project::Project, Database, Location};
use ly::log::prelude::*;

use crate::{Manager, Pair};

pub fn exists_path(db: &Database, path: PathBuf) -> bool {
    let path = Location::Path(path);
    return db.get_all_projects()
        .iter()
        .find(|p| *p.location() == path)
        .is_some();
}

pub fn exists_name(db: &Database, name: &String) -> bool {
    return db.get_all_projects()
        .iter()
        .find(|p| *p.name() == *name)
        .is_some();
}

pub fn database_path() -> Result<PathBuf>{
    let mut path = dirs::data_local_dir().unwrap();
    path.push("project_manager");

    let _ = log!("data path: {}", path.display());
    if !path.exists(){
        let _ = warn!("path does not exits creating...");
        fs::create_dir_all(&path)?;
    }

    path.push("projects");
    path.set_extension("toml");

    return Ok(path);
}

pub fn save_database(db: &Database) -> Result<()>{
    let path = database_path()?;
    let _ = log!("saving database {db:?}");
    let _ = log!("at {}", path.display());
    let mut manager = Manager::default();

    for project in db.build_project_trees()?{
        let _ = log!("saving project {:#?}", project);
        manager.projects.push(Pair{
            name: project.desc.name.clone(),
            loc: project.location.clone(),
        });

        if let Location::Path(p) = &project.location{
            let mut file = File::create(p)?;
            let _ = log!("writing project: {project:?}");
            file.write_all( toml::to_string_pretty(&project)?.as_bytes() )?;
        }
    }

    let mut file = File::create(path)?;
    let _ = log!("writing manager: {manager:?}");
    file.write_all( toml::to_string_pretty(&manager)?.as_bytes() )?;

    Ok(())
}

pub fn load_database(db: &mut Database) -> Result<()>{
    let path = database_path()?;
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string( &mut buf )?;
    let man : Manager = toml::from_str(buf.as_str()).unwrap();

    for Pair{loc, ..} in man.projects{
        if let Location::Path(p) = &loc{
            let mut file = File::open(p)?;
            let mut buf = String::new();
            file.read_to_string( &mut buf )?;

            let mut project : Project = toml::from_str(buf.as_str())?;
            project.location = loc;
            db.add_full_project(project)?;
        }

    }

    Ok(())
}
