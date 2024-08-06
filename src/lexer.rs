use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};

const EXIT_LEXICAL_ERROR: i32 = 65;

lazy_static! {
    static ref SINGLE_CHAR_TOKENS: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('(', TokenType::LeftParen);
        m.insert(')', TokenType::RightParen);
        m.insert('{', TokenType::LeftBrace);
        m.insert('}', TokenType::RightBrace);
        m.insert(',', TokenType::Comma);
        m.insert('.', TokenType::Dot);
        m.insert('-', TokenType::Minus);
        m.insert('+', TokenType::Plus);
        m.insert(';', TokenType::Semicolon);
        m.insert('*', TokenType::Star);
        m
    };
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
    column: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TokenType {
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct LexError {
    line: usize,
    column: usize,
    message: String,
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
    start: usize, //token start offset
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            errors: Vec::new(),
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

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            None,
            self.line,
            self.current,
        ));

        (self.tokens.clone(), self.errors.clone(), exit_code)
    }

    fn scan_token(&mut self) -> i32 {
        use TokenType::*;

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
                    self.add_token(Slash, None)
                }
            }
            '!' => self.match_next_add('=', BangEqual, Bang),
            '=' => self.match_next_add('=', EqualEqual, Equal),
            '<' => self.match_next_add('=', LessEqual, Less),
            '>' => self.match_next_add('=', GreaterEqual, Greater),
            _ => {
                if let Some(token_type) = SINGLE_CHAR_TOKENS.get(&c) {
                    self.add_token(token_type.clone(), None);
                } else if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.errors.push(LexError {
                        line: self.line,
                        column: self.current,
                        message: format!("Unexpected character: {}", c),
                    });
                    exit_code = EXIT_LEXICAL_ERROR;
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
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type, None);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        //TODO: Add Lex error for invalid tokens within a number?

        let mut literal = self.source[self.start..self.current].to_string();
        let number: f64 = literal.parse().unwrap_or(0.0);
        literal = format!("{:?}", number);
        self.add_token(TokenType::Number, Some(literal));
    }

    fn string(&mut self) -> i32 {
        let mut exit_code = 0;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LexError {
                line: self.line,
                column: self.current,
                message: "Unterminated string.".to_string(),
            });
            exit_code = EXIT_LEXICAL_ERROR;

            return exit_code; // exit without advancing on unterminated str
        }

        self.advance(); // closing "

        // trim quotes for literal
        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(literal));
        exit_code
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn match_next_add(&mut self, c: char, if_match: TokenType, if_not: TokenType) {
        if self.match_char(c) {
            self.add_token(if_match, None);
        } else {
            self.add_token(if_not, None);
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

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn advance_newline(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(
            token_type, lexeme, literal, self.line, self.start,
        ));
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = self.literal.as_deref().unwrap_or("null");
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
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
            Self::Less => write!(f, "LESS"),
            Self::LessEqual => write!(f, "LESS_EQUAL"),
            Self::Greater => write!(f, "GREATER"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL"),
            Self::String => write!(f, "STRING"),
            Self::Number => write!(f, "NUMBER"),
            Self::Identifier => write!(f, "IDENTIFIER"),
            Self::And => write!(f, "AND"),
            Self::Class => write!(f, "CLASS"),
            Self::Else => write!(f, "ELSE"),
            Self::False => write!(f, "FALSE"),
            Self::Fun => write!(f, "FUN"),
            Self::For => write!(f, "FOR"),
            Self::If => write!(f, "IF"),
            Self::Nil => write!(f, "NIL"),
            Self::Or => write!(f, "OR"),
            Self::Print => write!(f, "PRINT"),
            Self::Return => write!(f, "RETURN"),
            Self::Super => write!(f, "SUPER"),
            Self::This => write!(f, "THIS"),
            Self::True => write!(f, "TRUE"),
            Self::Var => write!(f, "VAR"),
            Self::While => write!(f, "WHILE"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
