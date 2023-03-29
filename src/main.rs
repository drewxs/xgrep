use std::env;
use std::process;

use xgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Failed to parsed arguments: {err}");
        process::exit(1);
    });
    println!("query: {}", config.query);
    println!("path: {}", config.file_path);

    if let Err(e) = xgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
