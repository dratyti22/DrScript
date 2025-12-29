use crate::ast::{Expr, ExprKind};
use crate::lexer::Token;
use crate::parser::ParserToken;
use crate::type_error::{ExpressionError, ParseError, ParseErrorKind, Span, TError, TokenPosition};

impl ParserToken {
    fn make_expr(&self, kind: ExprKind, span: Span) -> Expr {
        Expr { kind, span }
    }

    /// получение выражения за =
    pub(super) fn parse_expr(&mut self) -> TError<Expr> {
        let mut left = self.parse_less_greater()?;

        while let Some(TokenPosition { token, position: _ }) = self.peek().cloned() {
            match token {
                Token::Equal | Token::NotEqual => {
                    self.advance();
                    let right = self.parse_less_greater()?;
                    let span = self.merge_span(&left.span, &right.span);
                    left = self.make_expr(
                        match token {
                            Token::Equal => ExprKind::EqualEqual(Box::new(left), Box::new(right)),
                            Token::NotEqual => ExprKind::NotEqual(Box::new(left), Box::new(right)),
                            _ => unreachable!(),
                        },
                        span,
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// отвечает за < <= >=
    fn parse_less_greater(&mut self) -> TError<Expr> {
        let mut left = self.parse_plus_minus()?;

        while let Some(TokenPosition { token, position: _ }) = self.peek().cloned() {
            match token {
                Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => {
                    self.advance();
                    let right = self.parse_plus_minus()?;
                    let span = self.merge_span(&left.span, &right.span);
                    left = self.make_expr(
                        match token {
                            Token::Less => ExprKind::Less(Box::new(left), Box::new(right)),
                            Token::LessEqual => {
                                ExprKind::LessEqual(Box::new(left), Box::new(right))
                            }
                            Token::Greater => ExprKind::Greater(Box::new(left), Box::new(right)),
                            Token::GreaterEqual => {
                                ExprKind::GreaterEqual(Box::new(left), Box::new(right))
                            }
                            _ => unreachable!(),
                        },
                        span,
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// отвечает за + и -
    fn parse_plus_minus(&mut self) -> TError<Expr> {
        let mut left = self.parse_star_slash()?;

        while let Some(TokenPosition { token, position: _ }) = self.peek().cloned() {
            match token {
                Token::Plus | Token::Minus => {
                    self.advance();
                    let right = self.parse_star_slash()?;
                    let span = self.merge_span(&left.span, &right.span);
                    left = self.make_expr(
                        match token {
                            Token::Plus => ExprKind::Plus(Box::new(left), Box::new(right)),
                            Token::Minus => ExprKind::Minus(Box::new(left), Box::new(right)),
                            _ => unreachable!(),
                        },
                        span,
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }

    /// отвечает за * и /
    fn parse_star_slash(&mut self) -> TError<Expr> {
        let mut left = self.parse_unary()?;

        while let Some(TokenPosition { token, position: _ }) = self.peek().cloned() {
            match token {
                Token::Star | Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    let span = self.merge_span(&left.span, &right.span);
                    left = self.make_expr(
                        match token {
                            Token::Star => ExprKind::Star(Box::new(left), Box::new(right)),
                            Token::Slash => ExprKind::Slash(Box::new(left), Box::new(right)),
                            _ => unreachable!(),
                        },
                        span,
                    );
                }
                _ => break,
            }
        }
        Ok(left)
    }
    /// парсинг значений ++ --
    fn parse_unary(&mut self) -> TError<Expr> {
        // Префиксные ++ / --
        if let Some(TokenPosition { token, position }) = self.peek().cloned() {
            match token {
                Token::PlusPlus => {
                    self.advance();
                    let expr = self.parse_primary()?;
                    return match expr.kind {
                        ExprKind::Ident(name) => {
                            let span = self.merge_span(&position, &expr.span);
                            Ok(self.make_expr(ExprKind::PreInc(name), span))
                        }
                        _ => Err(Box::new(ParseError::new(
                            ParseErrorKind::ExprError(ExpressionError::InvalidPrefixOperator(
                                "Префикс ++ только для переменных".to_string(),
                            )),
                            Span {
                                start: position.start,
                                end: expr.span.end,
                            },
                            None,
                            None,
                        ))),
                    };
                }
                Token::MinusMinus => {
                    self.advance();
                    let expr = self.parse_primary()?;
                    return match expr.kind {
                        ExprKind::Ident(name) => {
                            let span = self.merge_span(&position, &expr.span);
                            Ok(self.make_expr(ExprKind::PreDec(name), span))
                        }
                        _ => Err(Box::new(ParseError::new(
                            ParseErrorKind::ExprError(ExpressionError::InvalidPrefixOperator(
                                "Префикс -- только для переменных".to_string(),
                            )),
                            Span {
                                start: position.start,
                                end: expr.span.end,
                            },
                            None,
                            None,
                        ))),
                    };
                }
                _ => {}
            }
        }

        // Основное выражение
        let mut expr = self.parse_primary()?;

        // Постфиксные ++ / --
        loop {
            match self.peek().cloned() {
                Some(TokenPosition {
                    token: Token::PlusPlus,
                    position,
                }) => {
                    self.advance();
                    let span = self.merge_span(&expr.span, &position);
                    match expr.kind {
                        ExprKind::Ident(ref name) => {
                            expr = self.make_expr(ExprKind::PostInc(name.clone()), span);
                        }
                        _ => {
                            return Err(Box::new(ParseError::new(
                                ParseErrorKind::ExprError(ExpressionError::InvalidPostfixOperator(
                                    "Постфикс ++ можно применять только к переменным".to_string(),
                                )),
                                Span {
                                    start: expr.span.start,
                                    end: position.end,
                                },
                                None,
                                None,
                            )));
                        }
                    }
                }
                Some(TokenPosition {
                    token: Token::MinusMinus,
                    position,
                }) => {
                    self.advance();
                    let span = self.merge_span(&expr.span, &position);
                    match expr.kind {
                        ExprKind::Ident(ref name) => {
                            expr = self.make_expr(ExprKind::PostDec(name.clone()), span);
                        }
                        _ => {
                            return Err(Box::new(ParseError::new(
                                ParseErrorKind::ExprError(ExpressionError::InvalidPostfixOperator(
                                    "Постфикс -- можно применять только к переменным".to_string(),
                                )),
                                Span {
                                    start: expr.span.start,
                                    end: position.end,
                                },
                                None,
                                None,
                            )));
                        }
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
                position,
            }) => Ok(self.make_expr(ExprKind::Num(n), position)),
            Some(TokenPosition {
                token: Token::Str(s),
                position,
            }) => Ok(self.make_expr(ExprKind::Str(s), position)),
            Some(TokenPosition {
                token: Token::Ident(name),
                position,
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
                    let rparen = self.expect(&Token::RParen)?;
                    let span = self.merge_span(&position, &rparen.position);
                    Ok(self.make_expr(
                        ExprKind::Call {
                            call: Box::new(self.make_expr(ExprKind::Ident(name), position)),
                            args,
                        },
                        span,
                    ))
                } else {
                    Ok(self.make_expr(ExprKind::Ident(name), position))
                }
            }
            Some(TokenPosition {
                token: Token::LParen,
                position,
            }) => {
                let expr = self.parse_expr()?;
                let rparen = self.expect(&Token::RParen)?;
                let span = self.merge_span(&position, &rparen.position);
                Ok(self.make_expr(expr.kind, span))
            }

            Some(TokenPosition { token, position }) => Err(Box::new(ParseError::new(
                ParseErrorKind::ExprError(ExpressionError::InvalidPrimary(token)),
                Span {
                    start: position.start,
                    end: position.end,
                },
                None,
                None,
            ))),
            None => Err(Box::new(ParseError::new(
                ParseErrorKind::Tokens(crate::type_error::TokensError::UnexpectedEOF),
                Span {
                    start: crate::type_error::Position { line: 0, column: 0 },
                    end: crate::type_error::Position { line: 0, column: 0 },
                },
                None,
                None,
            ))),
        }
    }
}
