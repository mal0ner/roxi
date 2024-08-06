mod lexer;

use std::env;
use std::fs;

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
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
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
