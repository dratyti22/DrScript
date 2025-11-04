use crate::lexer::Token;
use crate::parser::ParserToken;

impl ParserToken {
    /// получает текущий токен
    pub(super) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    /// получает следующий токен
    pub(super) fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    ///сдвигает позицию на 1 и возвращает предыдущий токен
    pub(super) fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).cloned()
    }

    /// проверка и потребление токена если совподают
    pub(super) fn match_token(&mut self, token: &Token) -> bool {
        if let Some(current) = self.peek()
            && std::mem::discriminant(current) == std::mem::discriminant(token)
        {
            self.advance();
            return true;
        }
        false
    }
    /// если токен не совпадает, то паника
    pub(super) fn expect(&mut self, token: &Token) {
        if !self.match_token(token) {
            panic!("Expected {:?} but got {:?}", token, self.peek())
        }
    }
}
