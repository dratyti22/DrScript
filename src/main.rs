use crate::parser::ParserToken;
use clap::Parser;

mod ast;
mod cli;
mod ffi_io;
mod functions;
mod interpreter;
mod io;
mod lexer;
mod parser;
mod type_error;
mod type_values;
mod import;

use cli::Cli;
use interpreter::Interpretation;
use std::panic;

fn main() {
    if let Err(err) = run() {
        err.report();
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<type_error::ParseError>> {
    let cli = Cli::parse();
    let code = std::fs::read_to_string(&cli.file)
        .unwrap_or_else(|_| panic!("File {} not found", cli.file));

    let lexer = lexer::lex(code.as_str());
    let mut parser = ParserToken::new(lexer);
    let ast = parser.parse().map_err(|mut e| {
        e.file = cli.file.clone();
        e.source = code.clone();
        *e
    })?;

    let mut i = Interpretation::new(None);

    match i.run(ast) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e.to_parse_error(Some(cli.file), Some(code)))),
    }
}
