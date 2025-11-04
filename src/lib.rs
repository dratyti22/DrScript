pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;

use crate::interpreter::Interpretation;
use crate::parser::ParserToken;

pub fn run_source(code: &str)->String {
    let lexer = lexer::lex(code);
    // println!("{:?}", lexer);
    // println!("----------");
    let mut parser = ParserToken::new(lexer);
    let ast = parser.parse();
    // println!("{:?}", ast);
    // println!("----------");

    let mut inter = Interpretation::new();
    let output = inter.run(ast);
    output
}
