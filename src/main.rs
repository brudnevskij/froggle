mod lexer;
mod parser;

fn main() {
    let input = "let x = 1 + 2 * (3 + 4); croak x;";
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.parse();

    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse();
    println!("{:#?}", statements);
}
