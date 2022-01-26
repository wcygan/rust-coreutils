use std::error::Error;
use std::io::Write;
use std::path::Path;

use crate::command::{write_bytes, DOUBLE_SPACE, NEWLINE};

pub fn list_directory_contents(
    directory: &Path,
    writer: Box<dyn Write>,
    long_format: bool,
) -> Result<(), Box<dyn Error>> {
    let mut contents = vec![];
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap();
        contents.push(name)
    }

    let separator = match long_format {
        true => NEWLINE,
        false => DOUBLE_SPACE,
    };

    let buf = vec![contents.join(separator), NEWLINE.to_string()].join("");
    write_bytes(writer, buf.as_bytes())
}
