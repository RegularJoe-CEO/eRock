mod lexer;
mod parser;
mod interpreter;

fn main() {
    let input = "2 + 2";
    let tokens = lexer::tokenize(input);
    let ast = parser::parse(tokens);
    interpreter::interpret(ast);
}
