extern crate core;

use clap::{App, AppSettings};

use crate::command::cat::{cat_command, cat_main, CAT};
use crate::command::date::{date_command, date_main, DATE};
use crate::command::echo::{echo_command, echo_main, ECHO};
use crate::command::ls::{ls_command, ls_main, LS};
use crate::command::tree::{tree_command, tree_main, TREE};
use crate::command::wc::{wc_command, wc_main, WC};
use crate::command::yell::{yell_command, yell_main, YELL};
use crate::command::yes::{yes_command, yes_main, YES};

mod command;

fn main() {
    let matches = App::new("rust-coreutils")
        .version("0.1.0")
        .author("Will C. <wcygan.io@gmail.com>")
        .about("A Rust implementation of gnu-coreutils programs")
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(get_subcommands())
        .get_matches();

    if let Err(e) = match matches.subcommand() {
        Some((ECHO, sub_matches)) => echo_main(sub_matches),
        Some((LS, sub_matches)) => ls_main(sub_matches),
        Some((WC, sub_matches)) => wc_main(sub_matches),
        Some((TREE, sub_matches)) => tree_main(sub_matches),
        Some((YES, sub_matches)) => yes_main(sub_matches),
        Some((DATE, sub_matches)) => date_main(sub_matches),
        Some((CAT, sub_matches)) => cat_main(sub_matches),
        Some((YELL, sub_matches)) => yell_main(sub_matches),
        _ => unreachable!(),
    } {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn get_subcommands() -> Vec<App<'static>> {
    vec![
        echo_command(),
        ls_command(),
        wc_command(),
        tree_command(),
        yes_command(),
        date_command(),
        cat_command(),
        yell_command(),
    ]
}
