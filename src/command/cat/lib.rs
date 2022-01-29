use std::error::Error;
use std::io::BufRead;

use crate::command::print_reader;

pub fn concatenate_readers(readers: Vec<Box<dyn BufRead>>) -> Result<(), Box<dyn Error>> {
    for reader in readers {
        print_reader(reader)?
    }
    Ok(())
}
