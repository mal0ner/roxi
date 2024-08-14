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
        if matches!(
            self.peek(),
            Token::True | Token::False | Token::Nil | Token::Number(_) | Token::String(_)
        ) {
            return Ok(Expr::Literal(self.advance()));
        }
        if matches!(self.peek(), Token::LeftParen) {
            self.advance();
            let expr = self.expression()?; // consume interior expr
            if !self.matches(Token::RightParen) {
                return Err(ParseError {
                    message: "Unmatched parentheses.".to_string(),
                });
            }
            self.advance();
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError {
            message: "missing expression".to_string(),
        })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comparison()
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.peek(), Token::Minus | Token::Bang) {
            let operator = self.advance();
            let rhs = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(rhs),
            });
        }
        self.primary()
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while matches!(
            self.peek(),
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual
        ) {
            let operator = self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while matches!(self.peek(), Token::Slash | Token::Star) {
            let operator = self.advance();
            let right = self.unary()?;
            expr = Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
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
