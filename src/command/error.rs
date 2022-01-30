use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NotFoundError {
    msg: String,
}

impl NotFoundError {
    pub fn new(msg: String) -> NotFoundError {
        NotFoundError { msg }
    }
}

impl Error for NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentError {
    msg: String,
}

impl ArgumentError {
    pub fn new(msg: String) -> NotFoundError {
        NotFoundError { msg }
    }
}

impl Error for ArgumentError {}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
