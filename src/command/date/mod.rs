use std::error::Error;

use clap::{App, ArgMatches};

use crate::command::date::lib::print_date;

mod lib;

pub const DATE: &str = "date";

pub fn date_command() -> App<'static> {
    App::new(DATE).about("Print the system date and time")
}

pub fn date_main(_: &ArgMatches) -> Result<(), Box<dyn Error>> {
    print_date()
}
