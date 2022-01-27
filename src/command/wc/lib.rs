use std::borrow::BorrowMut;
use std::io::Write;
use std::rc::Rc;

use clap::ArgMatches;

use crate::command::wc::WordCountModifiers;
use crate::command::NEWLINE;

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
        let newline_byte_count = 2;

        let (lines, words, bytes) = text
            .lines()
            .map(|l| {
                let words = l.split_ascii_whitespace().count();
                let bytes = l.bytes().count() + newline_byte_count;
                (1 as u64, words as u64, bytes as u64)
            })
            .fold((0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));

        ParsedResult {
            filepath,
            lines,
            words,
            bytes,
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

    pub fn print(&self, mods: &WordCountModifiers, mut writer: Box<dyn Write>) {
        let all = !mods.lines && !mods.words && !mods.bytes;

        let l = match mods.lines || all {
            true => {
                format!("{:<10} ", self.lines)
            }
            false => "".to_string(),
        };

        let w = match mods.words || all {
            true => {
                format!("{:<10} ", self.words)
            }
            false => "".to_string(),
        };

        let c = match mods.bytes || all {
            true => {
                format!("{:<10} ", self.bytes)
            }
            false => "".to_string(),
        };

        let name = self.filepath.unwrap_or("");

        writer.write(format!("{}{}{}{}\n", l, w, c, name).as_bytes());
    }
}
