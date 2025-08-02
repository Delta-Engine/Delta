use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod ast;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use codegen::CodeGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <source_file.de> [--compile|--interpret]", args[0]);
        eprintln!("  --compile   : Generate LLVM IR and compile (default)");
        eprintln!("  --interpret : Run in interpreter mode");
        process::exit(1);
    }
    
    let filename = &args[1];

    let mode = if args.len() == 3 {
        &args[2]
    } else {
        "--compile"
    };

    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
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
    
    // println!("Tokens: {:?}", tokens);
    
    // Step 2: Parse into AST
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            process::exit(1);
        }
    };
    
    // Step 3: Print AST (debug output)
    // println!("AST: {:#?}", ast);
    
    // Step 4: For now, just interpret
    let mut codegen = CodeGenerator::new();
    if let Err(err) = codegen.interpret(&ast) {
        eprintln!("Interpreter error: {}", err);
        process::exit(1);
    }
}
