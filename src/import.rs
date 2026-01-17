use crate::ast::Stmt;
use crate::interpreter::{Interpretation, RuntimeResult, Value};
use crate::io::DrScriptIo;
use crate::lexer::lex;
use crate::parser::ParserToken;
use crate::type_error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Interpretation {
    pub(crate) fn import_use(
        &self,
        code: &str,
        io: Rc<RefCell<dyn DrScriptIo>>,
    ) -> RuntimeResult<(
        HashMap<String, Value>,
        HashMap<String, (Vec<String>, Vec<Stmt>)>,
    )> {
        let lex = lex(code);
        let mut parser = ParserToken::new(lex);
        let ast = parser.parse().map_err(|e| RuntimeError::from(*e))?;

        let mut local = Interpretation::new(Some(io));
        local.run(ast).map_err(|e| RuntimeError::from(e))?;
        let (vars, funcs) = local.get_vars_funcs();
        Ok((vars.clone(), funcs.clone()))
    }
}
