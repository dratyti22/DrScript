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

        print(x < y); // 1
        print(x > y); // 0
        print(x == 5); // 1
        print(y != 5); // 1
        print(y >= 10); // 1
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
