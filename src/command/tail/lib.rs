use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

pub fn print_tail(
    mut reader: Box<dyn BufRead>,
    path: Option<&Path>,
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

    if follow {
        if let Some(path) = path {
            follow_reader(path)?
        }
    }

    Ok(())
}

fn follow_reader(path: &Path) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    let mut contents = fs::read_to_string(&path).unwrap();
    let mut pos = contents.len() as u64;

    print!("{}", contents);

    ctrlc::set_handler(move || {
        std::process::exit(0);
    })?;

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                let mut f = File::open(&path).unwrap();
                f.seek(SeekFrom::Start(pos)).unwrap();

                pos = f.metadata().unwrap().len();

                contents.clear();
                f.read_to_string(&mut contents).unwrap();

                print!("{}", contents);
            }
            Ok(_) => {}
            Err(err) => return Err(Box::new(err)),
        }
    }
}
