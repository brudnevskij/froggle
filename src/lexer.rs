use crate::lexer::Token::{EOF, Identifier, Keyword, Number, Operator, Punctuation};

#[derive(Debug, PartialEq)]
pub enum Token {
    Punctuation(String),
    Keyword(String),
    Operator(String),
    Identifier(String),
    Number(i32),
    Bool(bool),
    Type(String),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.input[self.position..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.input[self.position..].chars().nth(1)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    //
    pub fn parse(&mut self) -> Vec<Token> {
        let mut token_stream = Vec::new();

        loop {
            if let Some(c) = self.peek() {
                match c {
                    '(' | ')' | ';' | ':' | '{' | '}' => {
                        token_stream.push(Punctuation(c.to_string()));
                        self.position += 1;
                    }
                    '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => {
                        let mut word = c.to_string();
                        self.position += 1;

                        while let Some(c) = self.peek() {
                            if c.is_alphanumeric() || c == '_' {
                                word.push(c);
                                self.position += 1;
                            } else {
                                break;
                            }
                        }

                        let token = match word.as_str() {
                            "let" | "croak" | "while" => Keyword(word),
                            "bool" | "number" => Token::Type(word),
                            "true" | "false" => Token::Bool(word.as_str() == "true"),
                            _ => match word.parse::<i32>() {
                                Ok(number) => Number(number),
                                Err(_) => Identifier(word),
                            },
                        };

                        token_stream.push(token);
                    }
                    ' ' | '\n' | '\t' | '\r' => {
                        self.position += 1;
                    }
                    '=' => {
                        if let Some('=') = self.peek_next() {
                            token_stream.push(Operator("==".to_string()));
                            self.position += 2;
                        } else {
                            token_stream.push(Operator("=".to_string()));
                            self.position += 1;
                        }
                    }
                    '+' | '-' | '*' | '/' | '>' | '<' => {
                        token_stream.push(Operator(c.to_string()));
                        self.position += 1;
                    }
                    _ => {
                        panic!("Unknown character: {}", c);
                    }
                }
            } else {
                token_stream.push(EOF);
                break;
            }
        }

        token_stream
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_identifier() {
        let mut lexer = Lexer::new("frog");
        let tokens = lexer.parse();

        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Identifier(ref s) if s == "frog"));
        assert!(matches!(tokens[1], EOF));
    }

    #[test]
    fn test_let_assignment() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.parse();
        println!("{:?}", tokens);

        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Keyword(ref s) if s == "let"));
        assert!(matches!(tokens[1], Identifier(ref s) if s == "x"));
        assert!(matches!(tokens[2], Operator(ref s) if s == "="));
        assert!(matches!(tokens[3], Number(n) if n == 42));
        assert!(matches!(tokens[4], Punctuation(ref s) if s == ";"));
        assert!(matches!(tokens[5], EOF));
    }

    #[test]
    fn test_arithmetic_expression() {
        let mut lexer = Lexer::new("1 + 2 * 3");
        let tokens = lexer.parse();

        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Number(1)));
        assert!(matches!(tokens[1], Operator(ref s) if s == "+"));
        assert!(matches!(tokens[2], Number(2)));
        assert!(matches!(tokens[3], Operator(ref s) if s == "*"));
        assert!(matches!(tokens[4], Number(3)));
        assert!(matches!(tokens[5], EOF));
    }
}
