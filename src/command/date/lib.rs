use std::error::Error;

const DATE_FORMAT: &str = "%a %b %e %T %Y";

pub fn print_date() -> Result<(), Box<dyn Error>> {
    println!(
        "{:?}",
        chrono::offset::Local::now().format(DATE_FORMAT).to_string()
    );
    Ok(())
}
