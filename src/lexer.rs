use crate::lexer::Token::{EOF, Identifier, Keyword, Number, Operator, Punctuation};
use std::num::ParseIntError;

pub enum Token {
    Punctuation(String),
    Keyword(String),
    Operator(String),
    Identifier(String),
    Number(i32),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.input[self.position..].chars().next()
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    //
    fn parse(&mut self) -> Vec<Token> {
        let mut token_stream = Vec::new();
        let mut word = String::new();

        while !self.is_at_end() {
            if let Some(c) = self.peek() {
                match c {
                    '(' | ')' | ';' => {
                        token_stream.push(Punctuation(c.to_string()));
                        self.position += 1;
                    }
                    '0'..'9' | 'a'..'z' | 'A'..'Z' | '_' => {
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
                            "let" | "croak" => Keyword(word),
                            _ => match word.parse::<i32>() {
                                Ok(number) => Number(number),
                                Err(_) => Identifier(word),
                            },
                        };

                        token_stream.push(token);
                        word = String::new();
                    }
                    ' ' | '\n' | '\t' | '\r' => {
                        self.position += 1;
                    }
                    '+' | '-' | '*' | '/' => {
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
