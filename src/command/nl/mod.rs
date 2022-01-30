use std::error::Error;
use std::io::BufRead;

use clap::{App, Arg, ArgMatches};

use crate::command::nl::lib::prepend_line_numbers;
use crate::command::{into_reader, stdin_reader};

mod lib;

pub const NL: &str = "nl";
const FILES: &str = "files";

pub fn nl_command() -> App<'static> {
    App::new(NL)
        .about("Prepends line numbers to the input files and prints them to stdout")
        .arg(
            Arg::new(FILES)
                .required(false)
                .help("The files to number the lines of")
                .min_values(0),
        )
}

pub fn nl_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let readers: Vec<Box<dyn BufRead>> = match matches.values_of(FILES) {
        None => vec![stdin_reader()],
        Some(files) => {
            let mut readers = vec![];
            for file in files {
                readers.push(into_reader(file)?)
            }
            readers
        }
    };

    prepend_line_numbers(readers)
}
