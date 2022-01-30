use std::error::Error;
use std::io::BufRead;

pub fn prepend_line_numbers(readers: Vec<Box<dyn BufRead>>) -> Result<(), Box<dyn Error>> {
    let mut ct = 1;
    for reader in readers {
        for line in reader.lines() {
            println!("  {} {}", ct, line?);
            ct += 1;
        }
    }

    Ok(())
}
