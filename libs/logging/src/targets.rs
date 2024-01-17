// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/logging/src/targets.rs.hbs

use super::Target;

/// The target ID for `bee::logview`.
pub const LOGVIEW: Target = Target(0);

/// The target ID for `bee::dfagen`.
pub const DFAGEN: Target = Target(1);

/// The target ID for `bee::lalrgen`.
pub const LALRGEN: Target = Target(2);

/// The target ID for `bee::estree`.
pub const ESTREE: Target = Target(3);

/// The target ID for `bee::jsparser::lexer`.
pub const JSPARSER_LEXER: Target = Target(4);

/// The target ID for `bee::jsparser::parser`.
pub const JSPARSER_PARSER: Target = Target(5);

/// The target ID for `bee::htmlparser`.
pub const HTMLPARSER: Target = Target(6);

/// The target ID for `bee::toydom`.
pub const TOYDOM: Target = Target(7);

/// The target ID for `bee::htmltokenizer`.
pub const HTMLTOKENIZER: Target = Target(8);

/// The target ID for `bee::layout`.
pub const LAYOUT: Target = Target(9);

/// The target ID for `bee::tests`.
pub const TESTS: Target = Target(10);

// Use `const fn len()` instead of `const LEN` in order to avoid conflicts between target symbols.
// The `len` package might be created in the future.
#[inline(always)]
pub const fn len() -> usize {
    11
}

#[inline(always)]
pub const fn name(id: usize) -> &'static str {
    const NAMES: [&str; 11] = [
        "bee::logview",
        "bee::dfagen",
        "bee::lalrgen",
        "bee::estree",
        "bee::jsparser::lexer",
        "bee::jsparser::parser",
        "bee::htmlparser",
        "bee::toydom",
        "bee::htmltokenizer",
        "bee::layout",
        "bee::tests",
    ];
    NAMES[id]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logview() {
        const NAME: &str = LOGVIEW.name();
        assert_eq!(NAME, "bee::logview");
    }

    #[test]
    fn test_dfagen() {
        const NAME: &str = DFAGEN.name();
        assert_eq!(NAME, "bee::dfagen");
    }

    #[test]
    fn test_lalrgen() {
        const NAME: &str = LALRGEN.name();
        assert_eq!(NAME, "bee::lalrgen");
    }

    #[test]
    fn test_estree() {
        const NAME: &str = ESTREE.name();
        assert_eq!(NAME, "bee::estree");
    }

    #[test]
    fn test_jsparser_lexer() {
        const NAME: &str = JSPARSER_LEXER.name();
        assert_eq!(NAME, "bee::jsparser::lexer");
    }

    #[test]
    fn test_jsparser_parser() {
        const NAME: &str = JSPARSER_PARSER.name();
        assert_eq!(NAME, "bee::jsparser::parser");
    }

    #[test]
    fn test_htmlparser() {
        const NAME: &str = HTMLPARSER.name();
        assert_eq!(NAME, "bee::htmlparser");
    }

    #[test]
    fn test_toydom() {
        const NAME: &str = TOYDOM.name();
        assert_eq!(NAME, "bee::toydom");
    }

    #[test]
    fn test_htmltokenizer() {
        const NAME: &str = HTMLTOKENIZER.name();
        assert_eq!(NAME, "bee::htmltokenizer");
    }

    #[test]
    fn test_layout() {
        const NAME: &str = LAYOUT.name();
        assert_eq!(NAME, "bee::layout");
    }

    #[test]
    fn test_tests() {
        const NAME: &str = TESTS.name();
        assert_eq!(NAME, "bee::tests");
    }
}
