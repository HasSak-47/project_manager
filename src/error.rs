#[allow(dead_code)]
#[derive(Debug)]
pub enum ProjectError{
    #[allow(dead_code)]
    Option,
    DirNotFound,
    DirToStr,
    CliOptionUnknown,
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
