use std::error::Error;

use clap::{App, Arg, ArgMatches};

use crate::command::get_stdin_text;
use crate::command::yell::lib::yell;

mod lib;

pub const YELL: &str = "yell";
const TEXT: &str = "text";

pub fn yell_command() -> App<'static> {
    App::new(YELL)
        .about("Echos the provided text in uppercase")
        .arg(
            Arg::new(TEXT)
                .required(false)
                .help("The text to echo in uppercase")
                .min_values(0),
        )
}

pub fn yell_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let text = match matches.values_of(TEXT) {
        None => get_stdin_text()?,
        Some(values) => values.collect(),
    };

    yell(text);
    Ok(())
}
