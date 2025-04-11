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
