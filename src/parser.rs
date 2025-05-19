use crate::lexer::Token;
use crate::parser::Expression::BinaryOperation;
use crate::parser::Statement::{If, While};
use std::collections::HashMap;

// Vec<Statement>
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration(String, Expression, Option<Type>),
    Assignment(String, Expression),
    Print(Expression),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>),
    FunctionDeclaration {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    Expression(Expression),
    Return(Expression),
}

impl Statement {
    pub fn accept<V: ASTVisitor>(&self, visitor: &mut V) {
        match self {
            Statement::Declaration(name, exp, declared_type ) => {
                visitor.visit_declaration(name.clone(), exp.clone(), declared_type.clone())
            }
            Statement::Assignment(name, exp) => visitor.visit_assignment(name.clone(), exp.clone()),

            Statement::Print(exp) => visitor.visit_print(exp.clone()),

            While { condition, body } => visitor.visit_while(condition.clone(), body.clone()),

            Statement::Block(stmt) => visitor.visit_block(stmt.clone()),
            Statement::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
            } => visitor.visit_function_declaration(
                name.clone(),
                params.clone(),
                return_type.clone(),
                body.clone(),
            ),

            If {
                condition,
                then_block,
                else_block,
            } => visitor.visit_if(condition.clone(), then_block.clone(), else_block.clone()),

            Statement::Expression(exp) => visitor.visit_expression(exp.clone()),

            Statement::Return(ret) => visitor.visit_return(ret.clone()),
        }
    }
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
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    Boolean,
    Void,
}

pub trait ASTVisitor {
    fn visit_declaration(&mut self, name: String, expr: Expression, declared_type: Option<Type>);
    fn visit_assignment(&mut self, name: String, expr: Expression);
    fn visit_print(&mut self, expr: Expression);
    fn visit_while(&mut self, condition: Expression, body: Vec<Statement>);
    fn visit_block(&mut self, statements: Vec<Statement>);
    fn visit_function_declaration(
        &mut self,
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Statement>,
    );
    fn visit_if(
        &mut self,
        condition: Expression,
        body: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    );
    fn visit_expression(&mut self, expr: Expression);
    fn visit_return(&mut self, expr: Expression);
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
                    // implicit type declaration
                    Some(Token::Operator(op)) if op == "=" => {
                        let expr = self.parse_expression();
                        self.expect(Token::Punctuation(";".to_string()));
                        Some(Statement::Declaration(name, expr, None))
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

                        Some(Statement::Declaration(name, expr, Some(declared_data_type)))
                    }
                    _ => panic!("Unknown declaration structure"),
                }
            }

            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();

                if Some(&Token::Punctuation("(".to_string())) == self.peek() {
                    self.advance();

                    let arguments = self.parse_function_args();
                    self.expect(Token::Punctuation(")".to_string()));
                    self.expect(Token::Punctuation(";".to_string()));
                    Some(Statement::Expression(Expression::FunctionCall {
                        name,
                        arguments,
                    }))
                } else {
                    self.expect(Token::Operator("=".to_string()));
                    let expr = self.parse_expression();
                    self.expect(Token::Punctuation(";".to_string()));
                    Some(Statement::Assignment(name, expr))
                }
            }

            Some(Token::Keyword(k)) if k == "croak" => {
                self.advance(); // consume "print"
                let expr = self.parse_expression();
                self.expect(Token::Punctuation(";".to_string()));
                Some(Statement::Print(expr))
            }

            Some(Token::Keyword(k)) if k == "return" => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(Token::Punctuation(";".to_string()));
                Some(Statement::Return(expr))
            }

            Some(Token::Keyword(k)) if k == "while" => {
                self.advance();

                let condition = self.parse_expression();
                self.expect(Token::Punctuation("{".to_string()));

                let body = self.parse_block();
                self.expect(Token::Punctuation("}".to_string()));

                Some(While { condition, body })
            }

            Some(Token::Punctuation(p)) if p == "{" => {
                self.advance();

                let block = self.parse_block();

                self.expect(Token::Punctuation("}".to_string()));

                Some(Statement::Block(block))
            }

            Some(Token::Keyword(k)) if k == "if" => {
                self.advance();

                let condition = self.parse_expression();
                self.expect(Token::Punctuation("{".to_string()));

                let then_block = self.parse_block();
                self.expect(Token::Punctuation("}".to_string()));

                if self.peek() != Some(&Token::Keyword("else".to_string())) {
                    return Some(If {
                        condition,
                        then_block,
                        else_block: None,
                    });
                }
                self.advance();
                self.expect(Token::Punctuation("{".to_string()));

                let else_block = self.parse_block();
                self.expect(Token::Punctuation("}".to_string()));

                Some(If {
                    condition,
                    then_block,
                    else_block: Some(else_block),
                })
            }

            Some(Token::Keyword(k)) if k == "func" => {
                self.advance();

                let name = match self.advance() {
                    Some(Token::Identifier(s)) => s.clone(),
                    a => panic!("Expected identifier after 'func', got: {:?}", a),
                };

                self.expect(Token::Punctuation("(".to_string()));

                let mut params = Vec::new();

                while let Some(Token::Identifier(param_name)) = self.peek() {
                    let param_name = param_name.clone();
                    self.advance();

                    self.expect(Token::Punctuation(":".to_string()));

                    let param_type = match self.advance() {
                        Some(Token::Type(t)) if t == "bool" => Type::Boolean,
                        Some(Token::Type(t)) if t == "number" => Type::Number,
                        a => panic!("Expected type, got: {:?}", a),
                    };
                    params.push((param_name, param_type));

                    if self.peek() == Some(&Token::Punctuation(",".to_string())) {
                        self.advance();
                        continue;
                    } else {
                        break;
                    }
                }

                self.expect(Token::Punctuation(")".to_string()));

                let return_type = match self.peek() {
                    Some(Token::Punctuation(p)) if p == ":" => {
                        self.advance();
                        match self.advance() {
                            Some(Token::Type(t)) if t == "number" => Type::Number,
                            Some(Token::Type(t)) if t == "bool" => Type::Boolean,
                            a => panic!("Expected type, got: {:?}", a),
                        }
                    }
                    Some(Token::Punctuation(p)) if p == "{" => Type::Void,
                    a => panic!("Expected type, got: {:?}", a),
                };

                self.expect(Token::Punctuation("{".to_string()));

                let body = self.parse_block();

                self.expect(Token::Punctuation("}".to_string()));

                Some(Statement::FunctionDeclaration {
                    name,
                    params,
                    return_type,
                    body,
                })
            }

            Some(Token::EOF) => None,
            statement => panic!("unknown statement: {:?}", statement),
        }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        let mut block = Vec::new();

        while let Some(t) = self.peek() {
            if t == &Token::Punctuation("}".to_string()) {
                break;
            }

            if let Some(stmt) = self.parse_statement() {
                block.push(stmt);
            }
        }

        block
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
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                if self.peek() == Some(&Token::Punctuation("(".to_string())) {
                    self.advance();

                    let arguments = self.parse_function_args();

                    self.expect(Token::Punctuation(")".to_string()));

                    Expression::FunctionCall { name, arguments }
                } else {
                    Expression::Variable(name)
                }
            }
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

    // parses function call arguments
    fn parse_function_args(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();

        if Some(&Token::Punctuation(")".to_string())) == self.peek() {
            return args;
        }

        loop {
            let arg = self.parse_expression();
            args.push(arg);

            match self.peek() {
                Some(Token::Punctuation(t)) if t == ")" => break,
                Some(Token::Punctuation(t)) if t == "," => {
                    self.advance();
                    continue;
                }
                a => panic!("Unexpected token {:?}", a),
            }
        }
        args
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
