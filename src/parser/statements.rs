use crate::ast::Stmt;
use crate::lexer::Token;
use crate::parser::Parser;

impl Parser {
    /// проверка того что приходит на вход
    pub(super) fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::If) => self.parse_if(),
            Some(Token::Var) | Some(Token::Ver) => self.parse_var_decl(),
            Some(Token::Ident(name)) if name == "print" => self.parse_print(),
            Some(Token::Ident(_)) => self.parse_assign(),

            other => panic!("Ожидалось имя переменной {:?}", other),
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
