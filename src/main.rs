use std::io::{Read, Write};
use std::{env, io};

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Froggle REPL mode! 🐸 Type your code below (Ctrl+D to finish):");

        let mut interpreter = interpreter::Interpreter::new();
        loop {
            // read
            print!("froggle> ");
            io::stdout().lock().flush().unwrap();

            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                println!("Error reading line. Exiting.");
                break;
            }

            let line = line.trim();

            if line == "exit" {
                break;
            }

            if line.is_empty() {
                continue;
            }

            // evaluate
            let mut lexer = lexer::Lexer::new(&line);
            let mut parser = parser::Parser::new(lexer.parse());
            let ast = parser.parse();
            interpreter.interpret(ast);

            // print
            println!("Environment:");
            println!("{:#?}", interpreter.environment);
        }
    }
}
