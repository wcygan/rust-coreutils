use std::error::Error;
use std::io::BufRead;

pub fn print_first_n_lines(
    filename: Option<&str>,
    reader: Box<dyn BufRead>,
    n: u32,
) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = filename {
        let fmt = format!("==> {} <==", filename);
        println!("{}", fmt)
    }

    for (idx, line) in reader.lines().enumerate() {
        if idx as u32 == n {
            break;
        }

        println!("{}", line?)
    }

    Ok(())
}
