mod wave_reader;
use crate::wave_reader::*;
use std::convert::TryFrom;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cfg = Config::try_from(args.as_slice()).unwrap_or_else(|msg| {
        eprintln!("Arg parsing error: {}", msg);
        std::process::exit(1);
    });

    if let Err(e) = run(&cfg) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
