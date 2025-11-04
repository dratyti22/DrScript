use crate::parser::ParserToken;
use clap::Parser;

mod ast;
mod cli;
mod interpreter;
mod lexer;
mod parser;
use cli::Cli;
use interpreter::Interpretation;
use std::fs::read_to_string;

fn main() {
    let cli = Cli::parse();
    let code = read_to_string(&cli.file).unwrap_or_else(|_| panic!("File {} not found", cli.file));

    let lexer = lexer::lex(code.as_str());
    // println!("{:?}", lexer);
    // println!("----------");
    let mut parser = ParserToken::new(lexer);
    let ast = parser.parse();
    // println!("{:?}", ast);
    // println!("----------");

    let mut i = Interpretation::new();
    i.run(ast);
}
