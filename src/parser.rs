use std::fmt::Display;

use crate::{
    expr::Expr,
    lexer::{Token, TokenType},
};

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<ParseError>,
    current: usize,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    NonFatal(String),
    Fatal(String),
}

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            errors: Vec::new(),
            current: 0,
        }
    }

    /// Returns either a vec of parsed expressions and non-fatal/skippable errors, or a single fatal error
    /// that would cause a panic.
    pub fn parse(&mut self) -> Result<(Vec<Expr>, Vec<ParseError>), ParseError> {
        let mut errors: Vec<ParseError> = Vec::new();
        let mut exprs: Vec<Expr> = Vec::new();
        if self.tokens.is_empty() {
            let error = ParseError::Fatal("no tokens in token stream".to_string());
            errors.push(error.clone());
            return Err(error.clone());
        }

        let tokens = self.tokens.iter().peekable();

        for token in tokens {
            let expr: Result<Expr, ParseError> = match token.token_type {
                TokenType::True => Ok(Expr::Bool(true)),
                TokenType::False => Ok(Expr::Bool(false)),
                TokenType::Nil => Ok(Expr::Nil),
                TokenType::Number => self.number(token),
                TokenType::Eof => return Ok((exprs, errors)),
                _ => todo!(),
            };

            match expr {
                Ok(x) => exprs.push(x),
                Err(e) => {
                    // I have failed...
                    // but how badly...
                    match e {
                        ParseError::NonFatal(_) => errors.push(e),
                        // Exit early (PANIC) upon fatal/unrecoverable error
                        ParseError::Fatal(_) => {
                            return Err(e);
                        }
                    }
                }
            }
        }

        Ok((exprs, errors))
    }

    fn number(&self, token: &Token) -> Result<Expr, ParseError> {
        match parse_number_from_tok_literal(token.literal.clone()) {
            Ok(n) => Ok(Expr::Number(n)),
            Err(e) => Err(e),
        }
    }
}

fn parse_number_from_tok_literal(number: Option<String>) -> Result<f64, ParseError> {
    match number {
        Some(s) => s
            .parse::<f64>()
            .map_err(|_| ParseError::NonFatal("invalid number literal".to_string())),
        None => Err(ParseError::NonFatal("missing number literal".to_string())),
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NonFatal(msg) => write!(f, "Non-fatal parse error: {}", msg),
            ParseError::Fatal(msg) => write!(f, "Fatal Parse Error: {}", msg),
        }
    }
}
