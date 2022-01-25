use clap::{arg, App, ArgMatches};

pub const ECHO: &str = "echo";

pub fn echo_command() -> App<'static> {
    App::new(ECHO)
        .about("Echos the provided text")
        .arg(arg!([NAME]))
}

pub fn echo_main(matches: &ArgMatches) {
    println!("'name is: {:?}", matches.value_of("NAME"));
    todo!("Not implemented")
}
