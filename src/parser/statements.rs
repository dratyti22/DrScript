use crate::ast::{Stmt, StmtKind};
use crate::lexer::Token;
use crate::parser::ParserToken;
use crate::type_error::{ParseError, ParseErrorKind, Span, TError, TokenPosition, TokensError};

impl ParserToken {
    /// создание stmt
    fn make_stmt(&self, kind: StmtKind, span: Span) -> Stmt {
        Stmt { kind, span }
    }

    /// вспомогательная функция для получения идентификатора
    fn expect_ident(&mut self) -> TError<(String, Span)> {
        match self.advance() {
            Some(TokenPosition {
                token: Token::Ident(name),
                position,
            }) => Ok((name, position)),
            Some(TokenPosition { token, position }) => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::ExpectedToken(
                    Token::Ident(String::new()),
                    token,
                )),
                position,
                None,
                None,
            ))),
            None => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedEOF),
                Span::default(),
                None,
                None,
            ))),
        }
    }
    /// проверка того что приходит на вход
    pub(super) fn parse_stmt(&mut self) -> TError<Stmt> {
        match self.peek() {
            Some(TokenPosition {
                token: Token::If,
                position: _,
            }) => self.parse_if(),
            Some(TokenPosition {
                token: Token::While,
                position: _,
            }) => self.parse_while(),
            Some(TokenPosition {
                token: Token::For,
                position: _,
            }) => self.parse_for(),
            Some(TokenPosition {
                token: Token::Fun,
                position: _,
            }) => self.parse_fun(),
            Some(TokenPosition {
                token: Token::Return,
                position: _,
            }) => self.parse_return(),
            Some(TokenPosition {
                token: Token::Var,
                position: _,
            })
            | Some(TokenPosition {
                token: Token::Ver,
                position: _,
            }) => self.parse_var_decl(),
            Some(TokenPosition {
                token: Token::Ident(_),
                position: _,
            }) => {
                if matches!(self.peek_next().map(|t| &t.token), Some(&Token::Assign)) {
                    self.parse_assign()
                } else {
                    let expr = self.parse_expr()?;
                    let span = expr.span.clone();
                    self.expect(&Token::Semicolon)?;
                    Ok(self.make_stmt(StmtKind::Expr(expr), span))
                }
            }
            Some(TokenPosition {
                token: Token::Use,
                position: _,
            }) => self.parse_use(),

            Some(TokenPosition { token, position }) => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedToken(token.clone())),
                position.clone(),
                None,
                None,
            ))),
            None => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedEOF),
                Span::default(),
                None,
                None,
            ))),
        }
    }
    /// парсиег импорта
    pub(super) fn parse_use(&mut self) -> TError<Stmt> {
        let start = self.expect(&Token::Use)?;
        let file= self.advance().unwrap();
        let _= self.advance().unwrap();
        let file_name = match file.token {
            Token::Ident(name) => name,
            _ => return Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::ExpectedToken(
                    Token::Ident(String::new()),
                    file.token,
                )),
                file.position,
                None,
                None,
            ))),
        };
        let name = format!("{}.dr", file_name);
        let end = self.expect(&Token::Semicolon)?;
        let span = self.merge_span(&start.position, &end.position);
        Ok(self.make_stmt(StmtKind::Use(name), span))
    }

    /// парсинг переменной
    pub(super) fn parse_var_decl(&mut self) -> TError<Stmt> {
        match self.advance() {
            Some(TokenPosition {
                token: Token::Var,
                position,
            }) => {
                let (name, _) = self.expect_ident()?;
                self.expect(&Token::Assign)?; // тут проверяем что после имени идет =
                let value = self.parse_expr()?; // тут парсим выражение
                let semi = self.expect(&Token::Semicolon)?; // тут проверяем что после выражения идет ;
                let span = self.merge_span(&position, &semi.position);
                Ok(self.make_stmt(StmtKind::Assign { name, value }, span))
            }
            Some(TokenPosition {
                token: Token::Ver,
                position,
            }) => {
                let (name, _) = self.expect_ident()?;
                self.expect(&Token::Assign)?; // тут проверяем что после имени идет =
                let value = self.parse_expr()?; // тут парсим выражение
                let semi = self.expect(&Token::Semicolon)?; // тут проверяем что после выражения идет ;
                let span = self.merge_span(&position, &semi.position);
                Ok(self.make_stmt(
                    StmtKind::VerDecl {
                        name,
                        value,
                        mutable: true,
                    },
                    span,
                ))
            }
            Some(TokenPosition { token, position }) => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedToken(token)),
                position,
                None,
                None,
            ))),
            _ => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedEOF),
                Span::default(),
                None,
                None,
            ))),
        }
    }
    /// по логике изменение значения переменной
    fn parse_assign(&mut self) -> TError<Stmt> {
        let (name, start) = self.expect_ident()?;
        self.expect(&Token::Assign)?;
        let value = self.parse_expr()?;
        let end = self.expect(&Token::Semicolon)?;
        let span = self.merge_span(&start, &end.position);
        Ok(self.make_stmt(StmtKind::Assign { name, value }, span))
    }
    /// парсинг функции
    fn parse_fun(&mut self) -> TError<Stmt> {
        let start = self.expect(&Token::Fun)?;
        let (name, _) = self.expect_ident()?;
        self.expect(&Token::LParen)?;
        let mut params = Vec::new();

        if !matches!(self.peek().map(|t| &t.token), Some(&Token::RParen)) {
            loop {
                let (param, _) = self.expect_ident()?;
                params.push(param);
                if matches!(self.peek().map(|t| &t.token), Some(&Token::Comma)) {
                    self.advance();
                }
                if matches!(self.peek().map(|t| &t.token), Some(&Token::RParen)) {
                    break;
                }
            }
        }

        self.expect(&Token::RParen)?;
        self.expect(&Token::LBrace)?;
        let body = self.parse_block()?;
        let end = self.expect(&Token::RBrace)?;
        let span = self.merge_span(&start.position, &end.position);
        Ok(self.make_stmt(StmtKind::Func { name, params, body }, span))
    }
    fn parse_return(&mut self) -> TError<Stmt> {
        let start = self.expect(&Token::Return)?;
        let expr = self.parse_expr()?;
        let end = self.expect(&Token::Semicolon)?;
        let span = self.merge_span(&start.position, &end.position);
        Ok(self.make_stmt(StmtKind::Return(expr), span))
    }

    /// парсинг if
    fn parse_if(&mut self) -> TError<Stmt> {
        let start = self.expect(&Token::If)?;

        let cond = self.parse_expr()?;
        self.expect(&Token::LBrace)?;
        let then_branch = self.parse_block()?;
        self.expect(&Token::RBrace)?;

        let else_branch = if matches!(self.peek().map(|t| &t.token), Some(&Token::Else)) {
            self.advance(); // пропускаем else
            self.expect(&Token::LBrace)?;
            let block = self.parse_block()?;
            self.expect(&Token::RBrace)?;
            Some(block)
        } else {
            None
        };
        let end = if let Some(else_block) = &else_branch {
            else_block.last().map(|t| &t.span).unwrap_or(&cond.span)
        } else {
            then_branch.last().map(|t| &t.span).unwrap_or(&cond.span)
        };

        let span = self.merge_span(&start.position, end);
        Ok(self.make_stmt(
            StmtKind::If {
                cond,
                then_branch,
                else_branch,
            },
            span,
        ))
    }

    fn parse_while(&mut self) -> TError<Stmt> {
        let start = self.expect(&Token::While)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::LBrace)?;
        let body = self.parse_block()?;
        let rbrace = self.expect(&Token::RBrace)?;
        let span = self.merge_span(&start.position, &rbrace.position);
        Ok(self.make_stmt(StmtKind::While { cond, body }, span))
    }
    fn parse_for(&mut self) -> TError<Stmt> {
        let start = self.expect(&Token::For)?;
        self.expect(&Token::LParen)?;

        let init = self.parse_name_in_for()?;

        let cond = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        let incr = if matches!(self.peek().map(|t| &t.token), Some(&Token::RParen)) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(&Token::RParen)?;
        self.expect(&Token::LBrace)?;
        let body = self.parse_block()?;
        let rbrace = self.expect(&Token::RBrace)?;
        let span = self.merge_span(&start.position, &rbrace.position);

        Ok(self.make_stmt(
            StmtKind::For {
                init: Box::new(init),
                cond,
                incr,
                body,
            },
            span,
        ))
    }

    fn parse_block(&mut self) -> TError<Vec<Stmt>> {
        let mut stmts = Vec::new();

        while let Some(TokenPosition { token, position: _ }) = self.peek() {
            if matches!(token, &Token::RBrace) {
                break;
            }

            stmts.push(self.parse_stmt()?)
        }
        Ok(stmts)
    }
    fn parse_name_in_for(&mut self) -> TError<Stmt> {
        let (name, start) = self.expect_ident()?;
        self.expect(&Token::Assign)?; // тут проверяем что после имени идет =
        let value = self.parse_expr()?; // тут парсим выражение
        let end = self.expect(&Token::Semicolon)?; // тут проверяем что после выражения идет ;
        let span = self.merge_span(&start, &end.position);
        Ok(self.make_stmt(
            StmtKind::VerDecl {
                name,
                value,
                mutable: true,
            },
            span,
        ))
    }
}
