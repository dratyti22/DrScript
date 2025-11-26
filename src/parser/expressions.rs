use crate::ast::Expr;
use crate::lexer::Token;
use crate::parser::ParserToken;
use crate::type_error::{ExpressionError, ParseError, ParseErrorKind, TError, TokenPosition};

impl ParserToken {
    /// получение выражения за =
    pub(super) fn parse_expr(&mut self) -> TError<Expr> {
        let mut left = self.parse_less_greater()?;

        while let Some(tok) = self.peek() {
            match tok {
                TokenPosition {
                    token: Token::Equal,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_less_greater()?;
                    left = Expr::EqualEqual(Box::new(left), Box::new(right));
                }
                TokenPosition {
                    token: Token::NotEqual,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_less_greater()?;
                    left = Expr::NotEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// отвечает за < <= >=
    fn parse_less_greater(&mut self) -> TError<Expr> {
        let mut left = self.parse_plus_minus()?;

        while let Some(tok) = self.peek() {
            match tok {
                TokenPosition {
                    token: Token::Less,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_plus_minus()?;
                    left = Expr::Less(Box::new(left), Box::new(right));
                }
                TokenPosition {
                    token: Token::LessEqual,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_plus_minus()?;
                    left = Expr::LessEqual(Box::new(left), Box::new(right));
                }
                TokenPosition {
                    token: Token::Greater,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_plus_minus()?;
                    left = Expr::Greater(Box::new(left), Box::new(right));
                }
                TokenPosition {
                    token: Token::GreaterEqual,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_plus_minus()?;
                    left = Expr::GreaterEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// отвечает за + и -
    fn parse_plus_minus(&mut self) -> TError<Expr> {
        let mut left = self.parse_star_slash()?;

        while let Some(tok) = self.peek() {
            match tok {
                TokenPosition {
                    token: Token::Plus,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_star_slash()?;
                    left = Expr::Plus(Box::new(left), Box::new(right));
                }
                TokenPosition {
                    token: Token::Minus,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_star_slash()?;
                    left = Expr::Minus(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// отвечает за * и /
    fn parse_star_slash(&mut self) -> TError<Expr> {
        let mut left = self.parse_unary()?;

        while let Some(tok) = self.peek() {
            match tok {
                TokenPosition {
                    token: Token::Star,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Star(Box::new(left), Box::new(right));
                }
                TokenPosition {
                    token: Token::Slash,
                    position: _,
                } => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Slash(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// парсинг значений ++ --
    fn parse_unary(&mut self) -> TError<Expr> {
        // Префиксные ++ / --
        if let Some(tok) = self.peek() {
            match tok {
                TokenPosition {
                    token: Token::PlusPlus,
                    position: _,
                } => {
                    self.advance();
                    let expr = self.parse_primary()?;
                    match expr {
                        Expr::Ident(name) => return Ok(Expr::PreInc(name)),
                        _ => panic!("Префикс ++ можно применять только к переменным"),
                    }
                }
                TokenPosition {
                    token: Token::MinusMinus,
                    position: _,
                } => {
                    self.advance();
                    let expr = self.parse_primary()?;
                    match expr {
                        Expr::Ident(name) => return Ok(Expr::PreDec(name)),
                        _ => panic!("Префикс -- можно применять только к переменным"),
                    }
                }
                _ => {}
            }
        }

        // Основное выражение
        let mut expr = self.parse_primary()?;

        // Постфиксные ++ / --
        loop {
            match self.peek() {
                Some(TokenPosition {
                    token: Token::PlusPlus,
                    position: _,
                }) => {
                    self.advance();
                    match expr {
                        Expr::Ident(ref name) => expr = Expr::PostInc(name.clone()),
                        _ => panic!("Постфикс ++ можно применять только к переменным"),
                    }
                }
                Some(TokenPosition {
                    token: Token::MinusMinus,
                    position: _,
                }) => {
                    self.advance();
                    match expr {
                        Expr::Ident(ref name) => expr = Expr::PostDec(name.clone()),
                        _ => panic!("Постфикс -- можно применять только к переменным"),
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// получение числа или имя или значение в скобках
    pub(super) fn parse_primary(&mut self) -> TError<Expr> {
        match self.advance() {
            Some(TokenPosition {
                token: Token::Number(n),
                position: _,
            }) => Ok(Expr::Num(n)),
            Some(TokenPosition {
                token: Token::Ident(name),
                position: _,
            }) => {
                if matches!(self.peek().map(|t| &t.token), Some(&Token::LParen)) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek().map(|t| &t.token), Some(&Token::RParen)) {
                        args.push(self.parse_expr()?);
                        if matches!(self.peek().map(|t| &t.token), Some(&Token::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Call {
                        call: Box::new(Expr::Ident(name)),
                        args,
                    })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Some(TokenPosition {
                token: Token::LParen,
                position: _,
            }) => {
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Some(TokenPosition { token, position }) => Err(ParseError::new(
                ParseErrorKind::ExprError(ExpressionError::InvalidPrimary(token)),
                crate::type_error::Span {
                    start: position.start,
                    end: position.end,
                },
                None,
                None,
            )),
            None => Err(ParseError::new(
                ParseErrorKind::Tokens(crate::type_error::TokensError::UnexpectedEOF),
                crate::type_error::Span {
                    start: crate::type_error::Position { line: 0, column: 0 },
                    end: crate::type_error::Position { line: 0, column: 0 },
                },
                None,
                None,
            )),
        }
    }
}
