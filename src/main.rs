mod compiler;
mod lexer;
mod parser;

use inkwell::context::Context;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        process::exit(1);
    }

    let source_path = &args[1];
    if !source_path.ends_with(".tr") {
        eprintln!("Error: Source file must have .tr extension");
        process::exit(1);
    }

    let source = fs::read_to_string(source_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    let context = Context::create();
    let mut compiler = compiler::Compiler::new(&context, "turing_module");

    let lexer = lexer::Lexer::new(&source);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_program();

    if let Err(err) = compiler.compile(ast) {
        eprintln!("Compilation error: {}", err);
        process::exit(1);
    }

    let module = compiler.get_module();
    if let Err(err) = module.verify() {
        eprintln!("Module verification error: {}", err);
        process::exit(1);
    }

    if let Err(err) = module.print_to_file("output.ll") {
        eprintln!("Error writing LLVM IR to file: {}", err);
        process::exit(1);
    }
}
