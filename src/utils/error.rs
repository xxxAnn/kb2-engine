use core::fmt;
use std::error;

#[derive(Debug)]
pub enum Kb2Error {
    DBError(sqlite::Error),
    GenericError(String),
    MalformedRequest,
    IOError(std::io::Error),
}

impl fmt::Display for Kb2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kb2Error::DBError(_) => write!(f, "err_x\r\nDatabase error.\r\n"),
            Kb2Error::IOError(_) => write!(f, "err_x\r\nIO Error."),
            Kb2Error::GenericError(s) => write!(f, "err_x\r\nGeneric error: {s}.\r\n"),
            Kb2Error::MalformedRequest => write!(f, "err_0\r\nMalformed request.\r\n"),
        }
    }
}

impl error::Error for Kb2Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Kb2Error::DBError(ref e) => Some(e),
            Kb2Error::IOError(ref e) => Some(e),
            Kb2Error::GenericError(_) => None,
            Kb2Error::MalformedRequest => None
        }
    }
}

impl From<String> for Kb2Error {
    fn from(value: String) -> Self {
        Self::GenericError(value)
    }
}

impl From<&str> for Kb2Error {
    fn from(value: &str) -> Self {
        Self::GenericError(value.to_owned())
    }
}

impl From<&String> for Kb2Error {
    fn from(value: &String) -> Self {
        Self::GenericError(value.clone())
    }
}

impl From<sqlite::Error> for Kb2Error {
    fn from(value: sqlite::Error) -> Self {
        Self::DBError(value)
    }
}

impl From<std::io::Error> for Kb2Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}