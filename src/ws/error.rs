use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WsError {
    message: String,
}

impl WsError {
    pub fn boxed(message: &str) -> Box<Self> {
        Box::new(WsError {
            message: message.to_owned(),
        })
    }

    pub fn message(message: &str) -> Self {
        WsError {
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for WsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WsError {}