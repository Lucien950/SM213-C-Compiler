mod rucc;

fn main() {
    use std::fs;
    use rucc::{lexer, parser};
    let name = fs::read_to_string("./test.c")?;
    let mut lexer = lexer::Lexer::new(name);
    let mut parser = parser::Parser::new(&mut lexer);
    let ast = parser.parse();
}