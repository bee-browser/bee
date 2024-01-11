mod cursor;
mod dfa;
mod goals;
mod logger;
mod tokens;

use bitflags::bitflags;

use super::Error;
use cursor::SourceCursor;
use dfa::recognize;
pub use goals::Goal;
pub use tokens::TokenKind;

type LexerResult<'a> = Result<Token<'a>, Error>;

pub struct Lexer<'a> {
    cursor: SourceCursor<'a>,
    goal: Goal,
    location: Location,
}

impl<'a> Lexer<'a> {
    /// Creates a JavaScript lexer.
    ///
    /// `src` must contain a complete source text.
    ///
    /// The initial goal symbol of the created JavaScript lexer is [`Goal::InputElementDiv`].
    #[inline(always)]
    pub fn new(src: &'a str) -> Lexer {
        Lexer {
            cursor: SourceCursor::new(src),
            goal: Goal::InputElementDiv,
            location: Default::default(),
        }
    }

    /// Sets a goal symbol that the JavaScript lexer will recognize.
    #[inline(always)]
    pub fn set_goal(&mut self, goal: Goal) {
        logger::trace!(opcode = "set_goal", ?goal);
        self.goal = goal;
    }

    /// Gets a next token in the source text.
    #[inline(always)]
    pub fn next_token(&self) -> LexerResult<'a> {
        recognize(self.goal, &self.cursor)
    }

    /// Consumes the token and advances the cursor.
    #[inline(always)]
    pub fn consume_token(&mut self, token: Token<'a>) {
        match token.kind {
            TokenKind::Eof => {
                if !self.cursor.eof() {
                    logger::error!("Invalid source");
                }
            }
            _ => {
                self.cursor.advance(token.lexeme.len());
                if cfg!(feature = "location") {
                    self.location = token.compute_end(&self.location);
                }
            }
        }
    }

    #[inline(always)]
    pub fn src(&self) -> &'a str {
        self.cursor.src()
    }

    #[inline(always)]
    pub fn pos(&self) -> usize {
        self.cursor.pos()
    }

    #[inline(always)]
    pub fn location(&self) -> &Location {
        &self.location
    }
}

/// A token in a source text.
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    /// The lexeme of the token.
    pub lexeme: &'a str,

    /// The kind of the token.
    pub kind: TokenKind,

    /// Flags used internally.
    flags: TokenFlags,
}

impl<'a> Token<'a> {
    /// A token used for automatic semicolon insertion.
    pub(crate) const AUTO_SEMICOLON: Token<'static> = Token {
        lexeme: ";",
        kind: TokenKind::Semicolon,
        flags: TokenFlags::AUTO_INSERTION,
    };

    pub(crate) const SINGLE_LINE_TERMINATOR: Token<'static> = Token {
        lexeme: "\n",
        kind: TokenKind::LineTerminatorSequence,
        flags: TokenFlags::HAS_LINE_TERMINATORS,
    };

    /// Returns `true` if the token was inserted automatically.
    pub fn inserted_automatically(&self) -> bool {
        self.flags.contains(TokenFlags::AUTO_INSERTION)
    }

    /// Returns `true` if the lexeme contains line terminators.
    pub fn has_line_terminators(&self) -> bool {
        self.flags.contains(TokenFlags::HAS_LINE_TERMINATORS)
    }

    /// Returns the number of UTF-16 code units in the lexeme.
    pub fn len_utf16(&self) -> usize {
        self.lexeme.chars().fold(0, |n, ch| n + ch.len_utf16())
    }

    /// Computes the end location of the lexeme in the source text.
    pub fn compute_end(&self, start: &Location) -> Location {
        if self.inserted_automatically() {
            return start.clone();
        }
        let mut cr = false;
        self.lexeme.chars().fold(start.clone(), |mut loc, ch| {
            match ch {
                '\u{000A}' => {
                    if cr {
                        loc.crlf(1);
                    } else {
                        loc.eol(1);
                    }
                    cr = false;
                }
                '\u{000D}' => {
                    loc.eol(1);
                    cr = true;
                }
                '\u{2028}' | '\u{2029}' => {
                    loc.eol(1);
                    cr = false;
                }
                _ => {
                    loc.advance(ch.len_utf16());
                    cr = false;
                }
            }
            loc
        })
    }
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Token {
            lexeme: "",
            kind: TokenKind::Eof,
            flags: TokenFlags::empty(),
        }
    }
}

bitflags! {
    #[derive(Clone, Debug, PartialEq)]
    struct TokenFlags: u8 {
        const AUTO_INSERTION       = 0b00000001;
        const HAS_LINE_TERMINATORS = 0b00000010;
    }
}

/// A type representing a source location.
#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    /// 0-based offset in the source text.
    pub offset: usize,

    /// 1-based number of the line.
    pub line: usize,

    /// 0-based offset in the line.
    pub column: usize,
}

impl Location {
    /// Returns the source location of the origin.
    pub fn origin() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn forward(&self, n: usize) -> Self {
        debug_assert!(self.offset <= usize::MAX - n);
        debug_assert!(self.column <= usize::MAX - n);
        Self {
            offset: self.offset + n,
            line: self.line,
            column: self.column + n,
        }
    }

    pub fn backward(&self, n: usize) -> Self {
        debug_assert!(self.offset >= n);
        debug_assert!(self.column >= n);
        Self {
            offset: self.offset - n,
            line: self.line,
            column: self.column - n,
        }
    }

    fn advance(&mut self, n: usize) {
        self.offset += n;
        self.column += n;
    }

    fn eol(&mut self, n: usize) {
        self.offset += n;
        self.line += 1;
        self.column = 0;
    }

    fn crlf(&mut self, n: usize) {
        self.offset += n;
        debug_assert_eq!(self.column, 0);
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::origin()
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}:{}", self.offset, self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    logging::init!();

    macro_rules! loc {
        ($offset:expr, $line:expr, $column:expr) => {
            if cfg!(feature = "location") {
                Location {
                    offset: $offset,
                    line: $line,
                    column: $column,
                }
            } else {
                Location::origin()
            }
        };
    }

    macro_rules! token {
        ($kind:ident, $lexeme:literal) => {
            Token {
                lexeme: $lexeme,
                kind: TokenKind::$kind,
                flags: if $lexeme.contains(&['\u{000A}', '\u{000D}', '\u{2028}', '\u{2029}']) {
                    TokenFlags::HAS_LINE_TERMINATORS
                } else {
                    TokenFlags::empty()
                },
            }
        };
    }

    macro_rules! assert_token {
        ($lexer:ident, $kind:ident, $lexeme:literal, $start:expr, $end:expr) => {
            let token = $lexer.next_token().unwrap();
            assert_eq!(token, token!($kind, $lexeme));
            let start = $lexer.location();
            assert_eq!(*start, $start);
            let end = token.compute_end(start);
            assert_eq!(end, $end);
            $lexer.consume_token(token);
        };
    }

    macro_rules! assert_eof {
        ($lexer:ident, $start:expr, $end:expr) => {
            assert_token!($lexer, Eof, "", $start, $end);
        };
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("instanceof in");
        assert_token!(
            lexer,
            Instanceof,
            "instanceof",
            loc!(0, 1, 0),
            loc!(10, 1, 10)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(10, 1, 10),
            loc!(11, 1, 11)
        );
        assert_token!(lexer, In, "in", loc!(11, 1, 11), loc!(13, 1, 13));
        assert_eof!(lexer, loc!(13, 1, 13), loc!(13, 1, 13));
    }

    #[test]
    fn test_line_terminator_sequence_lf() {
        let mut lexer = Lexer::new("\n");
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\n",
            loc!(0, 1, 0),
            loc!(1, 2, 0)
        );
        assert_eof!(lexer, loc!(1, 2, 0), loc!(1, 2, 0));
    }

    #[test]
    fn test_line_terminator_sequence_cr() {
        let mut lexer = Lexer::new("\r");
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\r",
            loc!(0, 1, 0),
            loc!(1, 2, 0)
        );
        assert_eof!(lexer, loc!(1, 2, 0), loc!(1, 2, 0));
    }

    #[test]
    fn test_line_terminator_sequence_ls() {
        let mut lexer = Lexer::new("\u{2028}");
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\u{2028}",
            loc!(0, 1, 0),
            loc!(1, 2, 0)
        );
        assert_eof!(lexer, loc!(1, 2, 0), loc!(1, 2, 0));
    }

    #[test]
    fn test_line_terminator_sequence_ps() {
        let mut lexer = Lexer::new("\u{2029}");
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\u{2029}",
            loc!(0, 1, 0),
            loc!(1, 2, 0)
        );
        assert_eof!(lexer, loc!(1, 2, 0), loc!(1, 2, 0));
    }

    #[test]
    fn test_line_terminator_sequence_cr_lf() {
        let mut lexer = Lexer::new("\r\n");
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\r\n",
            loc!(0, 1, 0),
            loc!(2, 2, 0)
        );
        assert_eof!(lexer, loc!(2, 2, 0), loc!(2, 2, 0));
    }

    #[test]
    fn test_line_terminator_sequence_cr_cr() {
        let mut lexer = Lexer::new("\r\r");
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\r",
            loc!(0, 1, 0),
            loc!(1, 2, 0)
        );
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\r",
            loc!(1, 2, 0),
            loc!(2, 3, 0)
        );
        assert_eof!(lexer, loc!(2, 3, 0), loc!(2, 3, 0));
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("  // a b /* a b */\n  /*  \n  * a b \n  * a b */\n");
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            "  ",
            loc!(0, 1, 0),
            loc!(2, 1, 2)
        );
        assert_token!(
            lexer,
            Comment,
            "// a b /* a b */",
            loc!(2, 1, 2),
            loc!(18, 1, 18)
        );
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\n",
            loc!(18, 1, 18),
            loc!(19, 2, 0)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            "  ",
            loc!(19, 2, 0),
            loc!(21, 2, 2)
        );
        assert_token!(
            lexer,
            Comment,
            "/*  \n  * a b \n  * a b */",
            loc!(21, 2, 2),
            loc!(45, 4, 10)
        );
        assert_token!(
            lexer,
            LineTerminatorSequence,
            "\n",
            loc!(45, 4, 10),
            loc!(46, 5, 0)
        );
        assert_eof!(lexer, loc!(46, 5, 0), loc!(46, 5, 0));
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
        assert_token!(lexer, IdentifierName, "_", loc!(0, 1, 0), loc!(1, 1, 1));
        assert_token!(lexer, WhiteSpaceSequence, " ", loc!(1, 1, 1), loc!(2, 1, 2));
        assert_token!(lexer, IdentifierName, "$", loc!(2, 1, 2), loc!(3, 1, 3));
        assert_token!(lexer, WhiteSpaceSequence, " ", loc!(3, 1, 3), loc!(4, 1, 4));
        assert_token!(lexer, IdentifierName, "a", loc!(4, 1, 4), loc!(5, 1, 5));
        assert_token!(lexer, WhiteSpaceSequence, " ", loc!(5, 1, 5), loc!(6, 1, 6));
        assert_token!(lexer, IdentifierName, "a1", loc!(6, 1, 6), loc!(8, 1, 8));
        assert_token!(lexer, WhiteSpaceSequence, " ", loc!(8, 1, 8), loc!(9, 1, 9));
        assert_token!(lexer, IdentifierName, "a_", loc!(9, 1, 9), loc!(11, 1, 11));
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(11, 1, 11),
            loc!(12, 1, 12)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "a$",
            loc!(12, 1, 12),
            loc!(14, 1, 14)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(14, 1, 14),
            loc!(15, 1, 15)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u0024",
            loc!(15, 1, 15),
            loc!(21, 1, 21)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(21, 1, 21),
            loc!(22, 1, 22)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u{24}",
            loc!(22, 1, 22),
            loc!(28, 1, 28)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(28, 1, 28),
            loc!(29, 1, 29)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u{024}",
            loc!(29, 1, 29),
            loc!(36, 1, 36)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(36, 1, 36),
            loc!(37, 1, 37)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u005f",
            loc!(37, 1, 37),
            loc!(43, 1, 43)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(43, 1, 43),
            loc!(44, 1, 44)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u005F",
            loc!(44, 1, 44),
            loc!(50, 1, 50)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(50, 1, 50),
            loc!(51, 1, 51)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u{5f}",
            loc!(51, 1, 51),
            loc!(57, 1, 57)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(57, 1, 57),
            loc!(58, 1, 58)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u{5F}",
            loc!(58, 1, 58),
            loc!(64, 1, 64)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(64, 1, 64),
            loc!(65, 1, 65)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u{05f}",
            loc!(65, 1, 65),
            loc!(72, 1, 72)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(72, 1, 72),
            loc!(73, 1, 73)
        );
        assert_token!(
            lexer,
            IdentifierName,
            "\\u{05F}",
            loc!(73, 1, 73),
            loc!(80, 1, 80)
        );
        assert_token!(
            lexer,
            WhiteSpaceSequence,
            " ",
            loc!(80, 1, 80),
            loc!(81, 1, 81)
        );
        assert_eof!(lexer, loc!(81, 1, 81), loc!(81, 1, 81));
    }

    #[test]
    fn test_invalid_identifier() {
        let mut lexer = Lexer::new("0Z");
        assert_token!(lexer, NumericLiteral, "0", loc!(0, 1, 0), loc!(1, 1, 1));
        assert_token!(lexer, IdentifierName, "Z", loc!(1, 1, 1), loc!(2, 1, 2));
        assert_eof!(lexer, loc!(2, 1, 2), loc!(2, 1, 2));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""""#);
        assert_token!(lexer, StringLiteral, r#""""#, loc!(0, 1, 0), loc!(2, 1, 2));
        assert_eof!(lexer, loc!(2, 1, 2), loc!(2, 1, 2));

        let mut lexer = Lexer::new(r#""abc""#);
        assert_token!(
            lexer,
            StringLiteral,
            r#""abc""#,
            loc!(0, 1, 0),
            loc!(5, 1, 5)
        );
        assert_eof!(lexer, loc!(5, 1, 5), loc!(5, 1, 5));

        let mut lexer = Lexer::new("'abc'");
        assert_token!(lexer, StringLiteral, "'abc'", loc!(0, 1, 0), loc!(5, 1, 5));
        assert_eof!(lexer, loc!(5, 1, 5), loc!(5, 1, 5));

        let mut lexer = Lexer::new("'\\''");
        assert_token!(lexer, StringLiteral, "'\\''", loc!(0, 1, 0), loc!(4, 1, 4));
        assert_eof!(lexer, loc!(4, 1, 4), loc!(4, 1, 4));

        let mut lexer = Lexer::new("'\\n'");
        assert_token!(lexer, StringLiteral, "'\\n'", loc!(0, 1, 0), loc!(4, 1, 4));
        assert_eof!(lexer, loc!(4, 1, 4), loc!(4, 1, 4));

        let mut lexer = Lexer::new("'\\x00'");
        assert_token!(
            lexer,
            StringLiteral,
            "'\\x00'",
            loc!(0, 1, 0),
            loc!(6, 1, 6)
        );
        assert_eof!(lexer, loc!(6, 1, 6), loc!(6, 1, 6));

        let mut lexer = Lexer::new("'\\u0000'");
        assert_token!(
            lexer,
            StringLiteral,
            "'\\u0000'",
            loc!(0, 1, 0),
            loc!(8, 1, 8)
        );
        assert_eof!(lexer, loc!(8, 1, 8), loc!(8, 1, 8));

        let mut lexer = Lexer::new("'\\u{01}'");
        assert_token!(
            lexer,
            StringLiteral,
            "'\\u{01}'",
            loc!(0, 1, 0),
            loc!(8, 1, 8)
        );
        assert_eof!(lexer, loc!(8, 1, 8), loc!(8, 1, 8));

        let mut lexer = Lexer::new("'\\u{100001}'");
        assert_token!(
            lexer,
            StringLiteral,
            "'\\u{100001}'",
            loc!(0, 1, 0),
            loc!(12, 1, 12)
        );
        assert_eof!(lexer, loc!(12, 1, 12), loc!(12, 1, 12));
    }

    #[test]
    fn test_conditional_expression() {
        // '?' must be recognized as a token.
        let mut lexer = Lexer::new("x?.5:0");
        assert_token!(lexer, IdentifierName, "x", loc!(0, 1, 0), loc!(1, 1, 1));
        assert_token!(lexer, Conditional, "?", loc!(1, 1, 1), loc!(2, 1, 2));
        assert_token!(lexer, NumericLiteral, ".5", loc!(2, 1, 2), loc!(4, 1, 4));
        assert_token!(lexer, Colon, ":", loc!(4, 1, 4), loc!(5, 1, 5));
        assert_token!(lexer, NumericLiteral, "0", loc!(5, 1, 5), loc!(6, 1, 6));
        assert_eof!(lexer, loc!(6, 1, 6), loc!(6, 1, 6));
    }

    #[test]
    fn test_template_tail() {
        let mut lexer = Lexer::new("}$`1");
        lexer.set_goal(Goal::InputElementTemplateTail);
        assert_token!(lexer, TemplateTail, "}$`", loc!(0, 1, 0), loc!(3, 1, 3));
    }

    #[test]
    fn test_id_start() {
        let mut lexer = Lexer::new("あa");
        assert_token!(lexer, IdentifierName, "あa", loc!(0, 1, 0), loc!(2, 1, 2));
        assert_eof!(lexer, loc!(2, 1, 2), loc!(2, 1, 2));
    }

    #[test]
    fn test_id_continue() {
        let mut lexer = Lexer::new("aあ");
        assert_token!(lexer, IdentifierName, "aあ", loc!(0, 1, 0), loc!(2, 1, 2));
        assert_eof!(lexer, loc!(2, 1, 2), loc!(2, 1, 2));
    }
}
