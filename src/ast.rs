#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(i64),
    Ident(String),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Star(Box<Expr>, Box<Expr>),
    Slash(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    EqualEqual(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
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
