mod eval;
mod expr;
mod lexer;
mod parser;
mod position;

use std::env;
use std::fs;

use lexer::Scanner;
use lexer::Token;
use position::LineOffsets;
use position::WithSpan;

use crate::{eval::Evaluator, parser::Parser};

fn tokenize(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });
    let mut scanner = Scanner::new(&file_contents);
    let offsets = LineOffsets::new(&file_contents);
    let tokens: Vec<Token> = scanner
        .scan()
        .into_iter()
        .map(WithSpan::into_inner)
        .collect();
    for token in tokens {
        println!("{}", token);
    }
    if scanner.has_errors() {
        let diagnostics = scanner.diagnostics();
        for diag in diagnostics {
            let line = offsets.line(diag.span.end);
            eprintln!("[line {}] Error: {}", line, &diag.message);
        }
    }
}

fn parse(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(&file_contents);
        let _offsets = LineOffsets::new(&file_contents);
        let tokens: Vec<Token> = scanner
            .scan()
            .into_iter()
            .map(WithSpan::into_inner)
            .collect();

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(expr) => {
                println!("{}", expr);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn evaluate(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(&file_contents);
        let _offsets = LineOffsets::new(&file_contents);
        let tokens: Vec<Token> = scanner
            .scan()
            .into_iter()
            .map(WithSpan::into_inner)
            .collect();

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(expr) => {
                let evaluator = Evaluator::new(Box::new(expr));
                match evaluator.evaluate() {
                    Ok(value) => println!("{}", value),
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
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
