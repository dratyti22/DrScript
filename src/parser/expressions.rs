use crate::ast::Expr;
use crate::lexer::Token;
use crate::parser::Parser;

impl Parser {
    /// получение выражения за =
    pub(super) fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_less_greater();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Equal => {
                    self.advance();
                    let right = self.parse_less_greater();
                    left = Expr::EqualEqual(Box::new(left), Box::new(right));
                }
                Token::NotEqual => {
                    self.advance();
                    let right = self.parse_less_greater();
                    left = Expr::NotEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    pub(super) fn parse_less_greater(&mut self) -> Expr {
        let mut left = self.parse_plus_minus();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Less => {
                    self.advance();
                    let right = self.parse_plus_minus();
                    left = Expr::Less(Box::new(left), Box::new(right));
                }
                Token::LessEqual => {
                    self.advance();
                    let right = self.parse_plus_minus();
                    left = Expr::LessEqual(Box::new(left), Box::new(right));
                }
                Token::Greater => {
                    self.advance();
                    let right = self.parse_plus_minus();
                    left = Expr::Greater(Box::new(left), Box::new(right));
                }
                Token::GreaterEqual => {
                    self.advance();
                    let right = self.parse_plus_minus();
                    left = Expr::GreaterEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    pub(super) fn parse_plus_minus(&mut self) -> Expr {
        let mut left = self.parse_star_slash();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_star_slash();
                    left = Expr::Plus(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_star_slash();
                    left = Expr::Minus(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    /// отвечает за * и /
    pub(super) fn parse_star_slash(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Star => {
                    self.advance();
                    let right = self.parse_primary();
                    left = Expr::Star(Box::new(left), Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_primary();
                    left = Expr::Slash(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }
    /// получение числа или имя или значение в скабках
    pub(super) fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Some(Token::Number(n)) => Expr::Num(n),
            Some(Token::Ident(name)) => Expr::Ident(name),
            Some(Token::LParen) => {
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                expr
            }
            other => panic!("Ожидал число или переменную: {:?}", other),
        }
    }
}
