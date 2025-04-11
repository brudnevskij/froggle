use crate::parser::{Expression, Statement};
use std::collections::HashMap;

pub struct Interpreter {
    pub environment: HashMap<String, i32>,
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
                println!("{}", self.eval_expression(exp))
            }
        }
    }
    fn eval_expression(&mut self, expression: Expression) -> i32 {
        match expression {
            Expression::Number(n) => n,
            Expression::Variable(var) => match self.environment.get(&var) {
                None => panic!("Undefined variable: {}", var),
                Some(val) => *val,
            },
            Expression::BinaryOperation {
                left,
                operator,
                right,
            } => {
                let right_val = self.eval_expression(*right);
                let left_val = self.eval_expression(*left);

                match operator.as_str() {
                    "+" => right_val + left_val,
                    "-" => right_val - left_val,
                    "*" => right_val * left_val,
                    "/" => right_val / left_val,
                    v => panic!("Undefined operator: {}", v),
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

        assert_eq!(interpreter.environment.get("x"), Some(&10));
    }

    #[test]
    fn test_expression_evaluation() {
        let program = vec![
            Statement::Assignment("x".to_string(), number(5)),
            Statement::Assignment("y".to_string(), bin(var("x"), "+", number(3))),
        ];

        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("y"), Some(&8));
    }

    #[test]
    fn test_operator_precedence() {
        // x = 1 + 2 * 3
        let expr = bin(
            number(1),
            "+",
            bin(number(2), "*", number(3)),
        );

        let program = vec![Statement::Assignment("x".to_string(), expr)];
        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("x"), Some(&7));
    }

    #[test]
    fn test_parentheses_grouping() {
        // x = (1 + 2) * 3
        let expr = bin(
            bin(number(1), "+", number(2)),
            "*",
            number(3),
        );

        let program = vec![Statement::Assignment("x".to_string(), expr)];
        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(interpreter.environment.get("x"), Some(&9));
    }
}
