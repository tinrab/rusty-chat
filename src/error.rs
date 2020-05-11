use std::fmt::Display;
use std::{error, io};
use std::{fmt, result};

use serde::export::Formatter;
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    WebSocket(tungstenite::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::WebSocket(ref err) => write!(f, "WebSocket error: {}", err),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Error::WebSocket(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
