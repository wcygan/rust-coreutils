mod command;

use crate::command::echo::{echo_command, echo_main, ECHO};
use clap::{App, AppSettings};

fn main() {
    let matches = App::new("rust-coreutils")
        .version("0.1.0")
        .author("Will C. <wcygan.io@gmail.com>")
        .about("A Rust implementation of gnu-coreutils programs")
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(echo_command())
        .get_matches();

    match matches.subcommand() {
        Some((ECHO, sub_matches)) => {
            echo_main(sub_matches);
        }
        _ => unreachable!(),
    }
}
