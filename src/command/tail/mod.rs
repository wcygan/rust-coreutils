use std::error::Error;
use std::path::Path;

use clap::{App, Arg, ArgMatches};

use crate::command::tail::lib::print_tail;
use crate::command::{into_reader, stdin_reader};

mod lib;

pub const TAIL: &str = "tail";
const FILE: &str = "file";
const FOLLOW_L: &str = "follow";
const FOLLOW_S: char = 'f';
const NUMBER_L: &str = "number";
const NUMBER_S: char = 'n';
const NUMBER_DEFAULT_VALUE: &str = "10";

pub fn tail_command() -> App<'static> {
    App::new(TAIL)
        .about("Prints the last part of each file")
        .arg(
            Arg::new(FILE)
                .required(false)
                .help("The file to show the last part of"),
        )
        .arg(
            Arg::new(NUMBER_L)
                .required(false)
                .long(NUMBER_L)
                .short(NUMBER_S)
                .help("The number of lines to show")
                .default_value(NUMBER_DEFAULT_VALUE),
        )
        .arg(
            Arg::new(FOLLOW_L)
                .required(false)
                .long(FOLLOW_L)
                .short(FOLLOW_S)
                .help(
                    "Loop trying to read more characters at the end of the file until interrupted",
                ),
        )
}

pub fn tail_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let number = matches
        .value_of(NUMBER_L)
        .unwrap_or(NUMBER_DEFAULT_VALUE)
        .parse::<usize>()?;

    let (reader, path) = match matches.value_of(FILE) {
        None => (stdin_reader(), None),
        Some(file) => (into_reader(file)?, Some(Path::new(file))),
    };

    let follow = matches.is_present(FOLLOW_L);

    print_tail(reader, path, number, follow)
}
