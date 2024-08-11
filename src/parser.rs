use std::fmt::Display;

use crate::{expr::Expr, lexer::Token};

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<ParseError>,
    current: usize,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            errors: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Token::True | Token::False | Token::Nil | Token::Number(_) | Token::String(_) => {
                Ok(Expr::Literal(self.advance()))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?; // consume interior expr
                if !self.matches(Token::RightParen) {
                    return Err(ParseError {
                        message: "Unmatched parentheses.".to_string(),
                    });
                }
                self.advance();
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(ParseError {
                message: "missing expression.".to_string(),
            }),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.primary()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn matches(&self, expected: Token) -> bool {
        expected == *self.peek()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
