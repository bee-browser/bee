use crate::tables::CharClass;
use crate::tables::State;
use crate::tables::Token;

pub struct Lexer<'a> {
    src: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer {
        Lexer { src, offset: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token::Eof;
        let mut advance = 0;
        let mut state = State::default();
        let src = &self.src[self.offset..];
        let mut chars = src.char_indices();
        while let Some((pos, ch)) = chars.next() {
            let cc = match CharClass::try_from(ch) {
                Ok(cc) => cc,
                Err(_) => break,
            };
            state = state.next_state(cc);
            if let Some(candidate) = state.accept() {
                token = candidate;
                advance = pos + ch.len_utf8();
            }
        }
        if token != Token::Eof {
            self.offset += advance;
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut lexer = Lexer::new("instanceof in");
        assert_eq!(lexer.next_token(), Token::Instanceof);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::In);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("  // a b /* a b */\n  /*  \n  * a b \n  * a b */\n");
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Comment);
        assert_eq!(lexer.next_token(), Token::LineTerminator);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Comment);
        assert_eq!(lexer.next_token(), Token::LineTerminator);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_identifider() {
        let mut lexer = Lexer::new(
            "_ \
             $ \
             a \
             a1 \
             a_ \
             a$ \
             \\u0024 \
             \\u{24} \
             \\u{024} \
             \\u005f \
             \\u005F \
             \\u{5f} \
             \\u{5F} \
             \\u{05f} \
             \\u{05F} ",
        );
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::WhiteSpace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_invalid_identifier() {
        let mut lexer = Lexer::new("0Z");
        assert_eq!(lexer.next_token(), Token::NumericLiteral);
        assert_eq!(lexer.next_token(), Token::Identifier);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""""#);
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new(r#""abc""#);
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'abc'");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'\\''");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'\\n'");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'\\x00'");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'\\u0000'");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'\\u{01}'");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);

        let mut lexer = Lexer::new("'\\u{100001}'");
        assert_eq!(lexer.next_token(), Token::StringLiteral);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
