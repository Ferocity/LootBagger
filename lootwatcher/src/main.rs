use simple_logger::SimpleLogger;
use clap::{App, Arg, AppSettings};


fn main() {
    SimpleLogger::new().init().unwrap();

    let matches = App::new("LootWatcher")
        .version("0.1")
        .about("Command-Line tool to display information useful for penetration testing or during CTFs")
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
}