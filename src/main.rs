use std::io::Write;
use std::{env, fs, io};

mod interpreter;
mod lexer;
mod parser;
mod typechecker;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        repl();
        return;
    }
    let filename = &args[1];
    run_file(filename);
}

fn repl() {
    println!("Froggle REPL mode! 🐸 Type your code below (Ctrl+C to finish):");

    let mut interpreter = interpreter::Interpreter::new();
    loop {
        // read
        print!("froggle🐸> ");
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
        let mut typechecker = typechecker::TypeChecker::new();
        for stmt in &ast {
            stmt.accept(&mut typechecker);
        }
        interpreter.interpret(ast);

        // print
        println!("Environment:");
        println!("{:#?}", interpreter.environments);
    }
}

fn run_file(path: &str) {
    if let Ok(src_code) = fs::read_to_string(path) {
        let mut lexer = lexer::Lexer::new(&src_code);
        let mut parser = parser::Parser::new(lexer.parse());
        let ast = parser.parse();
        let mut typechecker = typechecker::TypeChecker::new();
        for node in &ast {
            node.accept(&mut typechecker);
        }
        let mut interpreter = interpreter::Interpreter::new();
        interpreter.interpret(ast);
    } else {
        panic!("Error reading file {}. Exiting.", path);
    }
}
