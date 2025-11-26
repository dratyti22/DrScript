use crate::ast::Stmt;
use crate::lexer::Token;
use crate::parser::ParserToken;
use crate::type_error::{ParseError, ParseErrorKind, Span, TError, TokenPosition, TokensError};

impl ParserToken {
    /// вспомогательная функция для получения идентификатора
    fn expect_ident(&mut self) -> TError<String> {
        match self.advance() {
            Some(TokenPosition {
                token: Token::Ident(name),
                position: _,
            }) => Ok(name),
            Some(TokenPosition { token, position }) => Err(ParseError::new(
                ParseErrorKind::Tokens(TokensError::ExpectedToken(
                    Token::Ident(String::new()),
                    token,
                )),
                position,
                None,
                None,
            )),
            None => Err(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedEOF),
                Span::default(),
                None,
                None,
            )),
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
                token: Token::Ident(name),
                position: _,
            }) if name == "print" => self.parse_print(),
            Some(TokenPosition {
                token: Token::Ident(_),
                position: _,
            }) => {
                if matches!(self.peek_next().map(|t| &t.token), Some(&Token::Assign)) {
                    self.parse_assign()
                } else {
                    let expr = self.parse_expr()?;
                    self.expect(&Token::Semicolon)?;
                    Ok(Stmt::Expr(expr))
                }
            }

            Some(TokenPosition { token, position }) => Err(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedToken(token.clone())),
                position.clone(),
                None,
                None,
            )),
            None => Err(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedEOF),
                Span::default(),
                None,
                None,
            )),
        }
    }
    /// парсинг переменной
    pub(super) fn parse_var_decl(&mut self) -> TError<Stmt> {
        match self.advance() {
            Some(TokenPosition {
                token: Token::Var,
                position: _,
            }) => {
                let name = self.expect_ident()?;
                self.expect(&Token::Assign)?; // тут проверяем что после имени идет =
                let value = self.parse_expr()?; // тут парсим выражение
                self.expect(&Token::Semicolon)?; // тут проверяем что после выражения идет ;
                Ok(Stmt::Assign { name, value })
            }
            Some(TokenPosition {
                token: Token::Ver,
                position: _,
            }) => {
                let name = self.expect_ident()?;
                self.expect(&Token::Assign)?; // тут проверяем что после имени идет =
                let value = self.parse_expr()?; // тут парсим выражение
                self.expect(&Token::Semicolon)?; // тут проверяем что после выражения идет ;
                Ok(Stmt::VerDecl {
                    name,
                    value,
                    mutable: true,
                })
            }
            Some(TokenPosition { token, position }) => Err(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedToken(token)),
                position,
                None,
                None,
            )),
            _ => Err(ParseError::new(
                ParseErrorKind::Tokens(TokensError::UnexpectedEOF),
                Span::default(),
                None,
                None,
            )),
        }
    }
    /// выводит значение
    pub(super) fn parse_print(&mut self) -> TError<Stmt> {
        self.advance(); // пропускаем print
        self.expect(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        self.expect(&Token::Semicolon)?;
        Ok(Stmt::Print(expr))
    }
    /// по логике изменение значения переменной
    fn parse_assign(&mut self) -> TError<Stmt> {
        let name = self.expect_ident()?;
        self.expect(&Token::Assign)?;
        let value = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        Ok(Stmt::Assign { name, value })
    }
    /// парсинг функции
    fn parse_fun(&mut self) -> TError<Stmt> {
        self.expect(&Token::Fun)?;
        let name = self.expect_ident()?;
        self.expect(&Token::LParen)?;
        let mut params = Vec::new();

        if !matches!(self.peek().map(|t| &t.token), Some(&Token::RParen)) {
            loop {
                let param = self.expect_ident()?;
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
        self.expect(&Token::RBrace)?;
        Ok(Stmt::Func { name, params, body })
    }
    fn parse_return(&mut self) -> TError<Stmt> {
        self.expect(&Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    /// парсинг if
    fn parse_if(&mut self) -> TError<Stmt> {
        self.expect(&Token::If)?;

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

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> TError<Stmt> {
        self.expect(&Token::While)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::LBrace)?;
        let body = self.parse_block()?;
        self.expect(&Token::RBrace)?;
        Ok(Stmt::While { cond, body })
    }
    fn parse_for(&mut self) -> TError<Stmt> {
        self.expect(&Token::For)?;
        self.expect(&Token::LParen)?;

        let init = self.parse_stmt()?;

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
        self.expect(&Token::RBrace)?;

        Ok(Stmt::For {
            init: Box::new(init),
            cond,
            incr,
            body,
        })
    }

    fn parse_block(&mut self) -> TError<Vec<Stmt>> {
        let mut stmts = Vec::new();

        while let Some(TokenPosition { token, position: _ }) = self.peek() {
            if matches!(token, &Token::RBrace) {
                break;
            }

            match token {
                Token::Ident(name) if name == "print" => {
                    stmts.push(self.parse_print()?);
                }
                _ => stmts.push(self.parse_stmt()?),
            }
        }
        Ok(stmts)
    }
}
