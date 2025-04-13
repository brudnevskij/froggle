use crate::lexer::Token;
use crate::parser::Expression::BinaryOperation;
use crate::parser::Statement::While;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assignment(String, Expression),
    Print(Expression),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
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

#[derive(Debug, PartialEq)]
pub enum Type {
    Number,
    Boolean,
}

pub struct Parser {
    tokens: Vec<Token>,
    type_env: HashMap<String, Type>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            type_env: HashMap::new(),
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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            statements.push(stmt);
        }
        statements
    }
    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Some(Token::Keyword(k)) if k == "let" => {
                self.advance();
                let name = match self.advance() {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => panic!("Expected identifier after 'let'"),
                };

                // skipping ":"
                self.advance();
                let data_type = match self.advance() {
                    Some(Token::Type(s)) if s.as_str() == "bool" => Type::Boolean,
                    Some(Token::Type(s)) if s.as_str() == "number" => Type::Number,
                    _ => panic!("Expected type after let"),
                };

                // check data type
                match self.type_env.get(&name) {
                    None => {
                        self.type_env.insert(name.clone(), data_type);
                    }
                    Some(dt) => {
                        if dt != &data_type {
                            // todo: add more logs
                            panic!(
                                "Type mismatch! {} type is {}, while expression is {}",
                                name, dt, &data_type
                            );
                        }
                    }
                }

                match self.advance() {
                    Some(Token::Operator(op)) if op == "=" => {}
                    _ => panic!("Expected '='"),
                };
                let expr = self.parse_expression();
                self.expect(Token::Punctuation(";".to_string()));
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

                let mut body = Vec::new();
                while let Some(t) = self.peek() {
                    if t == &Token::Punctuation("}".to_string()) {
                        break;
                    }

                    if let Some(stmt) = self.parse_statement() {
                        body.push(stmt);
                    }
                }

                self.expect(Token::Punctuation("}".to_string()));
                Some(While { condition, body })
            }

            Some(Token::EOF) => None,
            _ => panic!("Unknown statement"),
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
