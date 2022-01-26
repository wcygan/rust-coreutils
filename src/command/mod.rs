use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;

pub mod echo;
pub mod ls;

pub const NEWLINE: &str = "\n";
pub const DOUBLE_SPACE: &str = "  ";

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

fn write_bytes(mut writer: Box<dyn Write>, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    match writer.write(bytes) {
        Ok(_t) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines() {
        let count = 3 as usize;
        let mut buf = String::new();

        for _ in 0..count {
            buf.push_str(NEWLINE);
        }

        assert_eq!(count, buf.lines().count())
    }
}
