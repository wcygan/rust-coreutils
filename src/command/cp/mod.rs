use std::error::Error;





use clap::{App, Arg, ArgMatches};

use crate::command::error::ArgumentError;


mod lib;

pub const CP: &str = "cp";
const SOURCE: &str = "source";
const DESTINATION: &str = "destination";

pub fn cp_command() -> App<'static> {
    App::new(CP)
        .about("Copies a given file to the destination path")
        .arg(Arg::new(SOURCE).required(true).help("The file to copy"))
        .arg(
            Arg::new(DESTINATION)
                .required(true)
                .help("The directory to copy the file to"),
        )
}

pub fn cp_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let src = matches
        .value_of(SOURCE)
        .ok_or(ArgumentError::new(String::from(
            "please provide a source file",
        )))?;

    let dst = matches
        .value_of(DESTINATION)
        .ok_or(ArgumentError::new(String::from(
            "please provide a destination directory",
        )))?;

    lib::copy(src, dst)
}
