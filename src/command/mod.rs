use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use crate::command::error::{ArgumentError, NotFoundError};

pub mod cat;
pub mod cp;
pub mod date;
pub mod echo;
pub mod error;
pub mod head;
pub mod ls;
pub mod nl;
pub mod tail;
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

pub fn into_file_path(filename: &str) -> Result<&Path, Box<dyn Error>> {
    let path = Path::new(filename);

    if !path.exists() {
        return Err(Box::new(ArgumentError::new(format!(
            "the path {} does not exist",
            filename
        ))));
    }

    if !path.is_file() {
        return Err(Box::new(ArgumentError::new(format!(
            "the path {} is not a file",
            filename
        ))));
    }

    Ok(path)
}

pub fn into_directory_path(dirname: &str) -> Result<&Path, Box<dyn Error>> {
    let path = Path::new(dirname);

    if !path.exists() {
        return Err(Box::new(ArgumentError::new(format!(
            "the path {} does not exist",
            dirname
        ))));
    }

    if !path.is_dir() {
        return Err(Box::new(ArgumentError::new(format!(
            "the path {} is not a directory",
            dirname
        ))));
    }

    Ok(path)
}

fn write_bytes(mut writer: Box<dyn Write>, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    match writer.write(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
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
