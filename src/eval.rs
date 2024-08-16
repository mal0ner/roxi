use crate::expr::Expr;

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
            Expr::Literal(value) => println!("{}", value.lexeme()),
            _ => todo!(),
        }
        0
    }
}
