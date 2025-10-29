use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[\t\n\f]+")]
#[logos(skip r"//[^\n]*")]
pub enum Token {
    #[token("var")]
    Var,
    #[token("ver")]
    Ver,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Number(i64),
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
}

pub fn lex(input: &str) -> Vec<Token> {
    Token::lexer(input).filter_map(|x| x.ok()).collect()
}
