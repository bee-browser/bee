mod cursor;
mod dfa;
mod goals;
mod tokens;

use cursor::SourceCursor;
use dfa::recognize;
pub use goals::Goal;
pub use tokens::NUM_TOKENS;
pub use tokens::Token;
pub use tokens::TokenKind;

pub struct Lexer<'a> {
    cursor: SourceCursor<'a>,
    goal: Goal,
}

impl<'a> Lexer<'a> {
    /// Creates a JavaScript lexer.
    ///
    /// `src` must contain a complete source text.
    ///
    /// The initial goal symbol of the created JavaScript lexer is [`Goal::InputElementDiv`].
    pub fn new(src: &'a str) -> Lexer {
        Lexer {
            cursor: SourceCursor::new(src),
            goal: Goal::InputElementDiv,
        }
    }

    /// Sets a goal symbol that the JavaScript lexer will recognize.
    pub fn set_goal(&mut self, goal: Goal) {
        self.goal = goal;
    }

    /// Gets a next token in the source text.
    pub fn next_token(&mut self) -> Token<'a> {
        let kind = recognize(self.goal, &mut self.cursor);
        let lexeme = match kind {
            TokenKind::Eof => "",
            _ => self.cursor.lexeme(),
        };
        let token = Token { kind, lexeme };
        tracing::debug!(?token);
        self.cursor.advance();
        if kind == TokenKind::Eof && !self.cursor.eof() {
            tracing::error!(cursor.pos = self.cursor.pos(), "Invalid source");
        }
        token
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
        assert_token!(lexer, WhiteSpaceSequence, " ");
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
        assert_token!(lexer, WhiteSpaceSequence, "  ");
        assert_token!(lexer, Comment, "// a b /* a b */");
        assert_token!(lexer, LineTerminatorSequence, "\n");
        assert_token!(lexer, WhiteSpaceSequence, "  ");
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
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "$");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "a");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "a1");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "a_");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "a$");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u0024");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u{24}");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u{024}");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u005f");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u005F");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u{5f}");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u{5F}");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u{05f}");
        assert_token!(lexer, WhiteSpaceSequence, " ");
        assert_token!(lexer, IdentifierName, "\\u{05F}");
        assert_token!(lexer, WhiteSpaceSequence, " ");
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
