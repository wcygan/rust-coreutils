use clap::{App, AppSettings};

use crate::command::echo::{echo_command, echo_main, ECHO};

mod command;

fn main() {
    let matches = App::new("rust-coreutils")
        .version("0.1.0")
        .author("Will C. <wcygan.io@gmail.com>")
        .about("A Rust implementation of gnu-coreutils programs")
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(echo_command())
        .get_matches();

    let result = match matches.subcommand() {
        Some((ECHO, sub_matches)) => echo_main(sub_matches),
        _ => unreachable!(),
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
