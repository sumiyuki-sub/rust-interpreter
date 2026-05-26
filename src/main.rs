mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let input = "add(1, 2)".to_string();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();
    println!("{:#?}", program);
}
