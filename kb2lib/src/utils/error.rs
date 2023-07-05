use core::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    DBError(sqlite::Error),
    GenericError(String),
    MalformedRequest,
    CantCreateUser,
    IOError(std::io::Error),
    // Errors because of some malformed gamedata (.KB2 files)
    GameDataError(&'static str) 
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DBError(_) => write!(f, "err_x\r\nDatabase error.\r\n"),
            Error::IOError(_) => write!(f, "err_x\r\nIO Error."),
            Error::GenericError(s) => write!(f, "err_x\r\nGeneric error: {s}.\r\n"),
            Error::MalformedRequest => write!(f, "err_0\r\nMalformed request.\r\n"),
            Error::CantCreateUser => write!(f, "err_1\r\nCan't create user, ID might be invalid.\r\n"),
            Error::GameDataError(s) => write!(f, "err_2\r\nUnexpected Game Data error, please contact the developer: {s}.\r\n")
        } 
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::DBError(ref e) => Some(e),
            Error::IOError(ref e) => Some(e),
            Error::GenericError(_) => None,
            Error::MalformedRequest => None,
            Error::CantCreateUser => None,
            Error::GameDataError(_) => None
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::GenericError(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::GenericError(value.to_owned())
    }
}

impl From<&String> for Error {
    fn from(value: &String) -> Self {
        Self::GenericError(value.clone())
    }
}

impl From<sqlite::Error> for Error {
    fn from(value: sqlite::Error) -> Self {
        Self::DBError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}