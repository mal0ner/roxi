use crate::{
    expr::Expr,
    lexer::Token,
    position::{Diagnostic, Span, WithSpan},
};
use std::fmt::Display;

pub struct Evaluator {
    ast: Box<WithSpan<Expr>>,
    diagnostics: Vec<Diagnostic>,
}

pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Evaluator {
    pub fn new(ast: Box<WithSpan<Expr>>) -> Self {
        Self {
            ast,
            diagnostics: Vec::new(),
        }
    }

    pub fn evaluate(&self) -> Result<Value, Diagnostic> {
        self.evaluate_expression(&self.ast)
    }

    pub fn error(&self, message: &str, span: Span) -> Diagnostic {
        Diagnostic {
            message: message.to_string(),
            span,
        }
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    fn evaluate_expression(&self, e: &WithSpan<Expr>) -> Result<Value, Diagnostic> {
        // borrow the expr so we can match against it without moving
        // or copying it.
        match &e.value {
            Expr::Literal(t) => Ok(self.literal(t)),
            Expr::Grouping(expr) => self.grouping(expr),
            Expr::Unary { operator, right } => self.unary(operator, right),
            Expr::Binary {
                operator,
                left,
                right,
            } => self.binary(operator, left, right),
        }
    }

    fn grouping(&self, e: &WithSpan<Expr>) -> Result<Value, Diagnostic> {
        self.evaluate_expression(e)
    }

    fn literal(&self, t: &WithSpan<Token>) -> Value {
        match &t.value {
            // maybe dont do an unwrap here genius
            Token::Number(n) => Value::Number(n.parse::<f64>().unwrap()),
            Token::String(s) => Value::String(s.to_string()),
            Token::True => Value::Boolean(true),
            Token::False => Value::Boolean(false),
            _ => Value::Nil,
        }
    }

    fn unary(
        &self,
        operator: &WithSpan<Token>,
        right: &WithSpan<Expr>,
    ) -> Result<Value, Diagnostic> {
        let right_value = self.evaluate_expression(right)?;

        match &operator.value {
            Token::Minus => match right_value {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(self.error(
                    &format!("Operand {} must be a number.", operator.value.clone()),
                    operator.span,
                )),
            },
            Token::Bang => Ok(Value::Boolean(!self.is_truthy(&right_value))),
            _ => Err(self.error(
                &format!(
                    "Unrecognized unary operator \"{}\".",
                    operator.value.clone()
                ),
                operator.span,
            )),
        }
    }

    fn binary(
        &self,
        operator: &WithSpan<Token>,
        left: &WithSpan<Expr>,
        right: &WithSpan<Expr>,
    ) -> Result<Value, Diagnostic> {
        let left_value = self.evaluate_expression(left)?;
        let right_value = self.evaluate_expression(right)?;

        match &operator.value {
            // arithmetic operators
            Token::Plus => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                // string concatenation
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => Err(self.error(
                    "Operands must be two numbers or two strings.",
                    Span::union(left, right),
                )),
            },
            Token::Minus => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            Token::Slash => match (left_value, right_value) {
                // check for divide by zero
                (Value::Number(l), Value::Number(r)) => match r {
                    0.0 => Err(self.error("Divide by zero.", Span::union(left, right))),
                    _ => Ok(Value::Number(l / r)),
                },
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            Token::Star => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            // relational operators
            Token::Less => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            Token::LessEqual => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            Token::Greater => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            Token::GreaterEqual => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                _ => Err(self.error("Operands must be numbers.", Span::union(left, right))),
            },
            // equality
            Token::EqualEqual => Ok(Value::Boolean(self.is_equal(&left_value, &right_value))),
            Token::BangEqual => Ok(Value::Boolean(!self.is_equal(&left_value, &right_value))),
            _ => Err(self.error(
                &format!("Invalid operator \"{}\"", operator.value.clone()),
                Span::union(left, right),
            )),
        }
    }

    fn is_truthy(&self, v: &Value) -> bool {
        match v {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}
