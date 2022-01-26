use std::error::Error;
use std::io::{Read, Write};

use crate::command::NEWLINE;

pub fn print_args(args: Vec<&str>, writer: Box<dyn Write>) -> Result<(), Box<dyn Error>> {
    let mut buffer: String = args.join(" ");
    buffer.push_str(NEWLINE);
    write_bytes(writer, buffer.as_bytes())
}

pub fn print_stdin(writer: Box<dyn Write>) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    buffer.push_str(NEWLINE);
    write_bytes(writer, buffer.as_bytes())
}

fn write_bytes(mut writer: Box<dyn Write>, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    match writer.write(bytes) {
        Ok(_t) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
