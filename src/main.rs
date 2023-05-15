use minigrep::Config;
use minigrep::Runner;

use std::env;
use std::process::exit;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error reading arguments: {}", err);
        exit(1);
    });

    if let Err(_) = Runner::run(&config) {
        eprintln!("Search error");
        exit(1);
    }
}

