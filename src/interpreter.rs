use crate::interpreter::Value::Bool;
use crate::parser::{Expression, Statement, Type};
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Bool(bool),
}

pub struct Interpreter {
    pub environment: HashMap<String, Value>,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Number(n) => {
                if let Value::Number(o) = other {
                    return n == o;
                }
            }
            Bool(b) => {
                if let Bool(o) = other {
                    return b == o;
                }
            }
        }
        false
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Vec<Statement>) {
        for stmt in program {
            self.eval_statement(stmt);
        }
    }

    fn eval_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Assignment(var, exp) => {
                let value = self.eval_expression(exp);
                self.environment.insert(var, value);
            }
            Statement::Print(exp) => {
                println!("{:?}", self.eval_expression(exp))
            }
        }
    }
    fn eval_expression(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::Bool(b) => Value::Bool(b),
            Expression::Variable(var) => match self.environment.get(&var) {
                None => panic!("Undefined variable: {}", var),
                Some(val) => val.clone(),
            },
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left = self.eval_expression(*left);
                let right = self.eval_expression(*right);

                match (left, operator.as_str(), right) {
                    (Value::Number(left), "+", Value::Number(right)) => Value::Number(left + right),
                    (Value::Number(left), "-", Value::Number(right)) => Value::Number(left - right),
                    (Value::Number(left), "*", Value::Number(right)) => Value::Number(left * right),
                    (Value::Number(left), "/", Value::Number(right)) => Value::Number(left / right),

                    (Value::Number(left), ">", Value::Number(right)) => Value::Bool(left > right),
                    (Value::Number(left), "<", Value::Number(right)) => Value::Bool(left < right),

                    (l, "==", r) => Bool(l == r),
                    _ => panic!("unsupported operation: {}", operator.as_str()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Expression, Statement};

    fn number(n: i32) -> Expression {
        Expression::Number(n)
    }

    fn var(name: &str) -> Expression {
        Expression::Variable(name.to_string())
    }

    fn bin(left: Expression, op: &str, right: Expression) -> Expression {
        Expression::BinaryOperation {
            left: Box::new(left),
            operator: op.to_string(),
            right: Box::new(right),
        }
    }

    #[test]
    fn test_variable_assignment() {
        let program = vec![Statement::Assignment("x".to_string(), number(10))];
        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("x"), Some(&Value::Number(10)));
    }

    #[test]
    fn test_expression_evaluation() {
        let program = vec![
            Statement::Assignment("x".to_string(), number(5)),
            Statement::Assignment("y".to_string(), bin(var("x"), "+", number(3))),
        ];

        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("y"), Some(&Value::Number(8)));
    }

    #[test]
    fn test_operator_precedence() {
        // x = 1 + 2 * 3
        let expr = bin(number(1), "+", bin(number(2), "*", number(3)));

        let program = vec![Statement::Assignment("x".to_string(), expr)];
        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("x"), Some(&Value::Number(7)));
    }

    #[test]
    fn test_parentheses_grouping() {
        // x = (1 + 2) * 3
        let expr = bin(bin(number(1), "+", number(2)), "*", number(3));

        let program = vec![Statement::Assignment("x".to_string(), expr)];
        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("x"), Some(&Value::Number(9)));
    }
}
