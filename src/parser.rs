use crate::ast::{Expr, Stmt};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Self {
            tokens: token,
            pos: 0,
        }
    }
    /// получает текущий токен
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    ///сдвигает позицию на 1 и возвращает предыдущий токен
    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).cloned()
    }

    /// проверка и потребление токена если совподают
    fn match_token(&mut self, token: &Token) -> bool {
        if let Some(current) = self.peek() {
            if std::mem::discriminant(current) == std::mem::discriminant(token) {
                self.advance();
                return true;
            }
        }
        false
    }
    /// если токен не совпадает, то паника
    fn expect(&mut self, token: &Token) {
        if !self.match_token(token) {
            panic!("Expected {:?} but got {:?}", token, self.peek())
        }
    }
    /// парсинг значений
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt())
        }
        stmts
    }
    /// проверка того что приходит на вход
    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::Var) | Some(Token::Ver) => self.parse_var_decl(),
            Some(Token::Ident(name)) if name == "print" => self.parse_print(),
            Some(Token::Ident(_)) => self.parse_assign(),

            _ => panic!("Ожидал ось имя переменной"),
        }
    }
    /// парсинг переменной
    fn parse_var_decl(&mut self) -> Stmt {
        let mutable = self.advance(); // тут получаем тип переменной
        let x = mutable.unwrap();
        match x {
            Token::Var => {
                let name = match self.advance() {
                    Some(Token::Ident(name)) => name, // тут получаем имя переменной
                    _ => panic!("Ожидал имя переменной"),
                };
                self.expect(&Token::Assign); // тут проверяем что после имени идет =
                let value = self.parse_expr(); // тут парсим выражение
                self.expect(&Token::Semicolon); // тут проверяем что после выражения идет ;
                Stmt::Assign { name, value }
            }
            Token::Ver => {
                let name = match self.advance() {
                    Some(Token::Ident(name)) => name, // тут получаем имя переменной
                    _ => panic!("Ожидал имя переменной"),
                };
                self.expect(&Token::Assign); // тут проверяем что после имени идет =
                let value = self.parse_expr(); // тут парсим выражение
                self.expect(&Token::Semicolon); // тут проверяем что после выражения идет ;
                Stmt::VerDecl {
                    name,
                    value,
                    mutable: true,
                }
            }
            _ => panic!("Ожидал переменную"),
        }
    }
    /// выводит значение
    fn parse_print(&mut self) -> Stmt {
        self.advance(); // пропускаем print
        self.expect(&Token::LParen);
        let expr = self.parse_expr();
        self.expect(&Token::RParen);
        self.expect(&Token::Semicolon);
        Stmt::Print(expr)
    }
    /// получение выражения за =
    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_tern();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_tern();
                    left = Expr::Plus(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_tern();
                    left = Expr::Minus(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }
    /// по логике изменение значения переменной
    fn parse_assign(&mut self) -> Stmt {
        let name = match self.advance() {
            Some(Token::Ident(name)) => name,
            _ => panic!("Ожидал имя переменной"),
        };
        self.expect(&Token::Assign);
        let value = self.parse_expr();
        self.expect(&Token::Semicolon);
        Stmt::Assign { name, value }
    }
    /// отвечает за * и /
    fn parse_tern(&mut self) -> Expr {
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
    fn parse_primary(&mut self) -> Expr {
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
