use simple_logger::SimpleLogger;
use clap::{App, Arg, AppSettings};


fn main() {
    SimpleLogger::new().init().unwrap();

    let matches = App::new("LootCli")
        .version("0.1")
        .about("Command-Line tool to gather information during penetration tests and CTFs")
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
}