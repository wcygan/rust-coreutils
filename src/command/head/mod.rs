use std::error::Error;
use std::io::BufRead;

use clap::{App, Arg, ArgMatches};

use crate::command::head::lib::print_first_n_lines;
use crate::command::{into_reader, stdin_reader};

mod lib;

pub const HEAD: &str = "head";
const FILES: &str = "files";
const NUMBER_L: &str = "number";
const NUMBER_S: char = 'n';
const NUMBER_DEFAULT_VALUE: &str = "10";

pub fn head_command() -> App<'static> {
    App::new(HEAD)
        .about("Prints the first part of each file")
        .arg(
            Arg::new(FILES)
                .required(false)
                .help("The files to show the first part of")
                .min_values(0),
        )
        .arg(
            Arg::new(NUMBER_L)
                .required(false)
                .long(NUMBER_L)
                .short(NUMBER_S)
                .help("The number of lines to show")
                .default_value(NUMBER_DEFAULT_VALUE),
        )
}

pub fn head_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let number = matches
        .value_of(NUMBER_L)
        .unwrap_or(NUMBER_DEFAULT_VALUE)
        .parse::<u32>()?;

    let readers: Vec<(Option<&str>, Box<dyn BufRead>)> = match matches.values_of(FILES) {
        None => vec![(None, stdin_reader())],
        Some(files) => {
            let mut readers = vec![];
            for file in files {
                readers.push((Some(file), into_reader(file)?))
            }
            readers
        }
    };

    let sz = readers.len();
    for (name, reader) in readers {
        let name = match sz == 1 {
            true => None,
            false => name,
        };
        print_first_n_lines(name, reader, number)?
    }

    Ok(())
}
