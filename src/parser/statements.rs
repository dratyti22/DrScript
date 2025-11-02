use crate::ast::Stmt;
use crate::lexer::Token;
use crate::parser::Parser;

impl Parser {
    /// проверка того что приходит на вход
    pub(super) fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Fun) => self.parse_fun(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::Var) | Some(Token::Ver) => self.parse_var_decl(),
            Some(Token::Ident(name)) if name == "print" => self.parse_print(),
            Some(Token::Ident(_)) => {
                if self.peek_next() == Some(&Token::Assign) {
                    self.parse_assign()
                } else {
                    let expr = self.parse_expr();
                    self.expect(&Token::Semicolon);
                    Stmt::Expr(expr)
                }
            }

            other => panic!("Ожидалось имя переменной {:?}", other),
        }
    }
    /// парсинг переменной
    pub(super) fn parse_var_decl(&mut self) -> Stmt {
        let token = self.advance(); // тут получаем тип переменной
        let x = token.unwrap();
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
    /// парсинг функции
    fn parse_fun(&mut self) -> Stmt {
        self.expect(&Token::Fun);
        let name = match self.advance() {
            Some(Token::Ident(name)) => name,
            other => panic!("должно быть имя функции но полученно: {:?}", other),
        };
        self.expect(&Token::LParen);
        let mut params = Vec::new();

        if self.peek() != Some(&Token::RParen) {
            loop {
                let param = match self.advance() {
                    Some(Token::Ident(n)) => n,
                    other => panic!("Полученно: {:?} вместо имени аргумента", other),
                };
                params.push(param);
                if self.peek() == Some(&Token::Comma) {
                    self.advance();
                }
                if self.peek() == Some(&Token::RParen) {
                    break;
                }
            }
        }

        self.expect(&Token::RParen);
        self.expect(&Token::LBrace);
        let body = self.parse_block();
        self.expect(&Token::RBrace);
        Stmt::Func { name, params, body }
    }
    fn parse_return(&mut self) -> Stmt {
        self.expect(&Token::Return);
        let expr = self.parse_expr();
        self.expect(&Token::Semicolon);
        Stmt::Return(expr)
    }

    /// парсинг if
    fn parse_if(&mut self) -> Stmt {
        self.expect(&Token::If);

        let cond = self.parse_expr();
        self.expect(&Token::LBrace);
        let then_branch = self.parse_block();
        self.expect(&Token::RBrace);

        let else_branch = if self.peek() == Some(&Token::Else) {
            self.advance(); // пропускаем else
            self.expect(&Token::LBrace);
            let block = Some(self.parse_block());
            self.expect(&Token::RBrace);
            block
        } else {
            None
        };

        Stmt::If {
            cond,
            then_branch,
            else_branch,
        }
    }

    fn parse_while(&mut self) -> Stmt {
        self.expect(&Token::While);
        let cond = self.parse_expr();
        self.expect(&Token::LBrace);
        let body = self.parse_block();
        self.expect(&Token::RBrace);
        Stmt::While { cond, body }
    }
    fn parse_for(&mut self) -> Stmt {
        self.expect(&Token::For);
        self.expect(&Token::LParen);

        let init = self.parse_stmt();

        let cond = self.parse_expr();
        self.expect(&Token::Semicolon);
        let incr = if self.peek() == Some(&Token::RParen) {
            None
        } else {
            Some(self.parse_expr())
        };
        self.expect(&Token::RParen);
        self.expect(&Token::LBrace);
        let body = self.parse_block();
        self.expect(&Token::RBrace);

        Stmt::For {
            init: Box::new(init),
            cond,
            incr,
            body,
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while let Some(token) = self.peek() {
            if token == &Token::RBrace {
                break;
            }

            match token {
                Token::Ident(name) if name == "print" => {
                    stmts.push(self.parse_print());
                }
                _ => stmts.push(self.parse_stmt()),
            }
        }
        stmts
    }
}
