use std::env;
use std::process;

mod lib;

fn main() {
    let args :Vec<String> = env::args().collect();
    let args = lib::Config::new(&args).unwrap_or_else(|msg|{ // msg is the argument to the closure
        eprintln!("Unable to parse args: {}", msg);
        process::exit(1);
    });

    // Don't use unwrap_or_else cause i don't actually want to unwrap the result, 
    // I only care about the error, so if the result is an Err() type the let will work
    if let Err(err) = lib::run(&args) {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }

}
