use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::{fmt, io};

pub mod cat;
pub mod date;
pub mod echo;
pub mod head;
pub mod ls;
pub mod tree;
pub mod wc;
pub mod yell;
pub mod yes;

pub const NEWLINE: &str = "\n";
pub const SINGLE_SPACE: &str = " ";
pub const DOUBLE_SPACE: &str = "  ";
pub const TRIPLE_SPACE: &str = "   ";
pub const CURRENT_DIRECTORY: &str = ".";
pub const HIDDEN_FILE_PREFIX: &str = ".";

fn get_stdin_text() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(Box::new(e)),
    }
}

fn stdin_reader() -> Box<dyn BufRead> {
    Box::new(BufReader::new(io::stdin()))
}

fn print_reader(mut reader: Box<dyn BufRead>) -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}

fn into_reader(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    let file = into_file(filename)?;
    Ok(Box::new(BufReader::new(file)))
}

fn into_file(filename: &str) -> Result<File, Box<dyn Error>> {
    match File::open(filename) {
        Ok(file) => Ok(file),
        Err(_) => Err(Box::new(NotFoundError::new(format!(
            "cannot find file {}",
            filename
        )))),
    }
}

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
        let count = 3_usize;
        let mut buf = String::new();

        for _ in 0..count {
            buf.push_str(NEWLINE);
        }

        assert_eq!(count, buf.lines().count())
    }
}
