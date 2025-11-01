use crate::parser::Parser;

mod ast;
mod interpreter;
mod lexer;
mod parser;
use interpreter::Interpretation;

fn main() {
    let x = r#"
        ver x = 5;
        var y = 10;

        for (i=0;i<10;i++) {
            print(x);
        }
    "#;
    let y = lexer::lex(x);
    println!("{:?}", y);
    println!("----------");

    let mut parser = Parser::new(y);
    let ast = parser.parse();
    println!("{:?}", ast);
    println!("----------");

    let mut i = Interpretation::new();
    i.run(ast);
}
