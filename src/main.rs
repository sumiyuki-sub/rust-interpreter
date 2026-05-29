use std::io::{self, Write};

mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;

fn main() {
    let mut env = environment::Environment::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" || input == "exit" {
            break;
        }

        let lexer = lexer::Lexer::new(input.to_string());
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors().is_empty() {
            for error in parser.errors() {
                println!("parser error: {}", error)
            }
            continue;
        }

        let result = evaluator::eval_program(&program, &mut env);

        match result {
            object::Object::Null => {}
            other => println!("{}", other),
        }
    }
}
