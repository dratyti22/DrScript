use crate::ast::{Expr, Stmt};
use std::collections::HashMap;
use std::fmt;
use crate::type_error::RuntimeError;

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
        let output = String::new();

        for stmt in stmts {
            match self.run_stmt(stmt)? {
                ExecReturn::Return(_) => break,
                ExecReturn::None => {}
            }
        }

        Ok(output)
    }

    fn run_stmt(&mut self, stmt: Stmt) -> RuntimeResult<ExecReturn> {
        match stmt {
            Stmt::VerDecl {
                name,
                value,
                mutable,
            } => {
                let v = self.run_expr(value)?;
                self.vars.insert(name, Value { value: v, mutable });
                Ok(ExecReturn::None)
            }

            Stmt::Assign { name, value } => {
                let new_val = self.run_expr(value)?;

                match self.vars.get_mut(&name) {
                    Some(v) => {
                        if !v.mutable {
                            return Err(RuntimeError::ImmutableAssignment(name));
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

            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if self.run_expr(cond)? != 0 {
                    for stmt in then_branch {
                        match self.run_stmt(stmt)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        match self.run_stmt(stmt)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }
                }
                Ok(ExecReturn::None)
            }

            Stmt::While { cond, body } => {
                while self.run_expr(cond.clone())? != 0 {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }
                }
                Ok(ExecReturn::None)
            }

            Stmt::For {
                init,
                cond,
                incr,
                body,
            } => {
                self.run_stmt(*init)?;

                while self.run_expr(cond.clone())? != 0 {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt)? {
                            ExecReturn::Return(v) => return Ok(ExecReturn::Return(v)),
                            ExecReturn::None => {}
                        }
                    }

                    if let Some(e) = incr.clone() {
                        self.run_expr(e)?;
                    }
                }
                Ok(ExecReturn::None)
            }

            Stmt::Print(expr) => {
                let value = self.run_expr(expr)?;
                println!("{}", value);
                Ok(ExecReturn::None)
            }

            Stmt::Expr(expr) => {
                self.run_expr(expr)?;
                Ok(ExecReturn::None)
            }

            Stmt::Func { name, params, body } => {
                self.funcs.insert(name, (params, body));
                Ok(ExecReturn::None)
            }

            Stmt::Return(expr) => {
                let value = self.run_expr(expr)?;
                Ok(ExecReturn::Return(value))
            }
        }
    }

    fn run_expr(&mut self, expr: Expr) -> RuntimeResult<i64> {
        match expr {
            Expr::Num(n) => Ok(n),

            Expr::Ident(name) => self
                .vars
                .get(&name)
                .map(|v| v.value)
                .ok_or(RuntimeError::UndefinedVariable(name)),

            Expr::Plus(l, r) => Ok(self.run_expr(*l)? + self.run_expr(*r)?),
            Expr::Minus(l, r) => Ok(self.run_expr(*l)? - self.run_expr(*r)?),
            Expr::Star(l, r) => Ok(self.run_expr(*l)? * self.run_expr(*r)?),

            Expr::Slash(l, r) => {
                let rhs = self.run_expr(*r)?;
                if rhs == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(self.run_expr(*l)? / rhs)
            }

            Expr::Less(l, r) => Ok((self.run_expr(*l)? < self.run_expr(*r)?) as i64),
            Expr::LessEqual(l, r) => Ok((self.run_expr(*l)? <= self.run_expr(*r)?) as i64),
            Expr::Greater(l, r) => Ok((self.run_expr(*l)? > self.run_expr(*r)?) as i64),
            Expr::GreaterEqual(l, r) => Ok((self.run_expr(*l)? >= self.run_expr(*r)?) as i64),
            Expr::EqualEqual(l, r) => Ok((self.run_expr(*l)? == self.run_expr(*r)?) as i64),
            Expr::NotEqual(l, r) => Ok((self.run_expr(*l)? != self.run_expr(*r)?) as i64),

            Expr::PreInc(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable(name.clone()))?;
                v.value += 1;
                Ok(v.value)
            }

            Expr::PreDec(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable(name.clone()))?;
                v.value -= 1;
                Ok(v.value)
            }

            Expr::PostInc(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable(name.clone()))?;
                let old = v.value;
                v.value += 1;
                Ok(old)
            }

            Expr::PostDec(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable(name.clone()))?;
                let old = v.value;
                v.value -= 1;
                Ok(old)
            }

            Expr::Call { call, args } => {
                let name = match *call {
                    Expr::Ident(n) => n,
                    _ => return Err(RuntimeError::UndefinedFunction("<?>".into())),
                };

                let (params, body) = self
                    .funcs
                    .get(&name)
                    .cloned()
                    .ok_or(RuntimeError::UndefinedFunction(name.clone()))?;

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
                            value: self.run_expr(arg_expr)?,
                            mutable: false,
                        },
                    );
                }

                for stmt in body {
                    match local.run_stmt(stmt)? {
                        ExecReturn::Return(v) => return Ok(v),
                        ExecReturn::None => {}
                    }
                }

                Ok(0)
            }
        }
    }
}
