use std::fmt::Display;

use crate::{expr::Expr, lexer::Token};

pub struct Evaluator {
    ast: Box<Expr>,
}

pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

#[allow(dead_code)]
pub enum EvalError {
    NaN,
    InvalidUnaryOp,
    InvalidBinaryOp,
}

impl Evaluator {
    pub fn new(ast: Box<Expr>) -> Self {
        Self { ast }
    }

    pub fn evaluate(&self) -> Result<Value, EvalError> {
        self.evaluate_expression(&self.ast)
    }

    fn evaluate_expression(&self, e: &Expr) -> Result<Value, EvalError> {
        // borrow the expr so we can match against it without moving
        // or copying it.
        match &e {
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

    fn grouping(&self, e: &Expr) -> Result<Value, EvalError> {
        self.evaluate_expression(e)
    }

    fn literal(&self, t: &Token) -> Value {
        match t {
            // maybe dont do an unwrap here genius
            Token::Number(n) => Value::Number(n.parse::<f64>().unwrap()),
            Token::String(s) => Value::String(s.to_string()),
            Token::True => Value::Boolean(true),
            Token::False => Value::Boolean(false),
            _ => Value::Nil,
        }
    }

    fn unary(&self, operator: &Token, right: &Expr) -> Result<Value, EvalError> {
        let right_value = self.evaluate_expression(right)?;

        match operator {
            Token::Minus => match right_value {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(EvalError::NaN),
            },
            Token::Bang => Ok(Value::Boolean(!self.is_truthy(&right_value))),
            _ => Err(EvalError::InvalidUnaryOp),
        }
    }

    fn binary(&self, operator: &Token, left: &Expr, right: &Expr) -> Result<Value, EvalError> {
        let left_value = self.evaluate_expression(left)?;
        let right_value = self.evaluate_expression(right)?;

        match operator {
            // arithmetic operators
            Token::Plus => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                // string concatenation
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => Err(EvalError::NaN),
            },
            Token::Minus => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(EvalError::NaN),
            },
            Token::Slash => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => Err(EvalError::NaN),
            },
            Token::Star => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(EvalError::NaN),
            },
            // relational operators
            Token::Less => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                _ => Err(EvalError::NaN),
            },
            Token::LessEqual => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                _ => Err(EvalError::NaN),
            },
            Token::Greater => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                _ => Err(EvalError::NaN),
            },
            Token::GreaterEqual => match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                _ => Err(EvalError::NaN),
            },
            _ => todo!(),
        }
    }

    fn is_truthy(&self, v: &Value) -> bool {
        match v {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
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

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::NaN => write!(f, "Operand must be a number."),
            EvalError::InvalidUnaryOp => write!(f, "Unrecognized unary operator."),
            EvalError::InvalidBinaryOp => write!(f, "Unrecognized binary operator."),
        }
    }
}
