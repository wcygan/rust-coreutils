use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fs::DirEntry;
use std::io::Write;
use std::path::Path;

pub mod echo;
pub mod ls;
pub mod tree;
pub mod wc;

pub const NEWLINE: &str = "\n";
pub const SINGLE_SPACE: &str = " ";
pub const DOUBLE_SPACE: &str = "  ";
pub const TRIPLE_SPACE: &str = "   ";
pub const CURRENT_DIRECTORY: &str = ".";
pub const HIDDEN_FILE_PREFIX: &str = ".";

fn write_bytes(mut writer: Box<dyn Write>, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    match writer.write(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

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
