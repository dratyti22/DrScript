use std::fmt::{Formatter, Display};
use crate::type_error::{Position, Span, TokenPosition};
use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\f]+")]
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
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("fun")]
    Fun,
    #[token("return")]
    Return,
    #[token(",")]
    Comma,
    #[token("\n")]
    NewLine,
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    Str(String),
    #[token("use")]
    Use,
}

pub fn lex(input: &str) -> Vec<TokenPosition> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;

    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        let slice = lexer.slice();
        let len = slice.len();

        match token {
            Ok(t) => {
                if matches!(t, Token::NewLine) {
                    line += 1;
                    column = 1;
                } else {
                    let start = Position { line, column };
                    let end = Position {
                        line,
                        column: column + len,
                    };
                    tokens.push(TokenPosition {
                        token: t,
                        position: Span { start, end },
                    })
                }
                column += len;
            }

            Err(_) => column += 1,
        }
    }
    tokens
}
