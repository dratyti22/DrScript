use crate::parser::Parser;

mod ast;
mod interpreter;
mod lexer;
mod parser;
use interpreter::Interpretation;

fn main() {
    let x = r#"
        var x = 5;
        var y = 10;

        if x>y {
            print(1);
        }
        else {
            print(2);
        }
        if x>y {
            print(3);
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
