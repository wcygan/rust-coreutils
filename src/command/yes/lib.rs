use std::error::Error;





pub fn repeat(text: String) -> Result<(), Box<dyn Error>> {
    loop {
        println!("{}", text)
    }
}
