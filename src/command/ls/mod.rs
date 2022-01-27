use std::error::Error;
use std::path::Path;

use clap::{App, Arg, ArgMatches};

use crate::command::ls::lib::list_directory_contents;
use crate::command::{NotFoundError, CURRENT_DIRECTORY};

pub mod lib;

pub const LS: &str = "ls";
const DIR: &str = "dir";
const LONG_S: char = 'l';
const LONG_L: &str = "long";

pub fn ls_command() -> App<'static> {
    App::new(LS)
        .about("Echos the provided text")
        .arg(
            Arg::new(DIR)
                .required(false)
                .min_values(0)
                .max_values(1)
                .default_value(CURRENT_DIRECTORY)
                .help("The directory to list the contents of"),
        )
        .arg(
            Arg::new(LONG_L)
                .short(LONG_S)
                .long(LONG_L)
                .help("Display the directory contents in a column")
                .required(false),
        )
}

pub fn ls_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let dir = match matches.value_of(DIR) {
        None => CURRENT_DIRECTORY,
        Some(dir) => dir,
    };

    let path = std::fs::read_dir(Path::new(dir))?;
    let writer = Box::new(std::io::stdout());
    let long_format = matches.is_present(LONG_L);
    list_directory_contents(path, writer, long_format)
}
