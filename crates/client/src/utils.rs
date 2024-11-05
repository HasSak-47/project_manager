use std::path::PathBuf;
use project_manager_api::{Database, Location};

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
