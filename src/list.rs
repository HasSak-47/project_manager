use super::config::*;

pub fn list() {
    let projects = project::get_projects();
    println!("{projects:?}")
}
