use crate::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::type_error::{RuntimeError, Span};
use crate::type_values::Type;
use std::collections::HashMap;

pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type BuiltinResult = Result<Type, RuntimeError>;
#[derive(Debug, Clone)]
pub struct BuiltinFun {
    pub name: &'static str,
    pub args: Option<usize>,
    pub func: fn(Vec<Type>, &mut String, Span) -> BuiltinResult,
}

#[derive(Debug, Clone)]
pub struct Value {
    value: Type,
    mutable: bool,
}

#[derive(Debug)]
enum ExecReturn {
    Return(Type),
    None,
}

#[derive(Debug, Clone)]
pub struct Interpretation {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    builtins: HashMap<String, BuiltinFun>,
}

impl Interpretation {
    pub fn new() -> Self {
        let mut i = Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            builtins: HashMap::new(),
        };
        i.register_builtin();
        i
    }
    pub(super) fn register_builtin(&mut self) {
        self.add_builtin(Self::print_builtin())
    }
    pub fn add_builtin(&mut self, b: BuiltinFun) {
        self.builtins.insert(b.name.to_string(), b);
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
                if self.run_expr(cond, output)? != Type::Int(0) {
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
                while self.run_expr(cond.clone(), output)? != Type::Int(0) {
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

                while self.run_expr(cond.clone(), output)? != Type::Int(0) {
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

    fn run_expr(&mut self, expr: Expr, output: &mut String) -> RuntimeResult<Type> {
        let span = expr.span.clone();
        match expr.kind {
            ExprKind::Num(n) => Ok(Type::Int(n)),
            ExprKind::Str(s) => Ok(Type::Str(s)),

            ExprKind::Ident(name) => self
                .vars
                .get(&name)
                .map(|v| v.value.clone())
                .ok_or(RuntimeError::UndefinedVariable { name, span }),

            ExprKind::Plus(l, r) => Ok(self.run_expr(*l, output)? + self.run_expr(*r, output)?),
            ExprKind::Minus(l, r) => Ok(self.run_expr(*l, output)? - self.run_expr(*r, output)?),
            ExprKind::Star(l, r) => Ok(self.run_expr(*l, output)? * self.run_expr(*r, output)?),

            ExprKind::Slash(l, r) => {
                let rhs = self.run_expr(*r, output)?;
                if rhs.is_zero() {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(self.run_expr(*l, output)? / rhs)
            }

            ExprKind::Less(l, r) => Ok(Type::Int(
                (self.run_expr(*l, output)? < self.run_expr(*r, output)?) as i64,
            )),
            ExprKind::LessEqual(l, r) => Ok(Type::Int(
                (self.run_expr(*l, output)? <= self.run_expr(*r, output)?) as i64,
            )),
            ExprKind::Greater(l, r) => Ok(Type::Int(
                (self.run_expr(*l, output)? > self.run_expr(*r, output)?) as i64,
            )),
            ExprKind::GreaterEqual(l, r) => Ok(Type::Int(
                (self.run_expr(*l, output)? >= self.run_expr(*r, output)?) as i64,
            )),
            ExprKind::EqualEqual(l, r) => Ok(Type::Int(
                (self.run_expr(*l, output)? == self.run_expr(*r, output)?) as i64,
            )),
            ExprKind::NotEqual(l, r) => Ok(Type::Int(
                (self.run_expr(*l, output)? != self.run_expr(*r, output)?) as i64,
            )),

            ExprKind::PreInc(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                v.value += Type::Int(1);
                Ok(v.value.clone())
            }

            ExprKind::PreDec(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                v.value -= Type::Int(1);
                Ok(v.value.clone())
            }

            ExprKind::PostInc(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                let old = v.value.clone();
                v.value += Type::Int(1);
                Ok(old)
            }

            ExprKind::PostDec(name) => {
                let v = self
                    .vars
                    .get_mut(&name)
                    .ok_or(RuntimeError::UndefinedVariable { name, span })?;
                let old = v.value.clone();
                v.value -= Type::Int(1);
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

                // Для системных функций
                let mut vec_value: Vec<Type> = Vec::new();
                for v in args.clone() {
                    let t = self.run_expr(v, output)?;
                    vec_value.push(t);
                }
                if let Some(builtin) = self.builtins.get(&name) {
                    if let Some(arity) = builtin.args {
                        if args.len() != arity {
                            return Err(RuntimeError::ArgumentMismatch {
                                expected: arity,
                                got: vec_value.len(),
                            });
                        }
                    }
                    return (builtin.func)(vec_value, output, span);
                }

                // Для обычных функций
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
                    builtins: self.builtins.clone(),
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

                Ok(Type::Int(0))
            }
        }
    }
}
