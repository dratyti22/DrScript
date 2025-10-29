use crate::ast::{Expr, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Value {
    value: i64,
    mutable: bool,
}

#[derive(Debug, Clone)]
pub struct Interpretation {
    vars: HashMap<String, Value>,
}

impl Interpretation {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn run(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.run_stmt(stmt)
        }
    }

    fn run_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::VerDecl {
                name,
                value,
                mutable,
            } => {
                let v = self.run_expr(value);
                self.vars.insert(name, Value { value: v, mutable });
            }
            Stmt::Assign { name, value } => {
                if self.vars.contains_key(&name) {
                    if self.vars.get(&name).unwrap().mutable {
                        self.vars.get_mut(&name).unwrap().value = self.run_expr(value.clone());
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
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if self.run_expr(cond) != 0 {
                    for stmt in then_branch {
                        self.run_stmt(stmt);
                    }
                } else if let Some(branch) = else_branch {
                    for stmt in branch {
                        self.run_stmt(stmt);
                    }
                }
            }
            Stmt::Print(expr) => println!("{:?}", self.run_expr(expr)),
        }
    }
    fn run_expr(&self, expr: Expr) -> i64 {
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
        }
    }
}
