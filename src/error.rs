#[derive(Debug)]
pub enum ProjectError{
    DirNotFound,
    DirToStr,
}

pub type ProjectResult<T> = Result<T, ProjectError>;
