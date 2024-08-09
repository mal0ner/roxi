use std::fmt::Display;

use crate::lexer::Token;

#[allow(dead_code, clippy::vec_box)]
#[derive(Debug)]
pub enum Expr<'a> {
    Bool(bool),
    Nil,
    Number(f64),
    String(&'a str),
    Unary {
        operator: Token,
        right: Box<Expr<'a>>,
    },
    Binary {
        operator: Token,
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
    Grouping(Vec<Box<Expr<'a>>>),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Nil => write!(f, "nil"),
            Expr::Number(n) => write!(f, "{:?}", n),
            Expr::String(s) => write!(f, "{:?}", s),
            Expr::Unary { operator, right } => {
                write!(f, "{} {}", operator.lexeme, right)
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(_) => todo!(),
        }
    }
}
