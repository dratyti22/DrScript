use crate::type_error::Span;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub kind: T,
    pub span: Span,
}

pub type Expr = Node<ExprKind>;
pub type Stmt = Node<StmtKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Num(i64),
    Ident(String),
    Str(String),
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
    PreInc(String),
    PreDec(String),
    PostInc(String),
    PostDec(String),
    Call { call: Box<Expr>, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Expr(Expr),
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
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Box<Stmt>,
        cond: Expr,
        incr: Option<Expr>,
        body: Vec<Stmt>,
    },
    Func {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Expr),
}
