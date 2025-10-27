mod expressions;
mod statements;
mod utils;

use crate::ast::Stmt;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Self {
            tokens: token,
            pos: 0,
        }
    }

    /// парсинг значений
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt())
        }
        stmts
    }
}
