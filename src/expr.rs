use std::fmt::Display;

use crate::lexer::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Literal(Token),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(t @ (Token::Number(_) | Token::String(_))) => {
                write!(f, "{}", t.literal())
            }
            Expr::Literal(val) => write!(f, "{}", val.lexeme()),
            Expr::Unary { operator, right } => {
                write!(f, "{} {}", operator.lexeme(), right)
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme(), left, right)
            }
            Expr::Grouping(g) => write!(f, "(group {})", g),
        }
    }
}
