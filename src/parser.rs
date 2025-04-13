use crate::lexer::Token;
use crate::parser::Expression::BinaryOperation;
use crate::parser::Statement::While;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration(String, Expression),
    Assignment(String, Expression),
    Print(Expression),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(i32),
    Bool(bool),
    Variable(String),
    BinaryOperation {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    Boolean,
}

pub struct Parser {
    tokens: Vec<Token>,
    type_envs: Vec<HashMap<String, Type>>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut type_envs = Vec::new();
        type_envs.push(HashMap::new());
        Self {
            tokens,
            current: 0,
            type_envs,
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            return Some(&self.tokens[self.current]);
        }
        None
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current)?;
        self.current += 1;
        Some(token)
    }

    fn enter_scope(&mut self) {
        self.type_envs.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.type_envs.pop();
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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            statements.push(stmt);
        }
        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Some(Token::Keyword(k)) if k == "let" => {
                self.advance();
                let name = match self.advance() {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => panic!("Expected identifier after 'let'"),
                };

                match self.advance() {
                    // type declaration omitted
                    Some(Token::Operator(op)) if op == "=" => {
                        let expr = self.parse_expression();
                        self.expect(Token::Punctuation(";".to_string()));

                        let data_type = self.infer_datatype(&expr);

                        self.declare_variable(name.clone(), data_type);

                        Some(Statement::Declaration(name, expr))
                    }
                    // explicit type declaration
                    Some(Token::Punctuation(op)) if op == ":" => {
                        let declared_data_type = match self.advance() {
                            Some(Token::Type(s)) if s.as_str() == "bool" => Type::Boolean,
                            Some(Token::Type(s)) if s.as_str() == "number" => Type::Number,
                            _ => panic!("Expected type after :"),
                        };

                        self.expect(Token::Operator("=".to_string()));

                        let expr = self.parse_expression();
                        self.expect(Token::Punctuation(";".to_string()));

                        let expr_data_type = self.infer_datatype(&expr);
                        if expr_data_type != declared_data_type {
                            panic!(
                                "Declared datatype: {:?}, inferred datatype: {:?}",
                                declared_data_type, expr_data_type
                            );
                        }

                        self.declare_variable(name.clone(), declared_data_type);

                        Some(Statement::Declaration(name, expr))
                    }
                    _ => panic!("Unknown declaration structure"),
                }
            }

            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                let variable_type = self.resolve_variable(&name);

                self.expect(Token::Operator("=".to_string()));

                let expr = self.parse_expression();
                self.expect(Token::Punctuation(";".to_string()));

                // asserting data type
                let expr_data_type = self.infer_datatype(&expr);
                if expr_data_type != variable_type {
                    panic!(
                        "Variable datatype: {:?}, inferred datatype: {:?}",
                        variable_type, expr_data_type
                    );
                }

                Some(Statement::Assignment(name, expr))
            }
            Some(Token::Keyword(k)) if k == "croak" => {
                self.advance(); // consume "print"
                let expr = self.parse_expression();
                self.expect(Token::Punctuation(";".to_string()));
                Some(Statement::Print(expr))
            }

            Some(Token::Keyword(k)) if k == "while" => {
                self.advance();
                let condition = self.parse_expression();
                self.expect(Token::Punctuation("{".to_string()));

                self.enter_scope();
                let mut body = Vec::new();
                while let Some(t) = self.peek() {
                    if t == &Token::Punctuation("}".to_string()) {
                        break;
                    }

                    if let Some(stmt) = self.parse_statement() {
                        body.push(stmt);
                    }
                }
                self.exit_scope();

                self.expect(Token::Punctuation("}".to_string()));
                Some(While { condition, body })
            }

            Some(Token::Punctuation(p)) if p == "{" => {
                self.advance();

                let mut block = Vec::new();
                self.enter_scope();
                while let Some(t) = self.peek() {
                    if t == &Token::Punctuation("}".to_string()) {
                        break;
                    }

                    if let Some(stmt) = self.parse_statement() {
                        block.push(stmt);
                    }
                }
                self.exit_scope();
                self.expect(Token::Punctuation("}".to_string()));

                Some(Statement::Block(block))
            }

            Some(Token::EOF) => None,
            statement => panic!("unknown statement: {:?}", statement),
        }
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
                    "+" | "-" | "*" | "/" | ">" | "<" => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Type::Number
                        } else {
                            panic!("operator {} requires number operand", operator);
                        }
                    }

                    "==" => {
                        if left_type == right_type {
                            left_type
                        } else {
                            panic!("operator {} requires same type operand", operator);
                        }
                    }
                    _ => panic!("unknown operator {}", operator),
                }
            }
        }
    }

    fn parse_expression(&mut self) -> Expression {
        let mut expression = self.parse_addition();

        while let Some(Token::Operator(op)) = self.peek() {
            if op == "==" || op == ">" || op == "<" {
                let op = op.clone();
                self.advance();

                let right = self.parse_addition();
                expression = BinaryOperation {
                    left: Box::new(expression),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        expression
    }

    fn parse_addition(&mut self) -> Expression {
        let mut expression = self.parse_term();

        while let Some(Token::Operator(op)) = self.peek() {
            if op == "+" || op == "-" {
                let op = op.clone();
                self.advance();

                let right = self.parse_term();
                expression = Expression::BinaryOperation {
                    left: Box::new(expression),
                    operator: op,
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }
        expression
    }

    fn parse_term(&mut self) -> Expression {
        let mut expr = self.parse_factor();

        while let Some(Token::Operator(op)) = self.peek() {
            if op == "*" || op == "/" {
                let op = op.clone();
                self.advance();
                let right = self.parse_factor();
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        expr
    }

    fn expect(&mut self, token: Token) {
        if self.peek() == Some(&token) {
            self.advance();
            return;
        }
        panic!("Expected token {:?}, but got {:?}", token, self.peek());
    }

    fn parse_factor(&mut self) -> Expression {
        match self.advance() {
            Some(Token::Number(n)) => Expression::Number(*n),
            Some(Token::Bool(b)) => Expression::Bool(*b),
            Some(Token::Identifier(name)) => Expression::Variable(name.clone()),
            Some(Token::Punctuation(p)) if p == "(" => {
                let expr = self.parse_expression();
                self.expect(Token::Punctuation(")".to_string()));
                expr
            }
            Some(t) => {
                panic!("Unexpected token {:?}", t)
            }
            None => {
                panic!("Unexpected EOF")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    fn token_number(n: i32) -> Token {
        Token::Number(n)
    }

    fn token_ident(name: &str) -> Token {
        Token::Identifier(name.to_string())
    }

    fn token_keyword(word: &str) -> Token {
        Token::Keyword(word.to_string())
    }

    fn token_operator(op: &str) -> Token {
        Token::Operator(op.to_string())
    }

    fn token_punct(p: &str) -> Token {
        Token::Punctuation(p.to_string())
    }

    fn token_type(p: &str) -> Token {
        Token::Type(p.to_string())
    }

    fn eof() -> Token {
        Token::EOF
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = vec![
            token_keyword("let"),
            token_ident("x"),
            token_punct(":"),
            token_type("number"),
            token_operator("="),
            token_number(42),
            token_punct(";"),
            eof(),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let expected = vec![Statement::Assignment(
            "x".to_string(),
            Expression::Number(42),
        )];

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_print_statement() {
        let tokens = vec![
            token_keyword("croak"),
            token_ident("x"),
            token_punct(";"),
            eof(),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let expected = vec![Statement::Print(Expression::Variable("x".to_string()))];

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_expression_with_precedence() {
        // let x = 1 + 2 * 3;
        let tokens = vec![
            token_keyword("let"),
            token_ident("x"),
            token_punct(":"),
            token_type("number"),
            token_operator("="),
            token_number(1),
            token_operator("+"),
            token_number(2),
            token_operator("*"),
            token_number(3),
            token_punct(";"),
            eof(),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let expected_expr = Expression::BinaryOperation {
            left: Box::new(Expression::Number(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Number(2)),
                operator: "*".to_string(),
                right: Box::new(Expression::Number(3)),
            }),
        };

        let expected = vec![Statement::Assignment("x".to_string(), expected_expr)];

        assert_eq!(ast, expected);
    }

    #[test]
    fn test_parse_grouped_expression() {
        // let x = (1 + 2) * 3;
        let tokens = vec![
            token_keyword("let"),
            token_ident("x"),
            token_punct(":"),
            token_type("number"),
            token_operator("="),
            token_punct("("),
            token_number(1),
            token_operator("+"),
            token_number(2),
            token_punct(")"),
            token_operator("*"),
            token_number(3),
            token_punct(";"),
            eof(),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let expected_expr = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Number(1)),
                operator: "+".to_string(),
                right: Box::new(Expression::Number(2)),
            }),
            operator: "*".to_string(),
            right: Box::new(Expression::Number(3)),
        };

        let expected = vec![Statement::Assignment("x".to_string(), expected_expr)];

        assert_eq!(ast, expected);
    }
}
