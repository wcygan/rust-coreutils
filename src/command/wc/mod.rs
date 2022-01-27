use std::error::Error;

use clap::{App, Arg, ArgMatches};

use crate::command::wc::lib::{
    print_results, read_files, read_stdin, ParsedResult, WordCountModifiers,
};

mod lib;

pub const WC: &str = "wc";
const FILES: &str = "files";
const BYTES_S: char = 'b';
const BYTES_L: &str = "bytes";
const WORDS_S: char = 'w';
const WORDS_L: &str = "words";
const LINES_S: char = 'l';
const LINES_L: &str = "lines";

pub fn wc_command() -> App<'static> {
    App::new(WC)
        .about("Print newline, word, and byte counts")
        .arg(
            Arg::new(FILES)
                .required(false)
                .min_values(1)
                .help("The files to count"),
        )
        .arg(
            Arg::new(BYTES_L)
                .short(BYTES_S)
                .long(BYTES_L)
                .help("Print the byte count"),
        )
        .arg(
            Arg::new(WORDS_L)
                .short(WORDS_S)
                .long(WORDS_L)
                .help("Print the word count"),
        )
        .arg(
            Arg::new(LINES_L)
                .short(LINES_S)
                .long(LINES_L)
                .help("Print the line count"),
        )
}

pub fn wc_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let modifiers = WordCountModifiers::parse(matches);

    let results: Vec<ParsedResult> = match matches.values_of(FILES) {
        None => vec![read_stdin()?],
        Some(files) => read_files(files.collect())?,
    };

    print_results(results, modifiers)
}
