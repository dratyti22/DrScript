use crate::parser::Parser;

mod lexer;
mod ast;
mod parser;

fn main() {
    let x = r#"
        var x = 6;
        ver y = 8;
        print(x*y);
        y = 10 + x * (1+2);
        print(y);
    "#;
    let y = lexer::lex(x);
    println!("{:?}", y);

    let mut parser = Parser::new(y);
    let ast = parser.parse();
    println!("{:?}", ast);
}
