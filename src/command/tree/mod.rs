use std::error::Error;
use std::path::Path;

use clap::{App, Arg, ArgMatches};

use crate::command::tree::lib::show_directory_tree;
use crate::command::CURRENT_DIRECTORY;

mod lib;

pub const TREE: &str = "tree";
const DIR: &str = "dir";
const ALL_S: char = 'a';
const ALL_L: &str = "all";

pub fn tree_command() -> App<'static> {
    App::new(TREE)
        .about("Shows a file tree of a directory")
        .arg(
            Arg::new(DIR)
                .required(false)
                .min_values(0)
                .max_values(1)
                .default_value(CURRENT_DIRECTORY)
                .help("The directory to show the file tree for"),
        )
        .arg(
            Arg::new(ALL_L)
                .short(ALL_S)
                .long(ALL_L)
                .help("Display all of the directory contents (including hidden files)")
                .required(false),
        )
}

pub fn tree_main(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let show_hidden_files = matches.is_present(ALL_L);
    let dirname = match matches.value_of(DIR) {
        None => CURRENT_DIRECTORY,
        Some(path) => path,
    };

    let path = Path::new(dirname);
    let directory = std::fs::read_dir(path)?;
    show_directory_tree(directory, dirname, show_hidden_files)
}
