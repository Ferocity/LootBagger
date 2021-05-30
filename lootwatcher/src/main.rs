use simple_logger::SimpleLogger;
use clap::{App, Arg, AppSettings};

use core::workspace::fields::Workspace;
use std::path::Path;

fn main() {
    SimpleLogger::new().init().unwrap();

    let matches = App::new("LootWatcher")
        .version("0.1")
        .about("Command-Line tool to display information useful for penetration testing or during CTFs")
        .arg(Arg::new("WORKSPACE")
            .about("Path to Workspace file which should be loaded")
            .required(true)
            )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let ws = Workspace::load(Path::new("workspace.yaml"));
}