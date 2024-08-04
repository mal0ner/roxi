use core::panic;
use std::env;
use std::fmt::Display;
use std::fs;
// use std::io::{self, Write};

#[allow(dead_code)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // 1/2 char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Identifiers
    Identifier,
    String,
    Number,

    // Keywords,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Eof => write!(f, "EOF"),
            _ => panic!("not implemented"),
        }
    }
}

#[allow(dead_code)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} null", self.token_type, self.lexeme)
    }
}

// fn tokenize(content: &str) {
//     for char in content.chars() {
//         match char {
//             '(' => println!("LEFT_PAREN ( null"),
//             ')' => println!("RIGHT_PAREN ) null"),
//             '{' => println!("LEFT_BRACE {{ null"),
//             '}' => println!("RIGHT_BRACE }} null"),
//             _ => {}
//         }
//     }
//     println!("EOF  null");
// }

fn scan(content: String) -> Vec<Token> {
    let lines: Vec<&str> = content.split("\n").collect();
    let mut tokens: Vec<Token> = Vec::new();

    for (line_nr, line) in lines.iter().enumerate() {
        let mut lookahead = String::new();

        for (col, c) in line.chars().enumerate() {
            lookahead.push(c);

            let token_type = match lookahead.as_str() {
                "(" => Some(TokenType::LeftParen),
                ")" => Some(TokenType::RightParen),
                "{" => Some(TokenType::LeftBrace),
                "}" => Some(TokenType::RightBrace),
                _ => None,
            };

            if let Some(token) = token_type {
                tokens.push(Token::new(token, lookahead, line_nr, col));
                lookahead = String::new();
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, "".to_string(), 0, 0));

    tokens
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
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });
            let tokens = scan(file_contents);
            for token in tokens {
                println!("{}", token);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
