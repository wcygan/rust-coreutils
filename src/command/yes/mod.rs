use std::error::Error;

use clap::{App, Arg, ArgMatches};

use crate::command::get_stdin;
use crate::command::yes::lib::repeat;

mod lib;

pub const YES: &str = "yes";
const TEXT: &str = "text";

pub fn yes_command() -> App<'static> {
    App::new(YES)
        .about("Repeats the provided text until interrupted")
        .arg(
            Arg::new(TEXT)
                .help("The text to be repeated")
                .required(false)
                .min_values(0),
        )
}

pub fn yes_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let text = match matches.values_of(TEXT) {
        None => get_stdin()?,
        Some(values) => values.collect::<Vec<&str>>().join(" "),
    };
    repeat(text)
}
