use crate::interpreter::{Interpretation, RuntimeResult};
use crate::lexer::lex;
use crate::parser::ParserToken;
use crate::type_error::RuntimeError;

impl Interpretation {
    pub(crate) fn import_use(&mut self, code: &str) -> RuntimeResult<()> {
        let lex = lex(code);
        let mut parser = ParserToken::new(lex);
        let ast = parser.parse().map_err(|e| RuntimeError::from(*e))?;
        for st in ast {
            self.run_stmt(st)?;
        }
        Ok(())
    }
}
