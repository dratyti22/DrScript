#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(i64),
    Ident(String),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Star(Box<Expr>, Box<Expr>),
    Slash(Box<Expr>, Box<Expr>),
    Print(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Print(Expr),
    VerDecl {
        name: String,
        value: Expr,
        mutable: bool,
    },
    Assign {
        name: String,
        value: Expr,
    },
}
