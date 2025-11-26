mod expressions;
mod statements;
mod utils;

use crate::ast::Stmt;
use crate::type_error::{TError, TokenPosition};

pub struct ParserToken {
    tokens: Vec<TokenPosition>,
    pos: usize,
}

impl ParserToken {
    pub fn new(token: Vec<TokenPosition>) -> Self {
        Self {
            tokens: token,
            pos: 0,
        }
    }

    /// парсинг значений
    pub fn parse(&mut self) -> TError<Vec<Stmt>> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }
}
