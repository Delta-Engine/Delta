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
use inkwell::context::Context;

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
    
    // Step 1: Tokenize
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            process::exit(1);
        }
    };
    
    if mode == "--debug" {
        println!("Tokens: {:?}", tokens);
    }
    
    // Step 2: Parse into AST
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            process::exit(1);
        }
    };
    
    if mode == "--debug" {
        println!("AST: {:#?}", ast);
    }
    
    // Step 3: Execute based on mode
    match mode {
        "--interpret" => {
            println!("Running in interpreter mode...");
            let context = Context::create();
            let mut codegen = match CodeGenerator::new(&context, "delta_module") {
                Ok(cg) => cg,
                Err(err) => {
                    eprintln!("Failed to create code generator: {}", err);
                    process::exit(1);
                }
            };
            
            if let Err(err) = codegen.interpret(&ast) {
                eprintln!("Interpreter error: {}", err);
                process::exit(1);
            }
        }
        "--compile" | _ => {
            println!("Compiling to LLVM IR...");
            let context = Context::create();
            let mut codegen = match CodeGenerator::new(&context, "delta_module") {
                Ok(cg) => cg,
                Err(err) => {
                    eprintln!("Failed to create code generator: {}", err);
                    process::exit(1);
                }
            };
            
            if let Err(err) = codegen.compile(&ast) {
                eprintln!("Compilation error: {}", err);
                process::exit(1);
            }
            
            // Save LLVM IR to file
            let ir_filename = filename.replace(".de", ".ll");
            if let Err(err) = codegen.save_to_file(&ir_filename) {
                eprintln!("Failed to save LLVM IR: {}", err);
                process::exit(1);
            }
            
            println!("LLVM IR saved to: {}", ir_filename);
            println!("To compile to executable, run:");
            println!("  clang {} -o {}", ir_filename, filename.replace(".de", ""));
        }
    }
}
