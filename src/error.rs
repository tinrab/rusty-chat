use serde::export::Formatter;
use std::{error, io};
use std::{fmt, result};
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
