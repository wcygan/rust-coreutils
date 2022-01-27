use std::error::Error;
use std::fmt;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches};
use rayon::prelude::*;

use crate::command::wc::lib::ParsedResult;
use crate::command::{NoFilesProvidedError, NotFoundError};

mod lib;

pub const WC: &str = "wc";
const FILES: &str = "files";
const BYTES_S: char = 'b';
const BYTES_L: &str = "bytes";
const WORDS_S: char = 'w';
const WORDS_L: &str = "words";
const LINES_S: char = 'l';
const LINES_L: &str = "lines";

pub struct WordCountModifiers {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
}

impl WordCountModifiers {
    pub fn parse(matches: &ArgMatches) -> WordCountModifiers {
        WordCountModifiers {
            lines: matches.is_present(LINES_L),
            words: matches.is_present(WORDS_L),
            bytes: matches.is_present(BYTES_L),
        }
    }
}

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
        None => {
            let mut text = String::new();
            std::io::stdin().read_to_string(&mut text)?;
            let result = ParsedResult::parse_lines(text, None);
            vec![result]
        }
        Some(files) => {
            let mut paths: Vec<&str> = files.collect();

            for path in &paths {
                let fp = Path::new(path);
                if !fp.exists() && !fp.is_file() {
                    return Err(Box::new(NotFoundError::new(format!(
                        "{} is not a file",
                        path
                    ))));
                }
            }

            paths
                .par_iter_mut()
                .map(|path| ParsedResult::parse_file(path))
                .collect()
        }
    };

    print_results(results, modifiers);
    return Ok(());
}

fn print_results(
    results: Vec<ParsedResult>,
    modifiers: WordCountModifiers,
) -> Result<(), Box<dyn Error>> {
    let totals = ParsedResult::totals(&results);
    for r in &results {
        r.print(&modifiers, Box::new(std::io::stdout()));
    }

    if results.len() > 1 {
        totals.print(&modifiers, Box::new(std::io::stdout()));
    }

    Ok(())
}
