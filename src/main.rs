use crate::parser::Parser;

mod ast;
mod interpreter;
mod lexer;
mod parser;
use interpreter::Interpretation;

fn main() {
    let x = r#"
        fun add(n) {
            if n <= 1 {
                return 1;
            } else {
                return n * add(n-1);
            }
        }

        print(add(10));
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
