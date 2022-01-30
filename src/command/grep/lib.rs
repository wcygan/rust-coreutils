use std::error::Error;
use std::io::{BufRead, Read};

pub fn search_readers(
    readers: Vec<Box<dyn BufRead>>,
    query: &str,
    case_insensitive: bool,
) -> Result<(), Box<dyn Error>> {
    for mut reader in readers {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let matches = match case_insensitive {
            true => search_case_insensitive(query, &text),
            false => search(query, &text),
        };

        for line in matches {
            println!("{}", line)
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
