use std::error::Error;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;

use crate::command::{NotFoundError, HIDDEN_FILE_PREFIX, SINGLE_SPACE, TRIPLE_SPACE};

const VERTICAL: &str = "│";
const SINGLE_HORIZONTAL: &str = "─";
const DOUBLE_HORIZONTAL: &str = "──";
const BRANCH: &str = "├";
const ENDING: &str = "└─";

pub fn show_directory_tree(
    directory: ReadDir,
    dirname: &str,
    show_hidden_files: bool,
) -> Result<(), Box<dyn Error>> {
    println!("{}", dirname);
    walk_directory(directory, show_hidden_files, vec![])
}

pub fn walk_directory(
    directory: ReadDir,
    show_hidden_files: bool,
    active_pipes: Vec<bool>,
) -> Result<(), Box<dyn Error>> {
    let mut entries: Vec<DirEntry> = vec![];
    for entry in directory {
        entries.push(entry?);
    }

    let num_entries = entries.len();
    for (idx, entry) in entries.iter().enumerate() {
        let entry_name = entry.file_name();
        let entry_name_str = match entry_name.to_str() {
            None => {
                return Err(Box::new(NotFoundError::new(String::from(
                    "tree error reading directory contents",
                ))));
            }
            Some(str) => str,
        };

        // skip hidden files if necessary
        if !show_hidden_files && entry_name_str.starts_with(HIDDEN_FILE_PREFIX) {
            continue;
        }

        let is_final_entry = idx == num_entries - 1;
        println!(
            "{}{}",
            pipe_prefix(is_final_entry, &active_pipes),
            entry_name_str,
        );

        let path_buf = entry.path();
        let path = path_buf.as_path();

        if path.is_dir() {
            let next_directory = std::fs::read_dir(Path::new(path))?;
            let mut next_bars = active_pipes.clone();
            next_bars.push(!is_final_entry);
            walk_directory(next_directory, show_hidden_files, next_bars)?
        }
    }

    Ok(())
}

fn pipe_prefix(is_final_entry: bool, active_pipes: &Vec<bool>) -> String {
    let mut buffer: Vec<&str> = vec![];

    for is_active in active_pipes {
        match is_active {
            true => buffer.push(VERTICAL),
            false => buffer.push(SINGLE_SPACE),
        };
        buffer.push(TRIPLE_SPACE)
    }

    match is_final_entry {
        true => {
            buffer.push(ENDING);
            buffer.push(SINGLE_HORIZONTAL);
        }
        false => {
            buffer.push(BRANCH);
            buffer.push(DOUBLE_HORIZONTAL);
        }
    };
    buffer.push(SINGLE_SPACE);

    buffer.join("")
}
