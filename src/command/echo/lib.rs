use std::error::Error;
use std::io::Write;

use crate::command::{get_stdin_text, write_bytes, NEWLINE};

pub fn print_args(args: Vec<&str>, writer: Box<dyn Write>) -> Result<(), Box<dyn Error>> {
    let mut buffer: String = args.join(" ");
    buffer.push_str(NEWLINE);
    write_bytes(writer, buffer.as_bytes())
}

pub fn print_stdin(writer: Box<dyn Write>) -> Result<(), Box<dyn Error>> {
    let mut buffer = get_stdin_text()?;
    buffer.push_str(NEWLINE);
    write_bytes(writer, buffer.as_bytes())
}
