mod wave_reader;
use crate::wave_reader::*;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cfg = parse_args(&args).unwrap_or_else(|msg| {
        eprintln!("Arg parsing error: {}", msg);
        std::process::exit(1);
    });

    if let Err(e) = run(&cfg) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
