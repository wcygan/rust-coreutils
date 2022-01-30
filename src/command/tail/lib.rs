use std::error::Error;
use std::io::BufRead;

pub fn print_tail(
    mut reader: Box<dyn BufRead>,
    n: usize,
    follow: bool,
) -> Result<(), Box<dyn Error>> {
    let mut lines = vec![];
    for line in reader.as_mut().lines() {
        let line = line?;
        lines.push(line)
    }

    let size = lines.len();
    for (idx, line) in lines.iter().enumerate() {
        if idx < size - n {
            continue;
        }

        println!("{}", line)
    }

    // if follow {
    //     follow_reader(&mut reader)?
    // }

    Ok(())
}

fn follow_reader(reader: &mut Box<dyn BufRead>) -> Result<(), Box<dyn Error>> {
    todo!("Implement follow")
}
