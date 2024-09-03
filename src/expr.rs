use std::fmt::Display;

use crate::{
    lexer::Token,
    parser::Parser,
    position::{Span, WithSpan},
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Literal(WithSpan<Token>),
    Unary {
        operator: WithSpan<Token>,
        right: Box<WithSpan<Expr>>,
    },
    Binary {
        operator: WithSpan<Token>,
        left: Box<WithSpan<Expr>>,
        right: Box<WithSpan<Expr>>,
    },
    Grouping(Box<WithSpan<Expr>>),
}

impl Display for WithSpan<Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Expr::Literal(token) => match token.value {
                Token::Number(_) | Token::String(_) => write!(f, "{}", token.value.literal()),
                _ => write!(f, "{}", token.value.lexeme()),
            },
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.value.lexeme(), right)
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                write!(f, "({} {} {})", operator.value.lexeme(), left, right)
            }
            Expr::Grouping(g) => write!(f, "(group {})", g),
        }
    }
}

pub fn parse(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    expression(parser)
}

fn expression(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    equality(parser)
}

fn equality(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    let mut expr = comparison(parser)?;
    while matches!(parser.peek().unwrap(), Token::BangEqual | Token::EqualEqual) {
        // criminal behaviour again --^
        let operator = parser.advance();
        let right = comparison(parser)?;
        let span = Span::union(&expr, &right);
        expr = WithSpan::new(
            Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            },
            span,
        );
    }
    Ok(expr)
}

fn comparison(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    let mut expr = term(parser)?;
    while matches!(
        parser.peek().unwrap(),
        Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual
    ) {
        let operator = parser.advance();
        let right = term(parser)?;
        let span = Span::union(&expr, &right);
        expr = WithSpan::new(
            Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            },
            span,
        );
    }
    Ok(expr)
}

fn term(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    let mut expr = factor(parser)?;
    while matches!(parser.peek().unwrap(), Token::Plus | Token::Minus) {
        let operator = parser.advance();
        let right = factor(parser)?;
        let span = Span::union(&expr, &right);
        expr = WithSpan::new(
            Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            },
            span,
        );
    }
    Ok(expr)
}

fn factor(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    let mut expr = unary(parser)?;
    while matches!(parser.peek().unwrap(), Token::Slash | Token::Star) {
        let operator = parser.advance();
        let right = unary(parser)?;
        let span = Span::union(&expr, &right);
        expr = WithSpan::new(
            Expr::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            },
            span,
        );
    }
    Ok(expr)
}

fn unary(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    if matches!(parser.peek().unwrap(), Token::Minus | Token::Bang) {
        let operator = parser.advance();
        let right = unary(parser)?;
        let span = Span::union(&operator, &right);
        return Ok(WithSpan::new(
            Expr::Unary {
                operator,
                right: Box::new(right),
            },
            span,
        ));
    }
    primary(parser)
}

fn primary(parser: &mut Parser) -> Result<WithSpan<Expr>, ()> {
    if matches!(
        parser.peek().unwrap(),
        Token::True | Token::False | Token::Nil | Token::Number(_) | Token::String(_)
    ) {
        let token = parser.advance();
        return Ok(WithSpan::new(Expr::Literal(token.clone()), token.span));
    }
    if matches!(parser.peek().unwrap(), Token::LeftParen) {
        let left_paren = parser.advance();
        let expr = expression(parser)?;
        if !parser.matches(Token::RightParen) {
            parser.error("Unmatched parentheses.", expr.span);
            return Err(());
        }
        let right_paren = parser.advance();
        let span = Span::union(&left_paren, &right_paren);
        return Ok(WithSpan::new(Expr::Grouping(Box::new(expr)), span));
    }

    parser.error("Expected expression.", parser.current_span());
    Err(())
}
