use std::process;

use minigrep::Config;

#[macro_use]
extern crate clap;

fn main() {
    let args = clap_app!(minigrep =>
        (version: "1.0")
        (author: "Peter Bell")
        (about: "Miniature version of grep.")
        (@arg INSENSITIVE: -i --insensitive "Search results are case-insensitive (this can also be activated by using CASE_INSENSITIVE=1)")
        (@arg QUERY: +required "Text to search in provided file")
        (@arg FILENAME: +required "File to search")
    ).get_matches();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

