use std::io::Error as ioError;
use std::string::FromUtf8Error;
use toml::de::Error as TomlDeError;

#[derive(Clone, Debug)]
pub enum ErrorType{
    IO,
    StrFromUTF8,
    Toml,
    Other,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Error{
    error_type: ErrorType,
    message   : String,
}

impl Error{
    pub fn new() -> Self{
        Error{
            error_type: ErrorType::Other,
            message   : "can't be fucked".to_string(),
        }
    }
}

macro_rules! ImplError {
    ($err_type: tt, $err_enum: tt, $to_string: block, $value: ident) => {
        impl From<$err_type> for Error{
            fn from($value: $err_type) -> Self {
                Error{
                    error_type: ErrorType::$err_enum,
                    message   : $to_string,
                }
            }
        }
    }
}

ImplError!(ioError, IO, {value.to_string()}, value);
ImplError!(FromUtf8Error, StrFromUTF8, {value.to_string()}, value);
ImplError!(TomlDeError, Toml, {value.to_string()}, value);

// impl From<ioError> for Error{
//     fn from(value: ioError) -> Self {
//         Error{
//             error_type: ErrorType::IO,
//             message   : value.to_string(),
//         }
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;
