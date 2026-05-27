mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;

fn main() {
    let input = "if (10 > 1) { if (10 > 1) { return 10; } return 1; }".to_string();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();
    let mut env = environment::Environment::new();
    let result = evaluator::eval_program(&program, &mut env);
    println!("{:?}", result);
}
