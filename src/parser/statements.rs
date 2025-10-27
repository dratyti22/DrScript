use crate::ast::Stmt;
use crate::lexer::Token;
use crate::parser::Parser;

impl Parser {
    /// проверка того что приходит на вход
    pub(super) fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::Var) | Some(Token::Ver) => self.parse_var_decl(),
            Some(Token::Ident(name)) if name == "print" => self.parse_print(),
            Some(Token::Ident(_)) => self.parse_assign(),

            _ => panic!("Ожидал ось имя переменной"),
        }
    }
    /// парсинг переменной
    pub(super) fn parse_var_decl(&mut self) -> Stmt {
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
    pub(super) fn parse_print(&mut self) -> Stmt {
        self.advance(); // пропускаем print
        self.expect(&Token::LParen);
        let expr = self.parse_expr();
        self.expect(&Token::RParen);
        self.expect(&Token::Semicolon);
        Stmt::Print(expr)
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
}
