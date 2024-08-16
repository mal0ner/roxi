use crate::{expr::Expr, lexer::Token};

// mod visit {
//     use crate::expr::*;
//
//     pub trait Visitor<T> {
//         fn visit_expr(&mut self, e: &Expr) -> T;
//     }
// }

pub struct Evaluator {
    ast: Box<Expr>,
}

impl Evaluator {
    pub fn new(ast: Box<Expr>) -> Self {
        Self { ast }
    }

    pub fn evaluate(&self) -> i32 {
        self.evaluate_expression(&self.ast);
        0
    }

    fn evaluate_expression(&self, e: &Expr) -> i32 {
        match &e {
            Expr::Literal(t @ (Token::Number(_) | Token::String(_))) => {
                println!("{}", t.literal_trimmed())
            }
            Expr::Literal(t) => println!("{}", t.lexeme()),
            _ => todo!(),
        }
        0
    }
}
