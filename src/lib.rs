pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod type_error;
pub mod type_values;

use crate::interpreter::Interpretation;
use crate::parser::ParserToken;

pub fn run_source(code: &str) -> String {
    let lexer = lexer::lex(code);
    let mut parser = ParserToken::new(lexer);
    
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => return format!("Parse error: {:?}", e),
    };

    let mut inter = Interpretation::new();
    match inter.run(ast) {
        Ok(output) => output,
        Err(e) => format!("Runtime error: {:?}", e),
    }
}
