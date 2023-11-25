#[derive(Debug)]
pub enum ProjectError{
    #[allow(dead_code)]
    Option,
    DirNotFound,
    DirToStr,
    IOError(std::io::Error),
    UTF8Error(std::str::Utf8Error),
    UTF8ErrorString(std::string::FromUtf8Error),
    TOMLFromStr(toml::de::Error),
    TOMLToStr(toml::ser::Error),
}

pub type ProjectResult<T> = Result<T, ProjectError>;

macro_rules! into_error{
    ($error_ty: ty, $error_cont: tt) => {
        impl From<$error_ty> for ProjectError{
            fn from(value: $error_ty) -> Self {
                Self::$error_cont(value)
            }
        }
    }
}

into_error!(std::io::Error, IOError);
into_error!(std::str::Utf8Error, UTF8Error);
into_error!(std::string::FromUtf8Error, UTF8ErrorString);
into_error!(toml::de::Error, TOMLFromStr);
into_error!(toml::ser::Error, TOMLToStr);

pub fn get_dir(a: fn() -> Option<std::path::PathBuf>) -> ProjectResult<String>{
    use ProjectError as PE;
    let str = a().ok_or(PE::DirNotFound)?.to_str().ok_or(PE::DirToStr)?.to_string();
    Ok(str)
}

#[allow(dead_code)]
pub fn to_res<T>(o: Option<T>) -> ProjectResult<T>{
    match o{
        Some(s) => Ok(s),
        None => Err(ProjectError::Option)
    }
}

#[allow(dead_code)]
pub fn to_res_err<T>(o: Option<T>, e: ProjectError) -> ProjectResult<T>{
    match o{
        Some(s) => Ok(s),
        None => Err(e)
    }
}
