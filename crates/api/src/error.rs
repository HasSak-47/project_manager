
use crate::config::manager::Location;

#[derive(Debug, thiserror::Error)]
pub enum ProjectError{
    #[error("Toml doesn't work!")]
    BrokenToml,
    #[error("Project name ({name}) already in Manager")]
    NameAlreadyManaged{ name: String, },
    #[error("Project location ({location}) already in Manager")]
    LocationAlreadyManaged{ location: Location, },
}
