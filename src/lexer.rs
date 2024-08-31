use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display, iter::Peekable, str::Chars};

use crate::position::{BytePos, Diagnostic, Span, WithSpan};

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

#[allow(unused)]
pub struct Scanner<'a> {
    pos: BytePos,
    it: Peekable<Chars<'a>>,
    errors: Vec<Diagnostic>,
}

#[allow(unused)]
impl<'a> Scanner<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            pos: BytePos::default(),
            it: data.chars().peekable(),
            errors: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Vec<WithSpan<Token>> {
        let mut tokens: Vec<WithSpan<Token>> = Vec::new();

        loop {
            let start_pos = self.pos;
            let ch = match self.next() {
                None => break,
                Some(c) => c,
            };

            match self.match_token(ch, start_pos) {
                Ok(maybe_token) => {
                    if let Some(token) = maybe_token {
                        tokens.push(WithSpan::new(
                            token,
                            Span {
                                start: start_pos,
                                end: self.pos,
                            },
                        ));
                    }
                    // dont do anything if \n, \t, \r, //, or ' '
                }
                Err(diag) => self.error(diag),
            }
        }
        // do stuff
        tokens.push(WithSpan::new(
            Token::Eof,
            Span {
                start: self.pos,
                end: self.pos,
            },
        ));
        tokens
    }

    fn match_token(&mut self, ch: char, start_pos: BytePos) -> Result<Option<Token>, Diagnostic> {
        use Token::*;

        match ch {
            ' ' | '\n' | '\r' | '\t' => Ok(None),
            '"' => {
                let s = self.consume_while(|ch| ch != '"');
                match self.next() {
                    None => Err(Diagnostic::new("Unterminated String", start_pos, self.pos)),
                    _ => Ok(Some(String(s))),
                }
            }
            '/' => {
                if self.consume_if(|ch| ch == '/') {
                    self.consume_while(|ch| ch != '\n');
                    Ok(None)
                } else {
                    Ok(Some(Slash))
                }
            }
            '!' => Ok(Some(self.either('=', BangEqual, Bang))),
            '=' => Ok(Some(self.either('=', EqualEqual, Equal))),
            '<' => Ok(Some(self.either('=', LessEqual, Less))),
            '>' => Ok(Some(self.either('=', GreaterEqual, Greater))),
            c if c.is_numeric() => Ok(self.number(c)),
            c if c.is_alphabetic() || c == '_' => Ok(self.identifier(c)),
            _ => {
                if let Some(tok) = SINGLE_CHAR_TOKENS.get(&ch) {
                    Ok(Some(tok.clone()))
                } else {
                    Err(Diagnostic::new(
                        format!("Unexpected character: {}", ch),
                        start_pos,
                        self.pos,
                    ))
                }
            }
        }
    }

    fn identifier(&mut self, ch: char) -> Option<Token> {
        let mut ident = String::new();
        ident.push(ch);
        let rest: String = self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
        ident.push_str(&rest);
        let keyword = KEYWORDS.get(ident.as_str());
        match keyword {
            Some(kw) => Some(kw.clone()),
            None => Some(Token::Identifier(ident)),
        }
    }

    fn number(&mut self, ch: char) -> Option<Token> {
        let mut number = String::new();
        number.push(ch);
        let pre_decimal: String = self.consume_while(|c| c.is_numeric());
        number.push_str(&pre_decimal);

        if self.peek() == Some(&'.') && self.consume_if_next(|c| ch.is_numeric()) {
            let post_decimal: String = self.consume_while(|c| c.is_numeric());
            number.push('.');
            number.push_str(&post_decimal);
        }
        Some(Token::Number(number))
    }

    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
        if self.consume_if(|ch| ch == to_match) {
            matched
        } else {
            unmatched
        }
    }

    /// Consume next char if it matches some condition.
    fn consume_if<CharMatchFn>(&mut self, matches: CharMatchFn) -> bool
    where
        CharMatchFn: Fn(char) -> bool,
    {
        if let Some(&ch) = self.peek() {
            if matches(ch) {
                self.next().unwrap(); // safe, we peeked some
                return true; // char matches
            } else {
                return false; // char doesn't match
            }
        }
        false // no current char to peek
    }

    /// Consume next char if character following it matches some condition
    fn consume_if_next<CharMatchFn>(&mut self, matches: CharMatchFn) -> bool
    where
        CharMatchFn: Fn(char) -> bool,
    {
        let mut iter_copy = self.it.clone();
        if iter_copy.next().is_none() {
            return false;
        }

        if let Some(&ch) = iter_copy.peek() {
            // dont progress main iter unecessarily
            if matches(ch) {
                self.next().unwrap(); // safe, we peeked some
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Consume characters until reaching some condition.
    fn consume_while<CharMatchFn>(&mut self, matches: CharMatchFn) -> String
    where
        CharMatchFn: Fn(char) -> bool,
    {
        let mut chars = String::new();
        while let Some(&ch) = self.peek() {
            if matches(ch) {
                self.next().unwrap(); // safe, we peeked some
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

    fn next(&mut self) -> Option<char> {
        let next = self.it.next();
        if let Some(c) = next {
            // handle possible non-ascii width char
            self.pos = self.pos.shift(c);
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    fn error(&mut self, e: Diagnostic) {
        self.errors.push(e);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.errors
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
