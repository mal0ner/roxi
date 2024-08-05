use core::panic;
use std::env;
use std::fmt::Display;
use std::fs;
// use std::io::{self, Write};

const EXIT_LEXICAL_ERROR: i32 = 65;

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
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::LeftBrace => write!(f, "LEFT_BRACE"),
            Self::RightBrace => write!(f, "RIGHT_BRACE"),
            Self::Star => write!(f, "STAR"),
            Self::Dot => write!(f, "DOT"),
            Self::Comma => write!(f, "COMMA"),
            Self::Plus => write!(f, "PLUS"),
            Self::Minus => write!(f, "MINUS"),
            Self::Semicolon => write!(f, "SEMICOLON"),
            Self::Slash => write!(f, "SLASH"),
            Self::Equal => write!(f, "EQUAL"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Self::Bang => write!(f, "BANG"),
            Self::BangEqual => write!(f, "BANG_EQUAL"),
            Self::Eof => write!(f, "EOF"),
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

#[allow(dead_code)]
struct LexError {
    line: usize,
    column: usize,
    message: String,
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

enum LexItem {
    Token(Token),
    Error(LexError),
}

impl Display for LexItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexItem::Token(token) => write!(f, "{}", token),
            LexItem::Error(error) => write!(f, "{}", error),
        }
    }
}

fn scan(content: String) -> (Vec<LexItem>, i32) {
    let lines: Vec<&str> = content.split("\n").collect();
    let mut exit_code: i32 = 0;
    let mut items: Vec<LexItem> = Vec::new();

    for (line_nr, line) in lines.iter().enumerate() {
        let mut lookahead = String::new();
        let mut chars = line.chars().peekable();
        let mut col = 0;

        while let Some(c) = chars.next() {
            lookahead.push(c);

            let token_type = match lookahead.as_str() {
                "(" => Some(TokenType::LeftParen),
                ")" => Some(TokenType::RightParen),
                "{" => Some(TokenType::LeftBrace),
                "}" => Some(TokenType::RightBrace),
                "*" => Some(TokenType::Star),
                "." => Some(TokenType::Dot),
                "," => Some(TokenType::Comma),
                "+" => Some(TokenType::Plus),
                "-" => Some(TokenType::Minus),
                ";" => Some(TokenType::Semicolon),
                "/" => Some(TokenType::Slash),
                "=" => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        lookahead.push('=');
                        Some(TokenType::EqualEqual)
                    }
                    _ => Some(TokenType::Equal),
                },
                "!" => match chars.peek() {
                    Some('=') => {
                        chars.next();
                        lookahead.push('=');
                        Some(TokenType::BangEqual)
                    }
                    _ => Some(TokenType::Bang),
                },
                _ => None,
            };

            if let Some(token) = token_type {
                items.push(LexItem::Token(Token::new(
                    token,
                    lookahead,
                    line_nr + 1,
                    col,
                )));
                lookahead = String::new();
            } else {
                items.push(LexItem::Error(LexError {
                    line: line_nr + 1,
                    column: col,
                    message: format!("Unexpected character: {}", c),
                }));
                exit_code = EXIT_LEXICAL_ERROR;
                lookahead = String::new();
            }

            col += 1;
        }
        // for (col, c) in line.chars().enumerate() {
        //     lookahead.push(c);
        //
        //     let token_type = match lookahead.as_str() {
        //         "(" => Some(TokenType::LeftParen),
        //         ")" => Some(TokenType::RightParen),
        //         "{" => Some(TokenType::LeftBrace),
        //         "}" => Some(TokenType::RightBrace),
        //         "*" => Some(TokenType::Star),
        //         "." => Some(TokenType::Dot),
        //         "," => Some(TokenType::Comma),
        //         "+" => Some(TokenType::Plus),
        //         "-" => Some(TokenType::Minus),
        //         ";" => Some(TokenType::Semicolon),
        //         "/" => Some(TokenType::Slash),
        //         _ => None,
        //     };
        //
        //     if let Some(token) = token_type {
        //         items.push(LexItem::Token(Token::new(
        //             token,
        //             lookahead,
        //             line_nr + 1,
        //             col,
        //         )));
        //         lookahead = String::new();
        //     } else {
        //         items.push(LexItem::Error(LexError {
        //             line: line_nr + 1,
        //             column: col,
        //             message: format!("Unexpected character: {}", c),
        //         }));
        //         exit_code = EXIT_LEXICAL_ERROR;
        //         lookahead = String::new();
        //     }
        // }
    }

    items.push(LexItem::Token(Token::new(
        TokenType::Eof,
        "".to_string(),
        0,
        0,
    )));

    (items, exit_code)
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

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });
            let (items, exit_code) = scan(file_contents);
            for item in items {
                match item {
                    LexItem::Token(_) => println!("{}", item),
                    LexItem::Error(_) => eprintln!("{}", item),
                }
            }
            std::process::exit(exit_code);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
