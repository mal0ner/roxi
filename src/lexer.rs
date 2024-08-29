use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};

use crate::EXIT_OK;

const EXIT_LEXICAL_ERROR: i32 = 65;

const LEFT_PAREN: char = '(';
const RIGHT_PAREN: char = ')';
const LEFT_BRACE: char = '{';
const RIGHT_BRACE: char = '}';
const COMMA: char = ',';
const DOT: char = '.';
const MINUS: char = '-';
const PLUS: char = '+';
const SEMICOLON: char = ';';
const SLASH: char = '/';
const STAR: char = '*';
const BANG: char = '!';
const BANG_EQUAL: &str = "!=";
const EQUAL: char = '=';
const EQUAL_EQUAL: &str = "==";
const GREATER: char = '>';
const GREATER_EQUAL: &str = ">=";
const LESS: char = '<';
const LESS_EQUAL: &str = "<=";
const AND: &str = "and";
const CLASS: &str = "class";
const ELSE: &str = "else";
const FALSE: &str = "false";
const FUN: &str = "fun";
const FOR: &str = "for";
const IF: &str = "if";
const NIL: &str = "nil";
const OR: &str = "or";
const PRINT: &str = "print";
const RETURN: &str = "return";
const SUPER: &str = "super";
const THIS: &str = "this";
const TRUE: &str = "true";
const VAR: &str = "var";
const WHILE: &str = "while";

lazy_static! {
    static ref SINGLE_CHAR_TOKENS: HashMap<char, Token> = {
        let mut m = HashMap::new();
        m.insert('(', Token::LeftParen);
        m.insert(')', Token::RightParen);
        m.insert('{', Token::LeftBrace);
        m.insert('}', Token::RightBrace);
        m.insert(',', Token::Comma);
        m.insert('.', Token::Dot);
        m.insert('-', Token::Minus);
        m.insert('+', Token::Plus);
        m.insert(';', Token::Semicolon);
        m.insert('*', Token::Star);
        m
    };
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("and", Token::And);
        m.insert("class", Token::Class);
        m.insert("else", Token::Else);
        m.insert("false", Token::False);
        m.insert("for", Token::For);
        m.insert("fun", Token::Fun);
        m.insert("if", Token::If);
        m.insert("nil", Token::Nil);
        m.insert("or", Token::Or);
        m.insert("print", Token::Print);
        m.insert("return", Token::Return);
        m.insert("super", Token::Super);
        m.insert("this", Token::This);
        m.insert("true", Token::True);
        m.insert("var", Token::Var);
        m.insert("while", Token::While);
        m
    };
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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
    Identifier(String),
    String(String),
    Number(String),

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

    // END
    Eof,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct LexError {
    line: usize,
    column: usize,
    message: String,
}

pub struct Lexer {
    source: String,
    token_type: Vec<Token>,
    errors: Vec<LexError>,
    start: usize, //token start offset
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            errors: Vec::new(),
            token_type: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<LexError>, i32) {
        let mut exit_code: i32 = 0;
        while !self.is_at_end() {
            self.start = self.current;
            exit_code = exit_code.max(self.scan_token());
        }

        self.token_type.push(Token::Eof);

        (self.token_type.clone(), self.errors.clone(), exit_code)
    }

    fn scan_token(&mut self) -> i32 {
        use Token::*;

        let mut exit_code: i32 = 0;
        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => exit_code = self.string(),
            '/' => {
                if self.match_char('/') {
                    self.advance_newline();
                } else {
                    self.token_type.push(Token::Slash)
                }
            }
            '!' => self.match_next_add('=', BangEqual, Bang),
            '=' => self.match_next_add('=', EqualEqual, Equal),
            '<' => self.match_next_add('=', LessEqual, Less),
            '>' => self.match_next_add('=', GreaterEqual, Greater),
            _ => {
                if let Some(token_type) = SINGLE_CHAR_TOKENS.get(&c) {
                    self.token_type.push(token_type.clone());
                } else if c.is_ascii_digit() {
                    exit_code = self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    exit_code = self.log_error(format!("Unexpected character: {}", c).as_str());
                }
            }
        }
        exit_code
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];
        let token_type = KEYWORDS
            .get(lexeme)
            .cloned()
            .unwrap_or(Token::Identifier("".to_string()));
        match token_type {
            Token::Identifier(_) => self.token_type.push(Token::Identifier(lexeme.to_string())),
            _ => self.token_type.push(token_type),
        }
    }

    fn number(&mut self) -> i32 {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            // catches -> 10.x1 where x is an invalid character in a number literal.
            // if !self.peek().is_ascii_digit() {
            //     return self.log_error("Expected digit after decimal point");
            // }
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // check invalid characters directly after otherwise valid number literal
        // if self.peek().is_alphabetic() || self.peek() == '_' {
        //     return self.log_error("Invalid character in number literal");
        // }

        let literal = self.source[self.start..self.current].to_string();
        self.token_type.push(Token::Number(literal));
        EXIT_OK
    }

    fn string(&mut self) -> i32 {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.log_error("Unterminated string.");
        }

        self.advance(); // closing "

        // trim quotes for literal
        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.token_type.push(Token::String(literal));
        EXIT_OK
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn match_next_add(&mut self, c: char, if_match: Token, if_not: Token) {
        if self.match_char(c) {
            self.token_type.push(if_match);
        } else {
            self.token_type.push(if_not);
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // return next character
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    // advance to first character in new line, or EOF
    fn advance_newline(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn log_error(&mut self, message: &str) -> i32 {
        self.errors.push(LexError {
            line: self.line,
            column: self.current,
            message: message.to_string(),
        });
        EXIT_LEXICAL_ERROR
    }
}

impl Token {
    fn token_type(&self) -> String {
        match self {
            Token::LeftParen => "LEFT_PAREN".to_string(),
            Token::RightParen => "RIGHT_PAREN".to_string(),
            Token::LeftBrace => "LEFT_BRACE".to_string(),
            Token::RightBrace => "RIGHT_BRACE".to_string(),
            Token::Comma => "COMMA".to_string(),
            Token::Dot => "DOT".to_string(),
            Token::Minus => "MINUS".to_string(),
            Token::Plus => "PLUS".to_string(),
            Token::Semicolon => "SEMICOLON".to_string(),
            Token::Slash => "SLASH".to_string(),
            Token::Star => "STAR".to_string(),
            Token::Bang => "BANG".to_string(),
            Token::BangEqual => "BANG_EQUAL".to_string(),
            Token::Equal => "EQUAL".to_string(),
            Token::EqualEqual => "EQUAL_EQUAL".to_string(),
            Token::Greater => "GREATER".to_string(),
            Token::GreaterEqual => "GREATER_EQUAL".to_string(),
            Token::Less => "LESS".to_string(),
            Token::LessEqual => "LESS_EQUAL".to_string(),
            Token::Identifier(_) => "IDENTIFIER".to_string(),
            Token::String(_) => "STRING".to_string(),
            Token::Number(_) => "NUMBER".to_string(),
            Token::And => "AND".to_string(),
            Token::Class => "CLASS".to_string(),
            Token::Else => "ELSE".to_string(),
            Token::False => "FALSE".to_string(),
            Token::Fun => "FUN".to_string(),
            Token::For => "FOR".to_string(),
            Token::If => "IF".to_string(),
            Token::Nil => "NIL".to_string(),
            Token::Or => "OR".to_string(),
            Token::Print => "PRINT".to_string(),
            Token::Return => "RETURN".to_string(),
            Token::Super => "SUPER".to_string(),
            Token::This => "THIS".to_string(),
            Token::True => "TRUE".to_string(),
            Token::Var => "VAR".to_string(),
            Token::While => "WHILE".to_string(),
            Token::Eof => "EOF".to_string(),
        }
    }

    pub fn lexeme(&self) -> String {
        match self {
            Token::LeftParen => LEFT_PAREN.to_string(),
            Token::RightParen => RIGHT_PAREN.to_string(),
            Token::LeftBrace => LEFT_BRACE.to_string(),
            Token::RightBrace => RIGHT_BRACE.to_string(),
            Token::Comma => COMMA.to_string(),
            Token::Dot => DOT.to_string(),
            Token::Minus => MINUS.to_string(),
            Token::Plus => PLUS.to_string(),
            Token::Semicolon => SEMICOLON.to_string(),
            Token::Slash => SLASH.to_string(),
            Token::Star => STAR.to_string(),
            Token::Bang => BANG.to_string(),
            Token::BangEqual => BANG_EQUAL.to_string(),
            Token::Equal => EQUAL.to_string(),
            Token::EqualEqual => EQUAL_EQUAL.to_string(),
            Token::Greater => GREATER.to_string(),
            Token::GreaterEqual => GREATER_EQUAL.to_string(),
            Token::Less => LESS.to_string(),
            Token::LessEqual => LESS_EQUAL.to_string(),
            Token::Identifier(identifier) => identifier.to_string(),
            Token::String(string) => format!("\"{}\"", string),
            Token::Number(number) => number.to_string(),
            Token::And => AND.to_string(),
            Token::Class => CLASS.to_string(),
            Token::Else => ELSE.to_string(),
            Token::False => FALSE.to_string(),
            Token::Fun => FUN.to_string(),
            Token::For => FOR.to_string(),
            Token::If => IF.to_string(),
            Token::Nil => NIL.to_string(),
            Token::Or => OR.to_string(),
            Token::Print => PRINT.to_string(),
            Token::Return => RETURN.to_string(),
            Token::Super => SUPER.to_string(),
            Token::This => THIS.to_string(),
            Token::True => TRUE.to_string(),
            Token::Var => VAR.to_string(),
            Token::While => WHILE.to_string(),
            Token::Eof => "".to_string(),
        }
    }

    pub fn literal(&self) -> String {
        match self {
            Token::String(string) => string.to_string(),
            Token::Number(number) => format!("{:?}", number.parse::<f64>().unwrap()),
            _ => "null".to_string(),
        }
    }

    // pub fn literal_trimmed(&self) -> String {
    //     match self {
    //         Token::String(string) => string.to_string(),
    //         Token::Number(number) => format!("{}", number.parse::<f64>().unwrap()),
    //         _ => "null".to_string(),
    //     }
    // }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type(),
            self.lexeme(),
            self.literal()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_empty_input() {
        let input = String::from("");
        let mut lexer = Lexer::new(input);
        let (tokens, errors, _) = lexer.scan_tokens();
        assert!(errors.is_empty());
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_valid_tokens() {
        let input = String::from(")");
        let mut lexer = Lexer::new(input);
        let (tokens, errors, _) = lexer.scan_tokens();
        assert!(errors.is_empty());
        assert_eq!(tokens, vec![Token::RightParen, Token::Eof]);
    }
}
