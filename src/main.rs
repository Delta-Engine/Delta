use std::env;
use std::fs;
use std::process;

mod lexer;

use lexer::Lexer;

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

    // Step 1: Tokenize (Lexer) 
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };
    
    println!("Tokens: {:?}", tokens);
}
