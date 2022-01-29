use std::error::Error;
use std::io::{BufRead};

use clap::{App, Arg, ArgMatches};

use crate::command::cat::lib::concatenate_readers;
use crate::command::{into_reader, stdin_reader};

mod lib;

pub const CAT: &str = "cat";
const FILES: &str = "files";

pub fn cat_command() -> App<'static> {
    App::new(CAT)
        .about("Copies each file, or standard input if none are given, to standard output")
        .arg(
            Arg::new(FILES)
                .required(false)
                .help("The files to concatenate")
                .min_values(0),
        )
}

pub fn cat_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
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

    concatenate_readers(readers)
}
