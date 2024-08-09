use crate::{
    expr::Expr,
    lexer::{Token, TokenType},
};

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Option<Vec<Expr>> {
        if self.tokens.is_empty() {
            return None;
        }

        let mut exprs = Vec::new();
        let tokens = self.tokens.iter().peekable();

        for token in tokens {
            let expr = match token.token_type {
                TokenType::True => Some(Expr::Bool(true)),
                TokenType::False => Some(Expr::Bool(false)),
                TokenType::Nil => Some(Expr::Nil),
                TokenType::Eof => return Some(exprs),
                _ => None,
            };

            exprs.push(expr?);
        }

        Some(exprs)
    }
}
