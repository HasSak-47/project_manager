use std::path::PathBuf;
use project_manager_api::{Database, Location};

pub fn exists(db: &Database, path: PathBuf) -> bool {
    let path = Location::Path(path);
    return db.get_all_projects()
        .iter()
        .find(|p| p.get_table().location == path)
        .is_some();
}

