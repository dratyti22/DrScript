use crate::parser::Parser;

mod interpreter;
mod ast;
mod lexer;
mod parser;
use interpreter::Interpretation;

fn main() {
    let x = r#"
        var x = 6;
        ver y = 8;
        print(x*y);
        y = 10 + x * (1+2);
        print(y+2);
    "#;
    let y = lexer::lex(x);
    println!("{:?}", y);

    let mut parser = Parser::new(y);
    let ast = parser.parse();
    println!("{:?}", ast);
    let mut i = Interpretation::new();
    i.run(ast);
}
