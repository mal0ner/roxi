mod eval;
mod expr;
mod lexer;
mod parser;

use std::env;
use std::fs;

use crate::{eval::Evaluator, lexer::Lexer, parser::Parser};

const EXIT_OK: i32 = 0;
const EXIT_LEX_ERROR: i32 = 65;
const EXIT_RUNTIME_ERROR: i32 = 70;

fn tokenize(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });
    let mut lex = Lexer::new(file_contents);
    let (tokens, errors, exit_code) = lex.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
    for error in errors {
        eprintln!("{}", error);
    }
    std::process::exit(exit_code);
}

fn parse(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    if !file_contents.is_empty() {
        let mut lex = Lexer::new(file_contents);
        let mut exit_code: i32 = EXIT_OK;
        let (tokens, _, exit_lexer) = lex.scan_tokens();
        exit_code = exit_code.max(exit_lexer);

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(expr) => {
                println!("{}", expr);
            }
            Err(e) => {
                exit_code = exit_code.max(EXIT_LEX_ERROR);
                eprintln!("{}", e);
            }
        }
        std::process::exit(exit_code);
    }
}

fn evaluate(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    if !file_contents.is_empty() {
        let mut lex = Lexer::new(file_contents);
        let mut exit_code: i32 = EXIT_OK;
        let (tokens, _, exit_lexer) = lex.scan_tokens();
        exit_code = exit_code.max(exit_lexer);

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(expr) => {
                let evaluator = Evaluator::new(Box::new(expr));
                match evaluator.evaluate() {
                    Ok(value) => println!("{}", value),
                    Err(e) => {
                        eprintln!("{}", e);
                        exit_code = exit_code.max(EXIT_RUNTIME_ERROR);
                    }
                }
            }
            Err(e) => {
                exit_code = exit_code.max(EXIT_LEX_ERROR);
                eprintln!("{}", e);
            }
        }
        std::process::exit(exit_code);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => tokenize(filename),
        "parse" => parse(filename),
        "evaluate" => evaluate(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
