mod lexer;
mod parser;

/// A JavaScript parser.
pub use parser::Parser;

/// A trait to handle LALR parsing actions.
pub use parser::SyntaxHandler;

/// A token recognized by a JavaScript lexer.
pub use lexer::Token;

/// Token types.
pub use lexer::TokenKind;

pub use lexer::Location;

/// A production rule in the ECMA-262 syntactic grammar.
pub use parser::ProductionRule;

/// Errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected character")]
    UnexpectedCharacter,
    #[error("Syntax error")]
    SyntaxError,
}

/// Converts a template literal content into a raw string.
pub fn template_literal_to_raw_string(literal: &str) -> String {
    // TODO: 13.2.8.3 Static Semantics: TemplateString ( templateToken, raw )
    literal.replace("\r\n", "\n").replace('\r', "\n")
}

/// Converts a template literal content into a cooked string.
pub fn template_literal_to_cooked_string(literal: &str) -> Option<String> {
    // TODO: 13.2.8.3 Static Semantics: TemplateString ( templateToken, raw )
    let s = literal.replace("\r\n", "\n").replace('\r', "\n");
    literal_content_to_string(&s)
}

/// Converts a string literal into a string.
pub fn string_literal_to_string(literal: &str) -> String {
    literal_content_to_string(&literal[1..(literal.len() - 1)]).unwrap()
}

/// Converts a literal content into a string.
pub fn literal_content_to_string(content: &str) -> Option<String> {
    // TODO: improve performance

    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut high_surrogate = None;

    #[inline(always)]
    fn put(c: char, result: &mut String, high_surrogate: &mut Option<u32>) {
        if high_surrogate.take().is_some() {
            result.push('\u{FFFD}');
        }
        result.push(c);
    }

    while let Some(c) = chars.next() {
        if c != '\\' {
            put(c, &mut result, &mut high_surrogate);
            continue;
        }

        // escape sequence
        match chars.next().unwrap() {
            '0' => put('\u{0000}', &mut result, &mut high_surrogate),
            'b' => put('\u{0008}', &mut result, &mut high_surrogate),
            't' => put('\u{0009}', &mut result, &mut high_surrogate),
            'n' => put('\u{000A}', &mut result, &mut high_surrogate),
            'v' => put('\u{000B}', &mut result, &mut high_surrogate),
            'f' => put('\u{000C}', &mut result, &mut high_surrogate),
            'r' => put('\u{000D}', &mut result, &mut high_surrogate),
            '\u{000A}' | '\u{2028}' | '\u{2029}' => (),
            '\u{000D}' => {
                if let Some('\u{000A}') = chars.peek() {
                    chars.next();
                }
            }
            'x' if chars.peek().is_some() => {
                let hi = chars.next()?.to_digit(16)?;
                let lo = chars.next()?.to_digit(16)?;
                let c = char::from_u32((hi << 4) + lo).unwrap();
                put(c, &mut result, &mut high_surrogate);
            }
            'u' if chars.peek().is_some() => {
                if let Some('{') = chars.peek() {
                    let mut n = 0;
                    chars.next();
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(c) = chars.next() {
                        if c == '}' {
                            break;
                        }
                        // FIXME: `n << 4` causes wrong indentation in emacs...
                        use std::ops::Shl;
                        n = n.shl(4) + c.to_digit(16)?;
                    }
                    let c = char::from_u32(n)?;
                    put(c, &mut result, &mut high_surrogate);
                } else {
                    let d0 = chars.next()?.to_digit(16)?;
                    let d1 = chars.next()?.to_digit(16)?;
                    let d2 = chars.next()?.to_digit(16)?;
                    let d3 = chars.next()?.to_digit(16)?;
                    let cp = (d0 << 12) + (d1 << 8) + (d2 << 4) + d3;
                    match char::from_u32(cp) {
                        Some(c) => put(c, &mut result, &mut high_surrogate),
                        None => {
                            if let Some(high_surrogate) = high_surrogate.take() {
                                if high_surrogate < 0xD800 || cp < 0xDC00 {
                                    result.push('\u{FFFD}');
                                } else {
                                    let high = high_surrogate - 0xD800;
                                    let low = cp - 0xDC00;
                                    if high > 0x03FF || low > 0x03FF {
                                        result.push('\u{FFFD}');
                                    } else {
                                        result.push(char::from_u32((high << 10 | low) + 0x10000)?);
                                    }
                                }
                            } else {
                                high_surrogate = Some(cp)
                            }
                        }
                    }
                }
            }
            c => put(c, &mut result, &mut high_surrogate),
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_literal_content_to_string() {
        // surrogate pair
        assert_matches!(literal_content_to_string("\\ud83d\\udcf7"), Some(s) => {
            assert_eq!(s, "\u{1F4F7}");
        });

        // invalid escape
        assert_matches!(literal_content_to_string("\\uc1"), None);
    }
}
