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

/// Converts a string literal into a string.
pub fn string_literal_to_string(literal: &str) -> String {
    literal_content_to_string(&literal[1..(literal.len() - 1)])
}

/// Converts a literal content into a string.
pub fn literal_content_to_string(content: &str) -> String {
    // TODO: improve performance
    // TODO: a surrogate pair like "\uD87E\uDC04"

    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }

        // escape sequence
        match chars.next().unwrap() {
            '0' => result.push('\u{0000}'),
            'b' => result.push('\u{0008}'),
            't' => result.push('\u{0009}'),
            'n' => result.push('\u{000A}'),
            'v' => result.push('\u{000B}'),
            'f' => result.push('\u{000C}'),
            'r' => result.push('\u{000D}'),
            '\u{000A}' | '\u{2028}' | '\u{2029}' => (),
            '\u{000D}' => {
                if let Some('\u{000A}') = chars.peek() {
                    chars.next();
                }
            }
            'x' if chars.peek().is_some() => {
                let hi = chars.next().unwrap().to_digit(16).unwrap();
                let lo = chars.next().unwrap().to_digit(16).unwrap();
                let c = char::from_u32((hi << 4) + lo).unwrap();
                result.push(c);
            }
            'u' if chars.peek().is_some() => {
                if let Some('{') = chars.peek() {
                    let mut n = 0;
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '}' {
                            break;
                        }
                        // FIXME: `n << 4` causes wrong indentation in emacs...
                        use std::ops::Shl;
                        n = n.shl(4) + c.to_digit(16).unwrap();
                    }
                    let c = char::from_u32(n).unwrap();
                    result.push(c);
                } else {
                    let d0 = chars.next().unwrap().to_digit(16).unwrap();
                    let d1 = chars.next().unwrap().to_digit(16).unwrap();
                    let d2 = chars.next().unwrap().to_digit(16).unwrap();
                    let d3 = chars.next().unwrap().to_digit(16).unwrap();
                    let c = char::from_u32((d0 << 12) + (d1 << 8) + (d2 << 4) + d3).unwrap();
                    result.push(c);
                }
            }
            c => result.push(c),
        }
    }
    result
}
