use std::error::Error;
use std::io::{Read, Write};

use clap::{App, Arg, ArgMatches};

use crate::command::NEWLINE;

pub const ECHO: &str = "echo";
const TEXT: &str = "text";

pub fn echo_command() -> App<'static> {
    App::new(ECHO)
        .about("Echos the provided text")
        .arg(Arg::new(TEXT).required(false).min_values(0))
}

pub fn echo_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let writer = Box::new(std::io::stdout());
    match matches.values_of(TEXT) {
        None => print_stdin(writer),
        Some(values) => {
            let args = values.collect();
            print_args(args, writer)
        }
    }
}

fn print_args(args: Vec<&str>, writer: Box<dyn Write>) -> Result<(), Box<dyn Error>> {
    let mut buffer: String = args.join(" ");
    buffer.push_str(NEWLINE);
    write_bytes(writer, buffer.as_bytes())
}

fn print_stdin(writer: Box<dyn Write>) -> Result<(), Box<dyn Error>> {
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
