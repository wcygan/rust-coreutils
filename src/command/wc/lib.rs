use std::error::Error;
use std::io::{Read, Write};
use std::path::Path;

use clap::ArgMatches;
use rayon::prelude::*;

use crate::command::wc::{BYTES_L, LINES_L, WORDS_L};
use crate::command::NotFoundError;

///
/// Modifiers that determine the what is shown to the user
///
pub struct WordCountModifiers {
    pub lines: bool,
    pub words: bool,
    pub bytes: bool,
}

impl WordCountModifiers {
    ///
    /// Parse the arguments to determine which modifiers to use
    ///
    pub fn parse(matches: &ArgMatches) -> WordCountModifiers {
        WordCountModifiers {
            lines: matches.is_present(LINES_L),
            words: matches.is_present(WORDS_L),
            bytes: matches.is_present(BYTES_L),
        }
    }
}

///
/// Holds information about the word count of text
///
#[derive(Debug)]
pub struct ParsedResult<'a> {
    filepath: Option<&'a str>,
    lines: u64,
    words: u64,
    bytes: u64,
}

impl<'a> ParsedResult<'_> {
    ///
    /// Parse a file into text and then parse the text
    ///
    pub fn parse_file(filepath: &'a str) -> ParsedResult {
        let text = std::fs::read_to_string(&filepath).unwrap();
        ParsedResult::parse_lines(text, Some(filepath))
    }

    ///
    /// Parse text into word count information
    ///
    pub fn parse_lines(text: String, filepath: Option<&'a str>) -> ParsedResult<'a> {
        let (lines, words) = text
            .lines()
            .map(|l| {
                let words = l.split_ascii_whitespace().count();
                (1 as u64, words as u64)
            })
            .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        ParsedResult {
            filepath,
            lines,
            words,
            bytes: text.bytes().count() as u64,
        }
    }

    ///
    /// Merge the results of multiple ParsedResults into one
    ///
    pub fn totals(results: &Vec<ParsedResult>) -> ParsedResult<'a> {
        let (lines, words, bytes) = results.iter().fold((0, 0, 0), |a, b| {
            (a.0 + b.lines, a.1 + b.words, a.2 + b.bytes)
        });

        ParsedResult {
            filepath: Some("total"),
            lines,
            words,
            bytes,
        }
    }

    ///
    /// Prints all of the required results
    ///
    pub fn print(
        &self,
        mods: &WordCountModifiers,
        mut writer: Box<dyn Write>,
    ) -> Result<(), Box<dyn Error>> {
        let all = !mods.lines && !mods.words && !mods.bytes;
        let name = self.filepath.unwrap_or("");
        let results = format!(
            "{}{}{}{}\r\n",
            ParsedResult::fmt_result(self.lines, mods.lines, all),
            ParsedResult::fmt_result(self.words, mods.words, all),
            ParsedResult::fmt_result(self.bytes, mods.bytes, all),
            name
        );

        match writer.write(results.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    ///
    /// Formats a result
    ///
    fn fmt_result(v: u64, one: bool, all: bool) -> String {
        match one || all {
            true => format!("{:<10} ", v),
            false => String::from(""),
        }
    }
}

///
/// Take a list of files and count their contents
///
pub fn read_files(mut paths: Vec<&str>) -> Result<Vec<ParsedResult>, Box<dyn Error>> {
    for path in &paths {
        let fp = Path::new(path);
        if !fp.exists() && !fp.is_file() {
            return Err(Box::new(NotFoundError::new(format!(
                "{} is not a file",
                path
            ))));
        }
    }

    let results = paths
        .par_iter_mut()
        .map(|path| ParsedResult::parse_file(path))
        .collect();

    Ok(results)
}

///
/// Count the contents of stdin
///
pub fn read_stdin() -> Result<ParsedResult<'static>, Box<dyn Error>> {
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text)?;
    let result = ParsedResult::parse_lines(text, None);
    Ok(result)
}

///
/// Prints all of the ParsedResults to stdout
///
pub fn print_results(
    results: Vec<ParsedResult>,
    modifiers: WordCountModifiers,
) -> Result<(), Box<dyn Error>> {
    let totals = ParsedResult::totals(&results);

    for r in &results {
        r.print(&modifiers, Box::new(std::io::stdout()))?
    }

    if results.len() > 1 {
        totals.print(&modifiers, Box::new(std::io::stdout()))?
    }

    Ok(())
}
