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

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();
        let result = evaluator::eval_program(&program, &mut env);
        println!("{}", result);
    }
}
