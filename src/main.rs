use crate::parser::ParserToken;
use clap::Parser;

mod ast;
mod cli;
mod error_handler;
mod interpreter;
mod lexer;
mod parser;
mod type_error;

use cli::Cli;
use interpreter::Interpretation;
use std::panic;
use crate::type_error::{ParseError, ParseErrorKind};

fn main() {
    if let Err(err) = run() {
        err.report(); // <<< вот тут используется report()
        std::process::exit(1);
    }
}

fn run() -> Result<(), type_error::ParseError> {
    let cli = Cli::parse();
    let code = std::fs::read_to_string(&cli.file)
        .unwrap_or_else(|_| panic!("File {} not found", cli.file));

    let lexer = lexer::lex(code.as_str());
    let mut parser = ParserToken::new(lexer);
    let ast = parser.parse().map_err(|mut e| {
        e.file = cli.file.clone();
        e.source = code.clone();
        e
    })?;

    let mut i = Interpretation::new();

    // Обработка runtime ошибок
    match i.run(ast) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(ParseError::new(
                ParseErrorKind::Runtime(e),
                Default::default(), // временно используем пустой span
                Some(cli.file.clone()),
                Some(code.clone()),
            ))
        }
    }
}
