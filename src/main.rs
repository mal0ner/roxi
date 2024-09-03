mod eval;
mod expr;
mod lexer;
mod parser;
mod position;

use std::{env, fs};

use crate::{
    // eval::Evaluator,
    lexer::{Scanner, Token},
    parser::Parser,
    position::{LineOffsets, WithSpan},
};

fn tokenize(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(&file_contents);
        let offsets = LineOffsets::new(&file_contents);
        let tokens: Vec<Token> = scanner
            .scan()
            .into_iter()
            .map(WithSpan::into_inner)
            .collect();
        if scanner.has_errors() {
            let diagnostics = scanner.diagnostics();
            for diag in diagnostics {
                let line = offsets.line(diag.span.end);
                eprintln!("[line {}] Error: {}", line, &diag.message);
            }
        }
        for token in tokens {
            println!("{}", token);
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
        let offsets = LineOffsets::new(&file_contents);
        let tokens: Vec<WithSpan<Token>> = scanner.scan().into_iter().collect();

        let mut parser = Parser::new(&tokens);

        match expr::parse(&mut parser) {
            Ok(ast) => {
                println!("{}", ast);
            }
            Err(_) => {
                for diag in parser.diagnostics() {
                    let line = offsets.line(diag.span.end);
                    eprintln!("[line {}] Error: {}", line, &diag.message);
                }
            }
        }
    }
}

// fn evaluate(filename: &str) {
//     let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
//         eprintln!("Failed to read file {}", filename);
//         String::new()
//     });
//
//     if !file_contents.is_empty() {
//         let mut scanner = Scanner::new(&file_contents);
//         let _offsets = LineOffsets::new(&file_contents);
//         let tokens: Vec<WithSpan<Token>> = scanner.scan().into_iter().collect();
//
//         let mut parser = Parser::new(&tokens);
//
//         match expr::parse(&mut parser) {
//             Ok(expr) => {
//                 let evaluator = Evaluator::new(Box::new(expr));
//                 match evaluator.evaluate() {
//                     Ok(value) => println!("{}", value),
//                     Err(e) => {
//                         eprintln!("{}", e);
//                     }
//                 }
//             }
//             Err(e) => {
//                 eprintln!("{}", e);
//             }
//         }
//     }
// }

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
        // "evaluate" => evaluate(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
