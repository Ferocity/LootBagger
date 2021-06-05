use simple_logger::SimpleLogger;
use clap::{App, Arg, AppSettings};
use core::workspace::fields::Workspace;
use std::path::Path;

fn main() {
    SimpleLogger::new().init().unwrap();

    let matches = App::new("LootCli")
        .version("0.1")
        .about("Command-Line tool to gather information during penetration tests and CTFs")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("workspace")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::new("create")
                .long("create")
                .value_name("DIRECTORY")
                .about("Create a new workspace in selected directory")))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("workspace") {
        if let Some(workspace) = matches.value_of("create") {
            let ws = Workspace::create(Path::new(workspace));
            match ws {
                Ok(_) => log::info!("Workspace created"),
                Err(e) => log::error!("{}", e),
            };
        } else {
            //println!()
        }
    }
}