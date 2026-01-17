use crate::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::io::{DrScriptIo, IoTerminal};
use crate::type_error::{RuntimeError, Span};
use crate::type_values::Type;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type BuiltinResult = Result<Type, RuntimeError>;
#[derive(Debug, Clone)]
pub struct BuiltinFun {
    pub name: &'static str,
    pub args: Option<usize>,
    pub func: fn(Vec<Type>, &mut dyn DrScriptIo, Span) -> BuiltinResult,
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

#[derive(Debug)]
pub struct Interpretation {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    builtins: HashMap<String, BuiltinFun>,
    io: Rc<RefCell<dyn DrScriptIo>>,
    current_file: PathBuf,
    loaded_files: HashSet<PathBuf>,
}

impl Interpretation {
    pub fn new(io: Option<Rc<RefCell<dyn DrScriptIo>>>) -> Self {
        let mut i = Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            builtins: HashMap::new(),
            io: io.unwrap_or(Rc::new(RefCell::new(IoTerminal))),
            loaded_files: HashSet::new(),
            current_file: PathBuf::new(),
        };
        i.register_builtin();
        i
    }
    pub fn get_vars_funcs(
        &self,
    ) -> (
        &HashMap<String, Value>,
        &HashMap<String, (Vec<String>, Vec<Stmt>)>,
    ) {
        (&self.vars, &self.funcs)
    }

    pub fn add_builtin(&mut self, b: BuiltinFun) {
        self.builtins.insert(b.name.to_string(), b);
    }
    pub fn run(&mut self, stmts: Vec<Stmt>) -> RuntimeResult<()> {
        for stmt in stmts {
            match self.run_stmt(stmt)? {
                ExecReturn::Return(_) => break,
                ExecReturn::None => {}
            }
        }

        Ok(())
    }

    fn run_stmt(&mut self, stmt: Stmt) -> RuntimeResult<ExecReturn> {
        let span = stmt.span.clone();
        match stmt.kind {
            StmtKind::VerDecl {
                name,
                value,
                mutable,
            } => {
                let v = self.run_expr(value)?;
                self.vars.insert(name, Value { value: v, mutable });
                Ok(ExecReturn::None)
            }

            StmtKind::Assign { name, value } => {
                let new_val = self.run_expr(value)?;

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
                if self.run_expr(cond)? != Type::Int(0) {
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

            StmtKind::While { cond, body } => {
                while self.run_expr(cond.clone())? != Type::Int(0) {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt)? {
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
                self.run_stmt(*init)?;

                while self.run_expr(cond.clone())? != Type::Int(0) {
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

            StmtKind::Expr(expr) => {
                self.run_expr(expr)?;
                Ok(ExecReturn::None)
            }

            StmtKind::Func { name, params, body } => {
                self.funcs.insert(name, (params, body));
                Ok(ExecReturn::None)
            }

            StmtKind::Return(expr) => {
                let value = self.run_expr(expr)?;
                Ok(ExecReturn::Return(value))
            }
            StmtKind::Use(name) => {
                let path = std::env::current_dir()
                    .unwrap()
                    .to_path_buf()
                    .join(name);
                if self.loaded_files.contains(&path) {
                    return Ok(ExecReturn::None);
                }
                fs::canonicalize(&path).map_err(|_| RuntimeError::ImportNotFount {
                    name: path.to_str().unwrap().to_string(),
                    span: span.clone(),
                })?;
                self.loaded_files.insert(path.clone());
                let code =
                    std::fs::read_to_string(&path).map_err(|_| RuntimeError::ImportNotFount {
                        name: path.to_str().unwrap().to_string(),
                        span,
                    })?;
                let (ver, func) = self.import_use(code.as_str(), self.io.clone())?;

                for (name, value) in ver {
                    self.vars.insert(name.clone(), value.clone());
                }
                for (name, (param, body)) in func {
                    self.funcs
                        .insert(name.clone(), (param.clone(), body.clone()));
                }

                Ok(ExecReturn::None)
            }
        }
    }

    fn run_expr(&mut self, expr: Expr) -> RuntimeResult<Type> {
        let span = expr.span.clone();
        match expr.kind {
            ExprKind::Num(n) => Ok(Type::Int(n)),
            ExprKind::Str(s) => Ok(Type::Str(s)),

            ExprKind::Ident(name) => self
                .vars
                .get(&name)
                .map(|v| v.value.clone())
                .ok_or(RuntimeError::UndefinedVariable { name, span }),

            ExprKind::Plus(l, r) => Ok(self.run_expr(*l)? + self.run_expr(*r)?),
            ExprKind::Minus(l, r) => Ok(self.run_expr(*l)? - self.run_expr(*r)?),
            ExprKind::Star(l, r) => Ok(self.run_expr(*l)? * self.run_expr(*r)?),

            ExprKind::Slash(l, r) => {
                let rhs = self.run_expr(*r)?;
                if rhs.is_zero() {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(self.run_expr(*l)? / rhs)
            }

            ExprKind::Less(l, r) => Ok(Type::Int((self.run_expr(*l)? < self.run_expr(*r)?) as i64)),
            ExprKind::LessEqual(l, r) => {
                Ok(Type::Int((self.run_expr(*l)? <= self.run_expr(*r)?) as i64))
            }
            ExprKind::Greater(l, r) => {
                Ok(Type::Int((self.run_expr(*l)? > self.run_expr(*r)?) as i64))
            }
            ExprKind::GreaterEqual(l, r) => {
                Ok(Type::Int((self.run_expr(*l)? >= self.run_expr(*r)?) as i64))
            }
            ExprKind::EqualEqual(l, r) => {
                Ok(Type::Int((self.run_expr(*l)? == self.run_expr(*r)?) as i64))
            }
            ExprKind::NotEqual(l, r) => {
                Ok(Type::Int((self.run_expr(*l)? != self.run_expr(*r)?) as i64))
            }

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
                    let t = self.run_expr(v)?;
                    vec_value.push(t);
                }
                if let Some(builtin) = self.builtins.get(&name) {
                    if let Some(arity) = builtin.args
                        && args.len() != arity
                    {
                        return Err(RuntimeError::ArgumentMismatch {
                            expected: arity,
                            got: vec_value.len(),
                        });
                    }

                    return (builtin.func)(vec_value, &mut *self.io.borrow_mut(), span);
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
                    io: self.io.clone(),
                    current_file: self.current_file.clone(),
                    loaded_files: self.loaded_files.clone(),
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

                Ok(Type::Int(0))
            }
        }
    }
}
