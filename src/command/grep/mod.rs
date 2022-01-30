use std::error::Error;
use std::io::BufRead;

use clap::{App, Arg, ArgMatches};

use crate::command::error::ArgumentError;
use crate::command::grep::lib::search_readers;
use crate::command::{into_reader, stdin_reader};

mod lib;

pub const GREP: &str = "grep";
const FILES: &str = "files";
const QUERY: &str = "query";
const CASE_INSENSITIVE_L: &str = "case";
const CASE_INSENSITIVE_S: char = 'c';

pub fn grep_command() -> App<'static> {
    App::new(GREP)
        .about("Searches for lines that match a query")
        .arg(
            Arg::new(QUERY)
                .required(true)
                .help("The query to search for"),
        )
        .arg(
            Arg::new(FILES)
                .required(true)
                .help("The files to search")
                .min_values(0),
        )
        .arg(
            Arg::new(CASE_INSENSITIVE_L)
                .long(CASE_INSENSITIVE_L)
                .short(CASE_INSENSITIVE_S)
                .required(false)
                .help("Search for the query string regardless of type-case"),
        )
}

pub fn grep_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let case_insensitive = matches.is_present(CASE_INSENSITIVE_L);

    let query = match matches.value_of(QUERY) {
        None => {
            return Err(Box::new(ArgumentError::new(String::from(
                "Please provide a query",
            ))));
        }
        Some(query) => query,
    };

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

    search_readers(readers, query, case_insensitive)
}
