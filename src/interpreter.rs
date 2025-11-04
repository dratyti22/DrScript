use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Value {
    value: i64,
    mutable: bool,
}

enum ExecReturn {
    Return(i64),
    None,
}

#[derive(Debug, Clone)]
pub struct Interpretation {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    output: String,
}

impl Interpretation {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            output: String::new(),
        }
    }
    pub fn run(&mut self, stmts: Vec<Stmt>)->String {
        for stmt in stmts {
            match self.run_stmt(stmt) {
                ExecReturn::Return(_) => break,
                ExecReturn::None => continue,
            }
        }
        self.output.clone()
    }

    fn run_stmt(&mut self, stmt: Stmt) -> ExecReturn {
        match stmt {
            Stmt::VerDecl {
                name,
                value,
                mutable,
            } => {
                let v = self.run_expr(value);
                self.vars.insert(name, Value { value: v, mutable });
                ExecReturn::None
            }
            Stmt::Assign { name, value } => {
                if self.vars.contains_key(&name) {
                    if self.vars.get(&name).unwrap().mutable {
                        self.vars.get_mut(&name).unwrap().value = self.run_expr(value.clone());
                        return ExecReturn::None;
                    } else {
                        panic!("Variable {} is not mutable", name);
                    }
                }
                let v = self.run_expr(value);

                self.vars.insert(
                    name,
                    Value {
                        value: v,
                        mutable: false,
                    },
                );
                ExecReturn::None
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if self.run_expr(cond) != 0 {
                    for stmt in then_branch {
                        match self.run_stmt(stmt) {
                            ExecReturn::Return(v) => return ExecReturn::Return(v),
                            ExecReturn::None => {}
                        }
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        match self.run_stmt(stmt) {
                            ExecReturn::Return(v) => return ExecReturn::Return(v),
                            ExecReturn::None => {}
                        }
                    }
                }
                ExecReturn::None
            }
            Stmt::While { cond, body } => {
                while self.run_expr(cond.clone()) != 0 {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt) {
                            ExecReturn::Return(v) => return ExecReturn::Return(v),
                            ExecReturn::None => {}
                        }
                    }
                }
                ExecReturn::None
            }
            Stmt::For {
                init,
                cond,
                incr,
                body,
            } => {
                self.run_stmt(*init);

                while self.run_expr(cond.clone()) != 0 {
                    for stmt in body.clone() {
                        match self.run_stmt(stmt) {
                            ExecReturn::Return(v) => return ExecReturn::Return(v),
                            ExecReturn::None => {}
                        }
                    }
                    if let Some(expr) = incr.clone() {
                        self.run_expr(expr);
                    }
                }
                ExecReturn::None
            }
            Stmt::Print(expr) => {
                let r = self.run_expr(expr);
                self.output.push_str(&format!("{}\n",r));
                ExecReturn::None
            }
            Stmt::Expr(expr) => {
                self.run_expr(expr);
                ExecReturn::None
            }
            Stmt::Func { name, params, body } => {
                self.funcs.insert(name, (params, body));
                ExecReturn::None
            }
            Stmt::Return(expr) => {
                let val = self.run_expr(expr);
                ExecReturn::Return(val)
            }
        }
    }
    fn run_expr(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Num(n) => n,
            Expr::Ident(n) => {
                if !self.vars.contains_key(&n) {
                    panic!("Variable {} not declared, Ident", n);
                } else {
                    let a = self.vars.get(&n).unwrap().clone();
                    a.value
                }
            }
            Expr::Plus(l, r) => self.run_expr(*l) + self.run_expr(*r),
            Expr::Minus(l, r) => self.run_expr(*l) - self.run_expr(*r),
            Expr::Star(l, r) => self.run_expr(*l) * self.run_expr(*r),
            Expr::Slash(l, r) => self.run_expr(*l) / self.run_expr(*r),
            Expr::Less(l, r) => (self.run_expr(*l) < self.run_expr(*r)) as i64,
            Expr::LessEqual(l, r) => (self.run_expr(*l) <= self.run_expr(*r)) as i64,
            Expr::Greater(l, r) => (self.run_expr(*l) > self.run_expr(*r)) as i64,
            Expr::GreaterEqual(l, r) => (self.run_expr(*l) >= self.run_expr(*r)) as i64,
            Expr::EqualEqual(l, r) => (self.run_expr(*l) == self.run_expr(*r)) as i64,
            Expr::NotEqual(l, r) => (self.run_expr(*l) != self.run_expr(*r)) as i64,

            Expr::PreInc(n) => {
                let val = self.vars.get_mut(&n).unwrap();
                let new_val = val.value + 1;
                val.value = new_val;
                new_val
            }
            Expr::PreDec(n) => {
                let val = self.vars.get_mut(&n).unwrap();
                let new_val = val.value - 1;
                val.value = new_val;
                new_val
            }
            Expr::PostInc(n) => {
                let val = self.vars.get_mut(&n).unwrap();
                let old_val = val.value;
                val.value += 1;
                old_val
            }
            Expr::PostDec(n) => {
                let val = self.vars.get_mut(&n).unwrap();
                let old_val = val.value;
                val.value -= 1;
                old_val
            }
            Expr::Call { call, args } => {
                let name = match *call {
                    Expr::Ident(n) => n,
                    _ => panic!("Expected identifier"),
                };
                // получаем параметры и тело функции
                let (params, body) = self.funcs.get(&name).unwrap().clone();
                // сравниваем что бы аргументов было столькоже сколько и принмает функции
                if params.len() != args.len() {
                    panic!(
                        "Func {} expected: {} but got args: {}",
                        name,
                        params.len(),
                        args.len()
                    );
                }
                //создаем локальные переменные
                let mut local = Interpretation {
                    vars: HashMap::new(),
                    funcs: self.funcs.clone(),
                    output: self.output.clone(),
                };
                // заполняем локальные переменные данными
                for (i, arg) in params.into_iter().zip(args.into_iter()) {
                    local.vars.insert(
                        i,
                        Value {
                            value: self.run_expr(arg),
                            mutable: false,
                        },
                    );
                }
                // выполняем тело функции
                for stmt in body {
                    match local.run_stmt(stmt) {
                        ExecReturn::Return(v) => {
                            return v;
                        }
                        ExecReturn::None => continue,
                    }
                }
                0
            }
        }
    }
}
