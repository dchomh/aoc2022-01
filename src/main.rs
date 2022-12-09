use std::env;
use std::process;

use aoc2022_01::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = aoc2022_01::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

