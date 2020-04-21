// auto matically looks for a file called lib in the same path
mod lib;
// Import all the things in lib
use lib::*;

fn main() {
    let args: Vec<String> = std::env::args().collect(); // remember this

    // If the unwrap fails do a closure where the thing is passed on as |msg"
    let cfg = parse_args(&args).unwrap_or_else(|msg| {
        eprintln!("Argument Error: {}", msg);
        std::process::exit(1);
    });

    // Keep the main.rs minimal by putting the actual app in the lib.rs file
    if let Err(e) = run(&cfg) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
