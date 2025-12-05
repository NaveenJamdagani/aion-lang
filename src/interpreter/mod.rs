use std::collections::HashMap;

use crate::ast::{Expression, Statement};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}

#[derive(Debug)]
enum EvalResult {
    Value(Object),
    Return(Object),
}

use EvalResult::{Return, Value};

pub fn eval_program(statements: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for stmt in statements {
        match eval_statement(stmt, env) {
            Return(obj) => return obj,
            Value(obj) => result = obj,
        }
    }

    result
}

fn eval_statement(stmt: Statement, env: &mut Environment) -> EvalResult {
    match stmt {
        Statement::LetStatement { name, value } => {
            let val = eval_expression(value, env);
            env.set(name, val.clone());
            Value(Object::Null)
        }
        Statement::ReturnStatement { value } => {
            let val = eval_expression(value, env);
            Return(val)
        }
        Statement::ExpressionStatement(expr) => {
            let val = eval_expression(expr, env);
            Value(val)
        }
    }
}

fn eval_expression(expr: Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::NumberLiteral(n) => Object::Integer(n),
        Expression::Identifier(name) => {
            if let Some(v) = env.get(&name) {
                v
            } else {
                Object::Null
            }
        }
        Expression::Prefix { operator, right } => {
            let right_val = eval_expression(*right, env);
            eval_prefix_expression(&operator, right_val)
        }
        Expression::Infix { left, operator, right } => {
            let left_val = eval_expression(*left, env);
            let right_val = eval_expression(*right, env);
            eval_infix_expression(&operator, left_val, right_val)
        }
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => match right {
            Object::Boolean(b) => Object::Boolean(!b),
            Object::Null => Object::Boolean(true),
            Object::Integer(i) => Object::Boolean(i == 0),
        },
        "-" => match right {
            Object::Integer(i) => Object::Integer(-i),
            _ => Object::Null,
        },
        _ => Object::Null,
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    use Object::*;
    match (left, right) {
        (Integer(l), Integer(r)) => match operator {
            "+" => Integer(l + r),
            "-" => Integer(l - r),
            "*" => Integer(l * r),
            "/" => Integer(l / r),
            "<" => Boolean(l < r),
            ">" => Boolean(l > r),
            "==" => Boolean(l == r),
            "!=" => Boolean(l != r),
            _ => Null,
        },

        (Boolean(l), Boolean(r)) => match operator {
            "==" => Boolean(l == r),
            "!=" => Boolean(l != r),
            _ => Null,
        },

        (l, r) => {
            let eq = l == r;
            match operator {
                "==" => Boolean(eq),
                "!=" => Boolean(!eq),
                _ => Null,
            }
        }
    }
}
