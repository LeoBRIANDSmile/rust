use std::env;                                            // in order to use args fonction of env
use std::process;

use minigrep::Config;

fn main() {

    // Accepting command line arguments
    let args: Vec<String> = env::args().collect();      // Stocke les arguments donnés au cargo run dans un tableau de string

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for '{}' in file {}",config.query, config.file_path);

    // Reading a file
    // config.run();
    // // or
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
