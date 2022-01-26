use std::error::Error;

use clap::{App, Arg, ArgMatches};

use crate::command::echo::lib::{print_args, print_stdin};

mod lib;

pub const ECHO: &str = "echo";
const TEXT: &str = "text";

pub fn echo_command() -> App<'static> {
    App::new(ECHO)
        .about("Echos the provided text")
        .arg(Arg::new(TEXT).required(false).min_values(0))
}

pub fn echo_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let writer = Box::new(std::io::stdout());
    match matches.values_of(TEXT) {
        None => print_stdin(writer),
        Some(values) => print_args(values.collect(), writer),
    }
}
