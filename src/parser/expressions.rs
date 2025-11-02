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
    /// отвечает за < <= >=
    fn parse_less_greater(&mut self) -> Expr {
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
    /// отвечает за + и -
    fn parse_plus_minus(&mut self) -> Expr {
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
    fn parse_star_slash(&mut self) -> Expr {
        let mut left = self.parse_unary();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::Star(Box::new(left), Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::Slash(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }
    /// парсинг значений ++ --
    fn parse_unary(&mut self) -> Expr {
        // Префиксные ++ / --
        if let Some(tok) = self.peek() {
            match tok {
                Token::PlusPlus => {
                    self.advance();
                    let expr = self.parse_primary();
                    match expr {
                        Expr::Ident(name) => return Expr::PreInc(name),
                        _ => panic!("Префикс ++ можно применять только к переменным"),
                    }
                }
                Token::MinusMinus => {
                    self.advance();
                    let expr = self.parse_primary();
                    match expr {
                        Expr::Ident(name) => return Expr::PreDec(name),
                        _ => panic!("Префикс -- можно применять только к переменным"),
                    }
                }
                _ => {}
            }
        }

        // Основное выражение
        let mut expr = self.parse_primary();

        // Постфиксные ++ / --
        loop {
            match self.peek() {
                Some(Token::PlusPlus) => {
                    self.advance();
                    match expr {
                        Expr::Ident(ref name) => expr = Expr::PostInc(name.clone()),
                        _ => panic!("Постфикс ++ можно применять только к переменным"),
                    }
                }
                Some(Token::MinusMinus) => {
                    self.advance();
                    match expr {
                        Expr::Ident(ref name) => expr = Expr::PostDec(name.clone()),
                        _ => panic!("Постфикс -- можно применять только к переменным"),
                    }
                }
                _ => break,
            }
        }

        expr
    }

    /// получение числа или имя или значение в скобках
    pub(super) fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Some(Token::Number(n)) => Expr::Num(n),
            Some(Token::Ident(name)) => {
                if self.peek() == Some(&Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while self.peek() != Some(&Token::RParen) {
                        args.push(self.parse_expr());
                        if self.peek() == Some(&Token::Comma) {
                            self.advance();
                        }else {
                            break
                        }
                    }
                    self.expect(&Token::RParen);
                    return Expr::Call{call: Box::new(Expr::Ident(name)), args};

                }else {
                    Expr::Ident(name)
                }
            },
            Some(Token::LParen) => {
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                expr
            }
            other => panic!("Ожидал число или переменную: {:?}", other),
        }
    }
}
