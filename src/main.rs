use std::env::*;
use std::process;

use minigdd::Config;

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(0);
    });

    if let Err(e) = minigdd::run(config) {
        println!("Application Error: {e}");
        process::exit(0);
    }
}
