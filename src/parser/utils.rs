use crate::lexer::Token;
use crate::parser::ParserToken;
use crate::type_error::{TokenPosition, TError, ParseError, ParseErrorKind, TokensError, Span};

impl ParserToken {
    /// получает текущий токен
    pub(super) fn peek(&self) -> Option<&TokenPosition> {
        self.tokens.get(self.pos)
    }
    /// получает следующий токен
    pub(super) fn peek_next(&self) -> Option<&TokenPosition> {
        self.tokens.get(self.pos + 1)
    }

    ///сдвигает позицию на 1 и возвращает предыдущий токен
    pub(super) fn advance(&mut self) -> Option<TokenPosition> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).cloned()
    }

    /// проверка и потребление токена если совпадают
    pub(super) fn match_token(&mut self, token_t: &Token) -> bool {
        if let Some(TokenPosition {
            token: token_p,
            position: _,
        }) = self.peek()
            && std::mem::discriminant(token_p) == std::mem::discriminant(token_t)
        {
            self.advance();
            return true;
        }
        false
    }
    /// если токен не совпадает, то ошибка
    pub(super) fn expect(&mut self, expected_token: &Token) -> TError<()> {
        if !self.match_token(expected_token) {
            let current = self.peek();
            match current {
                Some(TokenPosition { token, position }) => Err(ParseError::new(
                    ParseErrorKind::Tokens(TokensError::ExpectedToken(expected_token.clone(), token.clone())),
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
        } else {
            Ok(())
        }
    }
    
    /// вспомогательная функция для обратной совместимости
    pub(super) fn expect_or_panic(&mut self, expected_token: &Token) {
        if let Err(_) = self.expect(expected_token) {
            panic!("Expected {:?} but got {:?}", expected_token, self.peek())
        }
    }
}
