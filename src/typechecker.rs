use crate::parser::Expression::BinaryOperation;
use crate::parser::{ASTVisitor, Expression, Statement, Type};
use std::collections::HashMap;

pub struct TypeChecker {
    type_envs: Vec<HashMap<String, Type>>,
    function_envs: Vec<HashMap<String, (Vec<Type>, Type)>>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            type_envs: vec![HashMap::new()],
            function_envs: vec![HashMap::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.type_envs.push(HashMap::new());
        self.function_envs.push(HashMap::new());
    }
    fn exit_scope(&mut self) {
        self.type_envs.pop();
        self.function_envs.pop();
    }

    fn declare_variable(&mut self, name: String, type_name: Type) {
        self.type_envs
            .last_mut()
            .expect(format!("error declaring variable {}", name).as_str())
            .insert(name, type_name);
    }

    fn resolve_variable(&mut self, name: &str) -> Type {
        for scope in self.type_envs.iter_mut().rev() {
            if let Some(type_name) = scope.get(name) {
                return type_name.clone();
            }
        }
        panic!("no variable {} in existing scopes", name);
    }

    fn declare_function(&mut self, name: String, parameters: Vec<Type>, return_type: Type) {
        self.function_envs
            .last_mut()
            .expect(format!("error declaring function {}", name).as_str())
            .insert(name, (parameters, return_type));
    }

    fn resolve_function(&mut self, name: &str) -> (Vec<Type>, Type) {
        for func_scope in self.function_envs.iter_mut().rev() {
            if let Some((parameters, return_type)) = func_scope.get(name) {
                return (parameters.clone(), return_type.clone());
            }
        }
        panic!("no function {} in existing scopes", name);
    }

    fn infer_datatype(&mut self, exp: &Expression) -> Type {
        match exp {
            Expression::Number(_) => Type::Number,
            Expression::Bool(_) => Type::Boolean,
            Expression::Variable(name) => self.resolve_variable(name),
            BinaryOperation {
                left,
                operator,
                right,
            } => {
                let left_type = self.infer_datatype(left);
                let right_type = self.infer_datatype(right);

                match operator.as_str() {
                    "+" | "-" | "*" | "/" => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Type::Number
                        } else {
                            panic!("operator {} requires number operand", operator);
                        }
                    }

                    ">" | "<" => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Type::Boolean
                        } else {
                            panic!("operator {} requires number operand", operator);
                        }
                    }

                    "==" => {
                        if left_type == right_type {
                            Type::Boolean
                        } else {
                            panic!("operator {} requires same type operand", operator);
                        }
                    }
                    _ => panic!("unknown operator {}", operator),
                }
            }
            Expression::FunctionCall { name, .. } => self.resolve_function(name).1,
        }
    }

    pub fn check(&mut self, stmts: Vec<Statement>) {
        for stmt in stmts {
            stmt.accept(self);
        }
    }
}

impl ASTVisitor for TypeChecker {
    fn visit_declaration(&mut self, name: String, expr: Expression, declared_type: Option<Type>) {
        let variable_type = self.infer_datatype(&expr);

        if let Some(dt) = declared_type {
            if variable_type != dt {
                panic!(
                    "Type mismatch in declaration of {}: expected {:?}, got {:?}",
                    name, dt, variable_type
                );
            }
        }

        self.declare_variable(name, variable_type);
    }

    fn visit_assignment(&mut self, name: String, expr: Expression) {
        let var_type = self.resolve_variable(&name);
        let expr_type = self.infer_datatype(&expr);
        if var_type != expr_type {
            panic!("variable {} is not equal to type of expression", name);
        }
    }

    fn visit_print(&mut self, _: Expression) {}

    fn visit_while(&mut self, condition: Expression, body: Vec<Statement>) {
        // TODO: rethink this condition
        if Type::Boolean != self.infer_datatype(&condition) {
            panic!("While condition is not boolean");
        }

        self.enter_scope();
        self.check(body);
        self.exit_scope();
    }

    fn visit_block(&mut self, statements: Vec<Statement>) {
        self.enter_scope();
        self.check(statements);
        self.exit_scope();
    }

    fn visit_function_declaration(
        &mut self,
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    ) {
        self.declare_function(
            name,
            params.iter().map(|(name, t)| t.clone()).collect(),
            return_type,
        );
        self.enter_scope();
        // adding params to scope
        for param in params {
            self.declare_variable(param.0, param.1);
        }
        self.check(body);
        self.exit_scope();
    }

    fn visit_if(
        &mut self,
        condition: Expression,
        body: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    ) {
        if self.infer_datatype(&condition) != Type::Boolean {
            panic!("If condition is not boolean");
        }
        self.enter_scope();
        self.check(body);
        self.exit_scope();
        if let Some(else_branch) = else_branch {
            self.enter_scope();
            self.check(else_branch);
            self.exit_scope();
        }
    }

    fn visit_expression(&mut self, expr: Expression) {
        self.infer_datatype(&expr);
    }

    fn visit_return(&mut self, expr: Expression) {
        // TODO: add declared return type lookup
        self.infer_datatype(&expr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Expression::{Number, Variable};
    use crate::parser::{Expression, Statement, Type};

    fn number_expr(n: i32) -> Expression {
        Expression::Number(n)
    }

    fn bool_expr(b: bool) -> Expression {
        Expression::Bool(b)
    }

    fn var(name: &str) -> Expression {
        Expression::Variable(name.to_string())
    }

    fn binop(left: Expression, op: &str, right: Expression) -> Expression {
        Expression::BinaryOperation {
            left: Box::new(left),
            operator: op.to_string(),
            right: Box::new(right),
        }
    }

    #[test]
    fn test_variable_declaration_and_assignment() {
        let mut checker = TypeChecker::new();
        let stmts = vec![
            Statement::Declaration("x".into(), number_expr(10),None),
            Statement::Assignment("x".into(), number_expr(42)),
        ];
        checker.check(stmts);
    }

    #[test]
    #[should_panic(expected = "variable x is not equal to type of expression")]
    fn test_type_mismatch_assignment() {
        let mut checker = TypeChecker::new();
        let stmts = vec![
            Statement::Declaration("x".into(), number_expr(10), None),
            Statement::Assignment("x".into(), bool_expr(true)),
        ];
        checker.check(stmts);
    }

    #[test]
    fn test_binary_operation_number_addition() {
        let mut checker = TypeChecker::new();
        let expr = binop(number_expr(1), "+", number_expr(2));
        let inferred = checker.infer_datatype(&expr);
        assert_eq!(inferred, Type::Number);
    }

    #[test]
    #[should_panic(expected = "While condition is not boolean")]
    fn test_while_condition_type_check() {
        let mut checker = TypeChecker::new();
        let stmts = vec![
            Statement::While {
                condition: number_expr(1),
                body: vec![],
            }, // wrong type
        ];
        checker.check(stmts);
    }

    #[test]
    fn test_valid_while_condition() {
        let mut checker = TypeChecker::new();
        let stmts = vec![
            Statement::Declaration("cond".into(), bool_expr(true) , None),
            Statement::While {
                condition: var("cond"),
                body: vec![
                    Statement::Declaration("x".into(), number_expr(5), None),
                    Statement::Assignment("x".into(), number_expr(10)),
                ],
            },
        ];
        checker.check(stmts); // should not panic
    }

    #[test]
    fn test_scope_within_while_block() {
        let mut checker = TypeChecker::new();
        let stmts = vec![
            Statement::Declaration("x".to_string(), Number(0), None),
            Statement::While {
                condition: bool_expr(true),
                body: vec![Statement::Assignment("x".to_string(), Number(10))],
            },
        ];
        checker.check(stmts);
    }

    #[test]
    fn test_function_declaration_and_return_type() {
        let mut checker = TypeChecker::new();
        let stmts = vec![Statement::FunctionDeclaration {
            name: "add".into(),
            params: vec![("a".into(), Type::Number), ("b".into(), Type::Number)],
            return_type: Type::Number,
            body: vec![Statement::Return(binop(var("a"), "+", var("b")))],
        }];
        checker.check(stmts);
    }
}
