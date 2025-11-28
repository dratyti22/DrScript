use crate::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::type_error::RuntimeError;
use std::collections::HashMap;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone)]
struct Value {
    value: i64,
    mutable: bool,
}

#[derive(Debug)]
enum ExecReturn {
    Return(i64),
    None,
}

#[derive(Debug, Clone)]
pub struct Interpretation {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>,
}

impl Interpretation {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn run(&mut self, stmts: Vec<Stmt>) -> RuntimeResult<String> {
        let mut output = String::new();

        for stmt in stmts {
            match self.run_stmt(stmt, &mut output)? {
                ExecReturn::Return(_) => break,
                ExecReturn::None => {}
            }
        }

        Ok(output)
    }

    fn run_stmt(&mut self, stmt: Stmt, output: &mut String) -> RuntimeResult<ExecReturn> {
        let span = stmt.span.clone();
        match stmt.kind {
            StmtKind::VerDecl {
                name,
                value,
                mutable,
            } => {
                let v = self.run_expr(value, output)?;
                self.vars.insert(name, Value { value: v, mutable });
                Ok(ExecReturn::None)
            }

            StmtKind::Assign { name, value } => {
                let new_val = self.run_expr(value, output)?;

                match self.vars.get_mut(&name) {
                    Some(v) => {
                        if !v.mutable {
                            return Err(RuntimeError::ImmutableAssignment { name, span });
                        }
                        v.value = new_val;
                    }
                    None => {
                        self.vars.insert(
                            name,
                            Value {
                                value: new_val,
                                mutable: false,
                            },
                        );
                    }
                }
                Ok(ExecReturn::None)
            }

            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if self.run_expr(cond, output)? != 0 {
                    for stmt in then_branch {
                        match self.run_stmt(stmt, output)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        match self.run_stmt(stmt, output)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }
                }
                Ok(ExecReturn::None)
            }

            StmtKind::While { cond, body } => {
                while self.run_expr(cond.clone(), output)? != 0 {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt, output)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }
                }
                Ok(ExecReturn::None)
            }

            StmtKind::For {
                init,
                cond,
                incr,
                body,
            } => {
                self.run_stmt(*init, output)?;

                while self.run_expr(cond.clone(), output)? != 0 {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt, output)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }

                    if let Some(e) = incr.clone() {
                        self.run_expr(e, output)?;
                    }
                }
                Ok(ExecReturn::None)
            }

            StmtKind::Print(expr) => {
                let value = self.run_expr(expr, output)?;
                output.push_str(&format!("{}
", value));
                Ok(ExecReturn::None)
            }

            StmtKind::Expr(expr) => {
                self.run_expr(expr, output)?;
                Ok(ExecReturn::None)
            }

            StmtKind::Func { name, params, body } => {
                self.funcs.insert(name, (params, body));
                Ok(ExecReturn::None)
            }

            StmtKind::Return(expr) => {
                let value = self.run_expr(expr, output)?;
                Ok(ExecReturn::Return(value))
            }
        }
    }

    fn run_expr(&mut self, expr: Expr, output: &mut String) -> RuntimeResult<i64> {
        let span = expr.span.clone();
        match expr.kind {
            ExprKind::Num(n) => Ok(n),

            ExprKind::Ident(name) => self
                .vars
                .get(&name)
                .map(|v| v.value)
                .ok_or(RuntimeError::UndefinedVariable { name, span }),

            ExprKind::Plus(l, r) => Ok(self.run_expr(*l, output)? + self.run_expr(*r, output)?),
            ExprKind::Minus(l, r) => Ok(self.run_expr(*l, output)? - self.run_expr(*r, output)?),
            ExprKind::Star(l, r) => Ok(self.run_expr(*l, output)? * self.run_expr(*r, output)?),

            ExprKind::Slash(l, r) => {
                let rhs = self.run_expr(*r, output)?;
                if rhs == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(self.run_expr(*l, output)? / rhs)
            }

            ExprKind::Less(l, r) => Ok((self.run_expr(*l, output)? < self.run_expr(*r, output)?) as i64),
            ExprKind::LessEqual(l, r) => Ok((self.run_expr(*l, output)? <= self.run_expr(*r, output)?) as i64),
            ExprKind::Greater(l, r) => Ok((self.run_expr(*l, output)? > self.run_expr(*r, output)?) as i64),
            ExprKind::GreaterEqual(l, r) => Ok((self.run_expr(*l, output)? >= self.run_expr(*r, output)?) as i64),
            ExprKind::EqualEqual(l, r) => Ok((self.run_expr(*l, output)? == self.run_expr(*r, output)?) as i64),
            ExprKind::NotEqual(l, r) => Ok((self.run_expr(*l, output)? != self.run_expr(*r, output)?) as i64),

            ExprKind::PreInc(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                v.value += 1;
                Ok(v.value)
            }

            ExprKind::PreDec(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                v.value -= 1;
                Ok(v.value)
            }

            ExprKind::PostInc(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                let old = v.value;
                v.value += 1;
                Ok(old)
            }

            ExprKind::PostDec(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                let old = v.value;
                v.value -= 1;
                Ok(old)
            }

            ExprKind::Call { call, args } => {
                let name = match call.kind {
                    ExprKind::Ident(n) => n,
                    _ => {
                        return Err(RuntimeError::UndefinedFunction {
                            name: "<?>".to_string(),
                            span,
                        });
                    }
                };

                let (params, body) = self
                    .funcs
                    .get(&name)
                    .cloned()
                    .ok_or(RuntimeError::UndefinedFunction { name, span })?;

                if params.len() != args.len() {
                    return Err(RuntimeError::ArgumentMismatch {
                        expected: params.len(),
                        got: args.len(),
                    });
                }

                let mut local = Interpretation {
                    vars: HashMap::new(),
                    funcs: self.funcs.clone(),
                };

                for (param, arg_expr) in params.into_iter().zip(args.into_iter()) {
                    local.vars.insert(
                        param,
                        Value {
                            value: self.run_expr(arg_expr, output)?,
                            mutable: false,
                        },
                    );
                }

                for stmt in body {
                    match local.run_stmt(stmt, output)? {
                        ExecReturn::Return(v) => return Ok(v),
                        ExecReturn::None => {}
                    }
                }

                Ok(0)
            }
        }
    }
}
