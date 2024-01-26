
pub const DEFAULT_MANAGER : &str = include_str!("./default_manager.toml");

pub fn create_project<S: AsRef<str>>(name: S, version: S, edition: S) -> String {
    let name = name.as_ref();
    let version = version.as_ref();
    let edition = edition.as_ref();
    format!(include_str!("./default_project.toml"), name, version, edition)
}
