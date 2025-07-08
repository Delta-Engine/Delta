use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>.de", args[0]);
        process::exit(0);
    }

    let filename = &args[1];

    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprint!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };
}
