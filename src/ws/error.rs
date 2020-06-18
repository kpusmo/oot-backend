use std::error::Error;
use serde::export::Formatter;
use std::fmt;

#[derive(Debug)]
pub struct WsError {
    message: String,
}

impl WsError {
    pub fn message(message: String) -> Box<Self> {
        Box::new(WsError {
            message,
        })
    }
}

impl fmt::Display for WsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WsError {}