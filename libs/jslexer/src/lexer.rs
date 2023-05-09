use crate::tables::CharClass;
use crate::tables::State;
use crate::tables::Token;
use crate::tables::TokenKind;

pub struct Lexer<'a> {
    cursor: SourceCursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer {
        Lexer {
            cursor: SourceCursor::new(src),
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        let mut token = Token::default();
        let mut state = State::default();

        while let Some(ch) = self.cursor.get() {
            tracing::debug!(?state, ?ch);
            let cc = match CharClass::try_from(ch) {
                Ok(cc) => cc,
                Err(_) => break,
            };
            state = state.next_state(cc);
            tracing::debug!(?cc, next_state = ?state, next_state.invalid = state.is_invalid());
            if state.is_invalid() {
                break;
            }
            if state.lookahead() {
                self.cursor.lookahead();
            } else {
                self.cursor.consume();
            }
            if let Some(kind) = state.accept() {
                token.kind = kind;
                token.lexeme = self.cursor.lexeme();
                tracing::debug!(candidate = ?token);
            }
        }
        self.cursor.advance();
        tracing::debug!(?token);
        if token.kind == TokenKind::Eof && !self.cursor.eof() {
            tracing::error!(cursor.pos = self.cursor.pos, "Invalid source");
        }
        token
    }
}

struct SourceCursor<'a> {
    src: &'a str,
    chars: Vec<(usize, char)>,
    pos: usize,
    next_pos: usize,
    token_end: usize,
}

impl<'a> SourceCursor<'a> {
    fn new(src: &'a str) -> Self {
        SourceCursor {
            src,
            chars: src.char_indices().collect(),
            pos: 0,
            next_pos: 0,
            token_end: 0,
        }
    }

    #[inline(always)]
    fn get(&self) -> Option<char> {
        self.chars.get(self.next_pos).map(|(_, ch)| *ch)
    }

    #[inline(always)]
    fn lexeme(&self) -> &'a str {
        self.src.get(self.pos..self.token_end).unwrap()
    }

    #[inline(always)]
    fn consume(&mut self) {
        self.next_pos += 1;
        self.token_end = self.next_pos;
        tracing::debug!(cursor.token_end = self.token_end);
    }

    #[inline(always)]
    fn lookahead(&mut self) {
        self.next_pos += 1;
        tracing::debug!(cursor.next_pos = self.next_pos);
    }

    #[inline(always)]
    fn advance(&mut self) {
        self.pos = self.token_end;
        tracing::debug!(cursor.pos = self.pos);
    }

    #[inline(always)]
    fn eof(&self) -> bool {
        self.pos == self.chars.len()
    }
}

//<coverage:exclude>
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! token {
        ($kind:ident, $lexeme:literal) => {
            Token {
                kind: TokenKind::$kind,
                lexeme: $lexeme,
            }
        };
    }

    macro_rules! assert_token {
        ($lexer:ident, $kind:ident, $lexeme:literal) => {
            assert_eq!($lexer.next_token(), token!($kind, $lexeme));
        };
    }

    macro_rules! assert_eof {
        ($lexer:ident) => {
            assert_token!($lexer, Eof, "");
        };
    }

    #[ctor::ctor]
    fn init() {
        tracing_subscriber::fmt::init();
    }

    #[test]
    fn test() {
        let mut lexer = Lexer::new("instanceof in");
        assert_token!(lexer, Instanceof, "instanceof");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, In, "in");
        assert_eof!(lexer);
    }

    #[test]
    fn test_line_terminator_sequence() {
        let mut lexer = Lexer::new("\n");
        assert_token!(lexer, LineTerminatorSequence, "\n");
        assert_eof!(lexer);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("  // a b /* a b */\n  /*  \n  * a b \n  * a b */\n");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, Comment, "// a b /* a b */");
        assert_token!(lexer, LineTerminatorSequence, "\n");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, Comment, "/*  \n  * a b \n  * a b */");
        assert_token!(lexer, LineTerminatorSequence, "\n");
        assert_eof!(lexer);
    }

    #[test]
    fn test_identifier_name() {
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
        assert_token!(lexer, IdentifierName, "_");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "$");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "a");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "a1");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "a_");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "a$");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u0024");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u{24}");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u{024}");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u005f");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u005F");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u{5f}");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u{5F}");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u{05f}");
        assert_token!(lexer, WhiteSpace, " ");
        assert_token!(lexer, IdentifierName, "\\u{05F}");
        assert_token!(lexer, WhiteSpace, " ");
        assert_eof!(lexer);
    }

    #[test]
    fn test_invalid_identifier() {
        let mut lexer = Lexer::new("0Z");
        assert_token!(lexer, NumericLiteral, "0");
        assert_token!(lexer, IdentifierName, "Z");
        assert_eof!(lexer);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""""#);
        assert_token!(lexer, StringLiteral, r#""""#);
        assert_eof!(lexer);

        let mut lexer = Lexer::new(r#""abc""#);
        assert_token!(lexer, StringLiteral, r#""abc""#);
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'abc'");
        assert_token!(lexer, StringLiteral, "'abc'");
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'\\''");
        assert_token!(lexer, StringLiteral, "'\\''");
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'\\n'");
        assert_token!(lexer, StringLiteral, "'\\n'");
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'\\x00'");
        assert_token!(lexer, StringLiteral, "'\\x00'");
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'\\u0000'");
        assert_token!(lexer, StringLiteral, "'\\u0000'");
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'\\u{01}'");
        assert_token!(lexer, StringLiteral, "'\\u{01}'");
        assert_eof!(lexer);

        let mut lexer = Lexer::new("'\\u{100001}'");
        assert_token!(lexer, StringLiteral, "'\\u{100001}'");
        assert_eof!(lexer);
    }
}
//</coverage:exclude>
