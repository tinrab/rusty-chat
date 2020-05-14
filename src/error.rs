use std::{error, fmt, io, result};

use serde::export::Formatter;
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    System(String),
    Io(io::Error),
    WebSocket(tungstenite::Error),
    Message(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::System(err) => write!(f, "system error: {}", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::WebSocket(ref err) => write!(f, "WebSocket error: {}", err),
            Error::Message(ref err) => write!(f, "Invalid message: {}", err),
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Message(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
