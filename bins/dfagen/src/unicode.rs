use std::ops::Range;
use std::ops::RangeInclusive;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use smallvec::smallvec;
use smallvec::SmallVec;

// macros

macro_rules! code_point {
    ($value:expr) => {
        CodePoint::from($value)
    };
}

macro_rules! unicode_span {
    ($value:expr) => {
        crate::unicode::UnicodeSpan::from($value)
    };
    ($first:expr, $last:expr) => {
        crate::unicode::UnicodeSpan::new($first.into(), $last.into())
    };
}

macro_rules! unicode_set {
    () => {
        crate::unicode::UnicodeSet::empty()
    };
    ($($spans:expr,)+) => {
        unicode_set![$($spans),+]
    };
    ($($spans:expr),+) => {
        crate::unicode::UnicodeSet::from(vec![$(crate::unicode::unicode_span!($spans).into()),+])
    };
}

pub(crate) use unicode_set;
pub(crate) use unicode_span;

/// Represents a single Unicode code point.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct CodePoint(u32);

impl CodePoint {
    pub fn get_char(&self) -> char {
        unsafe { char::from_u32_unchecked(self.0) }
    }
}

impl From<char> for CodePoint {
    fn from(value: char) -> Self {
        (value as u32).into()
    }
}

impl From<u32> for CodePoint {
    fn from(value: u32) -> Self {
        CodePoint(value)
    }
}

impl std::fmt::Display for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static ASCII_TABLE: [&str; 0x21] = [
            "<NUL>", "<SOH>", "<STX>", "<ETX>", "<EOT>", "<ENQ>", "<ACK>", "<BEL>", "<BS>", "<HT>",
            "<LF>", "<VT>", "<FF>", "<CR>", "<SO>", "<SI>", "<DLE>", "<DC1>", "<DC2>", "<DC3>",
            "<DC4>", "<NAK>", "<SYN>", "<ETB>", "<CAN>", "<EM>", "<SUB>", "<ESC>", "<FS>", "<GS>",
            "<RS>", "<US>", "<SP>",
        ];
        if self.0 <= 0x20 {
            write!(f, "{}", ASCII_TABLE[self.0 as usize])
        } else if self.0 < 0x7F {
            write!(f, "{}", self.get_char())
        } else if self.0 == 0x7F {
            write!(f, "<DEL>")
        } else if self.0 < 0x10000 {
            write!(f, "U+{:04X}", self.0)
        } else {
            write!(f, "U+{:06X}", self.0)
        }
    }
}

/// Represents a continuous range in Unicode code points.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct UnicodeSpan {
    base: u32,
    length: u32,
}

impl UnicodeSpan {
    /// An empty Unicode span.
    pub const EMPTY: Self = Self { base: 0, length: 0 };

    /// A Unicode span containing all characters.
    pub const ANY: Self = Self {
        base: 0,
        length: char::MAX as u32 + 1,
    };

    /// Create a Unicode span with a particular range.
    pub const fn new(first: CodePoint, last: CodePoint) -> Self {
        debug_assert!(first.0 <= last.0);
        let base = first.0;
        let length = last.0 - first.0 + 1;
        Self { base, length }
    }

    /// Returns `true` if the Unicode span contains no character.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of characters in the Unicode span.
    pub fn len(&self) -> usize {
        self.length as usize
    }

    /// Returns the first code point in the Unicode span.
    pub fn first_code_point(&self) -> CodePoint {
        debug_assert!(!self.is_empty());
        code_point!(self.base)
    }

    /// Returns the last code point in the Unicode span.
    pub fn last_code_point(&self) -> CodePoint {
        debug_assert!(!self.is_empty());
        code_point!(self.base + self.length - 1)
    }

    /// Returns a code point just before the first code point in the Unicode span.
    pub fn prev_code_point(&self) -> CodePoint {
        debug_assert!(!self.is_empty());
        debug_assert!(self.base > 0);
        code_point!(self.base - 1)
    }

    /// Returns a code point just after the last code point in the Unicode span.
    pub fn next_code_point(&self) -> CodePoint {
        debug_assert!(!self.is_empty());
        debug_assert!(self.base + self.length <= (char::MAX as u32));
        code_point!(self.base + self.length)
    }

    /// Returns `true` if the Unicode span contains a specified code point.
    pub fn contains_code_point(&self, cp: CodePoint) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.first_code_point() > cp {
            return false;
        }
        if self.last_code_point() < cp {
            return false;
        }
        true
    }

    /// Returns `true` if the Unicode span contains a specified Unicode span.
    ///
    /// An empty Unicode span doesn't contain any other spans and is not contained in any other
    /// spans.
    pub fn contains(&self, other: &Self) -> bool {
        if self.is_empty() {
            return false;
        }
        if other.is_empty() {
            return false;
        }
        self.contains_code_point(other.first_code_point())
            && self.contains_code_point(other.last_code_point())
    }

    /// Expands the both endpoints of the Unicode span by a specified number if possible.
    pub fn expand(&self, n: u32) -> Self {
        if self.is_empty() {
            return self.clone();
        }
        let first = if self.base < n { 0 } else { self.base - n };
        let last = (char::MAX as u32).min(self.base + self.length - 1 + n);
        Self::new(first.into(), last.into())
    }

    pub fn can_merge(&self, other: &Self) -> bool {
        !other.expand(1).intersect(self).is_empty()
    }

    pub fn merge(&self, other: &Self) -> Self {
        debug_assert!(self.can_merge(other));
        if self.contains(other) {
            return self.clone();
        }
        if other.contains(self) {
            return other.clone();
        }
        if self.contains_code_point(other.first_code_point()) {
            return UnicodeSpan::new(self.first_code_point(), other.last_code_point());
        }
        if self.contains_code_point(other.last_code_point()) {
            return UnicodeSpan::new(other.first_code_point(), self.last_code_point());
        }
        let expanded = other.expand(1);
        if self.first_code_point() == expanded.last_code_point() {
            return UnicodeSpan::new(other.first_code_point(), self.last_code_point());
        }
        debug_assert!(self.last_code_point() == expanded.first_code_point());
        UnicodeSpan::new(self.first_code_point(), other.last_code_point())
    }

    pub fn intersect(&self, other: &Self) -> Self {
        if other.is_empty() {
            return UnicodeSpan::EMPTY;
        }
        // self
        // --*----#----#-------*-------
        //      span
        if self.contains(other) {
            return other.clone();
        }
        //       self
        // --#----*----*-------#-------
        // span
        if other.contains(self) {
            return self.clone();
        }
        // self
        // --*----#----*-------#-------
        //      span
        if self.contains_code_point(other.first_code_point()) {
            return UnicodeSpan::new(other.first_code_point(), self.last_code_point());
        }
        //      self
        // --#----*----#-------*-------
        // span
        if self.contains_code_point(other.last_code_point()) {
            return UnicodeSpan::new(self.first_code_point(), other.last_code_point());
        }
        // self
        // --*-------*----#-------#-------
        //              span
        //
        //              self
        // --#-------#----*-------*-------
        // span
        UnicodeSpan::EMPTY
    }

    pub fn exclude(&self, other: &Self) -> SmallVec<[Self; 2]> {
        if self.is_empty() {
            return smallvec![];
        }
        if other.is_empty()
            || self.first_code_point() > other.last_code_point()
            || self.last_code_point() < other.first_code_point()
        {
            // No intersection.
            return smallvec![self.clone()];
        }
        let mut remaining = smallvec![];
        if self.contains_code_point(other.first_code_point())
            && self.first_code_point() != other.first_code_point()
        {
            remaining.push(UnicodeSpan::new(
                self.first_code_point(),
                other.prev_code_point(),
            ));
        }
        if self.contains_code_point(other.last_code_point())
            && self.last_code_point() != other.last_code_point()
        {
            remaining.push(UnicodeSpan::new(
                other.next_code_point(),
                self.last_code_point(),
            ));
        }
        remaining
    }
}

impl From<()> for UnicodeSpan {
    fn from(_: ()) -> Self {
        Self { base: 0, length: 0 }
    }
}

impl From<char> for UnicodeSpan {
    fn from(value: char) -> Self {
        Self {
            base: value as u32,
            length: 1,
        }
    }
}

impl From<Range<char>> for UnicodeSpan {
    fn from(value: Range<char>) -> Self {
        let start = value.start as u32;
        let end = value.end as u32;
        let base = if start == end { 0 } else { start };
        let length = end - start;
        Self { base, length }
    }
}

impl From<RangeInclusive<char>> for UnicodeSpan {
    fn from(value: RangeInclusive<char>) -> Self {
        let start = *value.start() as u32;
        let end = *value.end() as u32;
        let base = if start == end { 0 } else { start };
        let length = end - start + 1;
        Self { base, length }
    }
}

impl std::fmt::Display for UnicodeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "(empty)")
        } else if self.len() == 1 {
            write!(f, "{}", self.first_code_point())
        } else {
            write!(f, "{}..{}", self.first_code_point(), self.last_code_point())
        }
    }
}

/// Represents a set of Unicode spans.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct UnicodeSet {
    spans: Arc<Vec<UnicodeSpan>>, // sorted

    /// `true` if the EOF is allowed, otherwise `false`.
    ///
    /// The EOF has no actual code point in Unicode, but it's needed for processing a transition
    /// generated from lookahead terms in a lexical grammar.  We call such a transition a lookahead
    /// transition.
    ///
    /// For example, the ES2022 lexical grammar contains the following rule:
    ///
    /// ```text
    /// LineTerminatorSequence ::
    ///   <LF>
    ///   <CR> [lookahead != <LF>]
    ///   <LS>
    ///   <PS>
    ///   <CR> <LF>
    /// ```
    ///
    /// The `LineTerminatorSequenec` can be recognized by the following NFA:
    ///
    /// ```text
    ///                    { <LF>, <LS>, <PS> }
    /// [State(0):Start] ------------------------> [State(1):LineTerminatorSequence]
    ///    |                                          A
    ///    |                                          |
    ///    +------------> [State(2)]------------------+
    ///       { <CR> }       :               { <LF> }
    ///                      : { Others }
    ///                      : (lookahead transition)
    ///                      V
    ///                   [State(3):LineTerminatorSequence]
    /// ```
    ///
    /// Where `{ Others }` must include the EOF so that a single `<CR>` character can be recognized
    /// as the `LineTerminatorSequenec`.
    eof: bool,
}

macro_rules! define_builtin_unicode_set {
    ($fname:ident, $unicode_set:expr) => {
        pub fn $fname() -> Self {
            $unicode_set
        }
    };
}

impl UnicodeSet {
    pub fn empty() -> Self {
        UnicodeSet {
            spans: Arc::new(vec![]),
            eof: false,
        }
    }

    pub fn any() -> Self {
        UnicodeSet {
            spans: Arc::new(vec![UnicodeSpan::ANY]),
            eof: false,
        }
    }

    define_builtin_unicode_set! {tab, unicode_set!['\u{0009}']}
    define_builtin_unicode_set! {vt, unicode_set!['\u{000B}']}
    define_builtin_unicode_set! {ff, unicode_set!['\u{000C}']}
    define_builtin_unicode_set! {sp, unicode_set![' ']}
    define_builtin_unicode_set! {usp, unicode_set![
        ' ', '\u{00A0}', '\u{1680}', '\u{2000}'..='\u{200A}', '\u{200F}', '\u{205F}',
    ]}
    define_builtin_unicode_set! {lf, unicode_set!['\u{000A}']}
    define_builtin_unicode_set! {cr, unicode_set!['\u{000D}']}
    define_builtin_unicode_set! {ls, unicode_set!['\u{2028}']}
    define_builtin_unicode_set! {ps, unicode_set!['\u{2029}']}
    define_builtin_unicode_set! {zwnj, unicode_set!['\u{200C}']}
    define_builtin_unicode_set! {zwj, unicode_set!['\u{200D}']}
    define_builtin_unicode_set! {zwnbsp, unicode_set!['\u{FEFF}']}

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() && !self.eof
    }

    #[allow(dead_code)]
    pub fn spans(&self) -> &[UnicodeSpan] {
        self.spans.as_slice()
    }

    #[allow(dead_code)]
    pub fn first_code_point(&self) -> Option<CodePoint> {
        self.spans.first().map(UnicodeSpan::first_code_point)
    }

    #[allow(dead_code)]
    pub fn last_code_point(&self) -> Option<CodePoint> {
        self.spans.last().map(UnicodeSpan::last_code_point)
    }

    #[allow(dead_code)]
    pub fn contains_eof(&self) -> bool {
        self.eof
    }

    pub fn contains_span(&self, other: &UnicodeSpan) -> bool {
        self.spans.iter().any(|span| span.contains(other))
    }

    pub fn contains(&self, other: &Self) -> bool {
        // There are more efficient algorithms, but we choice a simple one which
        // takes O(N*M) time complexity, from maintenance cost point of view.
        (!other.eof || self.eof) && other.spans.iter().all(|other| self.contains_span(other))
    }

    pub fn merge_eof(&self) -> Self {
        let mut set = self.clone();
        set.eof = true;
        set
    }

    pub fn merge_span(&self, span: &UnicodeSpan) -> Self {
        debug_assert!(!span.is_empty());
        let mut spans = vec![];
        let mut added = false;
        let mut span = span.clone();
        for mine in self.spans.iter() {
            if span.can_merge(mine) {
                span = span.merge(mine);
            } else if span.first_code_point() > mine.last_code_point() {
                spans.push(mine.clone());
            } else {
                debug_assert!(span.last_code_point() < mine.first_code_point());
                if !added {
                    spans.push(span.clone());
                    added = true;
                }
                spans.push(mine.clone());
            }
        }
        if !added {
            spans.push(span.clone());
        }
        UnicodeSet {
            spans: Arc::new(spans),
            eof: self.eof,
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        if other.is_empty() {
            return self.clone();
        }
        // There are more efficient algorithms, but we choice a simple one which
        // takes O(N*M) time complexity, from maintenance cost point of view.
        let mut set = self.clone();
        for span in other.spans.iter() {
            set = set.merge_span(span);
        }
        if other.eof {
            set.eof = true;
        }
        set
    }

    pub fn intersect(&self, other: &Self) -> Self {
        if other.is_empty() {
            return Self::empty();
        }
        // There are more efficient algorithms, but we choice a simple one which
        // takes O(N*M) time complexity, from maintenance cost point of view.
        let mut spans = vec![];
        for span in other.spans.iter() {
            let intersection = self.intersect_span(span);
            spans.extend(intersection.spans.iter().cloned());
        }
        UnicodeSet {
            spans: Arc::new(spans),
            eof: self.eof && other.eof,
        }
    }

    pub fn intersect_span(&self, span: &UnicodeSpan) -> Self {
        let mut spans = vec![];
        for mine in self.spans.iter() {
            let intersection = span.intersect(mine);
            if !intersection.is_empty() {
                spans.push(intersection);
            }
        }
        spans.into()
    }

    pub fn exclude(&self, other: &Self) -> Self {
        if other.is_empty() {
            return self.clone();
        }
        // There are more efficient algorithms, but we choice a simple one which
        // takes O(N*M) time complexity, from maintenance cost point of view.
        let mut set = self.clone();
        for span in other.spans.iter() {
            set = set.exclude_span(span);
        }
        if other.eof {
            set.eof = false;
        }
        set
    }

    pub fn exclude_span(&self, span: &UnicodeSpan) -> Self {
        let mut spans = vec![];
        for mine in self.spans.iter() {
            spans.extend(mine.exclude(span).iter().cloned());
        }
        UnicodeSet {
            spans: Arc::new(spans),
            eof: self.eof,
        }
    }
}

impl From<UnicodeSpan> for UnicodeSet {
    fn from(span: UnicodeSpan) -> Self {
        debug_assert!(!span.is_empty());
        UnicodeSet {
            spans: Arc::new(vec![span]),
            eof: false,
        }
    }
}

impl From<Vec<UnicodeSpan>> for UnicodeSet {
    fn from(spans: Vec<UnicodeSpan>) -> Self {
        UnicodeSet {
            spans: Arc::new(spans),
            eof: false,
        }
    }
}

impl std::fmt::Display for UnicodeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        match self.spans.split_last() {
            Some((last, leadings)) => {
                for span in leadings.iter() {
                    write!(f, "{}, ", span)?;
                }
                write!(f, "{}", last)?;
                if self.eof {
                    write!(f, ", (eof)")?;
                }
            }
            None => {
                if self.eof {
                    write!(f, "(eof)")?;
                }
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, Default)]
pub struct UnicodeSetsBuilder {
    sets: Vec<UnicodeSet>,
}

impl UnicodeSetsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(dead_code)]
    pub fn contains(&self, unicode_set: &UnicodeSet) -> bool {
        self.sets.iter().any(|set| set == unicode_set)
    }

    pub fn add(&mut self, unicode_set: &UnicodeSet) -> &mut Self {
        if unicode_set.is_empty() {
            return self;
        }

        let mut unicode_set = unicode_set.clone();
        let mut sets = vec![];
        for us in self.sets.iter() {
            let intersection = unicode_set.intersect(us);
            if intersection.is_empty() {
                sets.push(us.clone());
            } else {
                sets.push(intersection.clone());
                let remaining = us.exclude(&intersection);
                if !remaining.is_empty() {
                    sets.push(remaining);
                }
            }
            unicode_set = unicode_set.exclude(&intersection);
        }
        if !unicode_set.is_empty() {
            sets.push(unicode_set);
        }
        self.sets = sets;
        self
    }

    pub fn build(self) -> Vec<UnicodeSet> {
        self.sets
    }
}

impl std::fmt::Display for UnicodeSetsBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if let Some((last, leadings)) = self.sets.split_last() {
            for set in leadings.iter() {
                write!(f, "{}, ", set)?;
            }
            write!(f, "{}", last)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    logging::init!();

    #[test]
    fn test_code_point_get_char() {
        assert_eq!(code_point!('\0').get_char(), '\0');
    }

    #[test]
    fn test_code_point_format() {
        assert_eq!(format!("{}", code_point!('\0')), "<NUL>");
        assert_eq!(format!("{}", code_point!(' ')), "<SP>");
        assert_eq!(format!("{}", code_point!('A')), "A");
        assert_eq!(format!("{}", code_point!('\x7F')), "<DEL>");
        assert_eq!(format!("{}", code_point!('\u{ABCD}')), "U+ABCD");
        assert_eq!(format!("{}", code_point!('\u{1ABCD}')), "U+01ABCD");
    }

    #[test]
    fn test_unicode_span_is_empty() {
        assert!(UnicodeSpan::EMPTY.is_empty());
        assert!(UnicodeSpan::default().is_empty());
        assert!(!unicode_span!('0'..='0').is_empty());
    }

    #[test]
    fn test_unicode_span_len() {
        assert_eq!(UnicodeSpan::EMPTY.len(), 0);
        assert_eq!(UnicodeSpan::default().len(), 0);
        assert_eq!(unicode_span!('a').len(), 1);
        assert_eq!(unicode_span!('0'..='9').len(), 10);
    }

    #[test]
    fn test_unicode_span_first_code_point() {
        assert_eq!(
            unicode_span!('0'..='9').first_code_point(),
            code_point!('0')
        );
    }

    #[test]
    fn test_unicode_span_contains_code_point() {
        let span = unicode_span!('2'..='7');
        assert!(!span.contains_code_point(code_point!('1')));
        assert!(span.contains_code_point(code_point!('2')));
        assert!(span.contains_code_point(code_point!('5')));
        assert!(span.contains_code_point(code_point!('7')));
        assert!(!span.contains_code_point(code_point!('8')));
    }

    #[test]
    fn test_unicode_span_contains() {
        assert!(unicode_span!('2'..='7').contains(&unicode_span!('2'..='7')));
        assert!(unicode_span!('2'..='7').contains(&unicode_span!('3'..='6')));
        assert!(!unicode_span!('2'..='7').contains(&unicode_span!('1'..='8')));
        assert!(!unicode_span!('2'..='7').contains(&unicode_span!('0'..='5')));
        assert!(!unicode_span!('2'..='7').contains(&unicode_span!('5'..='9')));
        assert!(!unicode_span!('2'..='7').contains(&unicode_span!('0'..='1')));
        assert!(!unicode_span!('2'..='7').contains(&unicode_span!('8'..='9')));
        assert!(!unicode_span!('2'..='7').contains(&UnicodeSpan::EMPTY));
        assert!(!UnicodeSpan::EMPTY.contains(&UnicodeSpan::EMPTY));
        assert!(UnicodeSpan::ANY.contains(&unicode_span!('2'..='7')));
        assert!(UnicodeSpan::ANY.contains(&UnicodeSpan::ANY));
    }

    #[test]
    fn test_unicode_span_expand() {
        assert_eq!(unicode_span!('2'..='7').expand(1), unicode_span!('1'..='8'));
        assert_eq!(unicode_span!('\0').expand(1), unicode_span!(0, 1));
        assert_eq!(UnicodeSpan::EMPTY.expand(1), UnicodeSpan::EMPTY);
        assert_eq!(UnicodeSpan::ANY.expand(1), UnicodeSpan::ANY);
    }

    #[test]
    fn test_unicode_span_can_merge() {
        assert!(!unicode_span!('2'..='7').can_merge(&unicode_span!('0')));
        assert!(unicode_span!('2'..='7').can_merge(&unicode_span!('1')));
        assert!(unicode_span!('2'..='7').can_merge(&unicode_span!('2')));
        assert!(unicode_span!('2'..='7').can_merge(&unicode_span!('6')));
        assert!(unicode_span!('2'..='7').can_merge(&unicode_span!('7')));
        assert!(unicode_span!('2'..='7').can_merge(&unicode_span!('8')));
        assert!(!unicode_span!('2'..='7').can_merge(&unicode_span!('9')));
        assert!(!unicode_span!('2'..='7').can_merge(&UnicodeSpan::EMPTY));
    }

    #[test]
    fn test_unicode_span_merge() {
        assert_eq!(
            unicode_span!('2'..='7').merge(&unicode_span!('5')),
            unicode_span!('2'..='7')
        );
        assert_eq!(
            unicode_span!('2'..='7').merge(&unicode_span!('0'..='9')),
            unicode_span!('0'..='9')
        );
        assert_eq!(
            unicode_span!('2'..='7').merge(&unicode_span!('5'..='9')),
            unicode_span!('2'..='9')
        );
        assert_eq!(
            unicode_span!('2'..='7').merge(&unicode_span!('0'..='5')),
            unicode_span!('0'..='7')
        );
        assert_eq!(
            unicode_span!('2'..='7').merge(&unicode_span!('1')),
            unicode_span!('1'..='7')
        );
        assert_eq!(
            unicode_span!('2'..='7').merge(&unicode_span!('8')),
            unicode_span!('2'..='8')
        );
    }

    #[test]
    fn test_unicode_span_intersect() {
        assert_eq!(
            unicode_span!('2'..='7').intersect(&UnicodeSpan::EMPTY),
            UnicodeSpan::EMPTY
        );
        assert_eq!(
            unicode_span!('2'..='7').intersect(&unicode_span!('3'..='6')),
            unicode_span!('3'..='6')
        );
        assert_eq!(
            unicode_span!('2'..='7').intersect(&unicode_span!('1'..='8')),
            unicode_span!('2'..='7')
        );
        assert_eq!(
            unicode_span!('2'..='7').intersect(&unicode_span!('5'..='9')),
            unicode_span!('5'..='7')
        );
        assert_eq!(
            unicode_span!('2'..='7').intersect(&unicode_span!('0'..='5')),
            unicode_span!('2'..='5')
        );
        assert_eq!(
            unicode_span!('2'..='7').intersect(&unicode_span!('0'..='1')),
            UnicodeSpan::EMPTY
        );
        assert_eq!(
            unicode_span!('2'..='7').intersect(&unicode_span!('8'..='9')),
            UnicodeSpan::EMPTY
        );
    }

    #[test]
    fn test_unicode_span_exclude() {
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![unicode_span!('2'..='7')];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&UnicodeSpan::EMPTY),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('2'..='7')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> =
            smallvec![unicode_span!('2'), unicode_span!('7')];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('3'..='6')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('1'..='8')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![unicode_span!('2'..='4')];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('5'..='9')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![unicode_span!('6'..='7')];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('0'..='5')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![unicode_span!('2'..='7')];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('0'..='1')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![unicode_span!('2'..='7')];
        assert_eq!(
            unicode_span!('2'..='7').exclude(&unicode_span!('8'..='9')),
            expected
        );
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![];
        assert_eq!(UnicodeSpan::EMPTY.exclude(&UnicodeSpan::EMPTY), expected);
        let expected: SmallVec<[UnicodeSpan; 2]> = smallvec![];
        assert_eq!(
            UnicodeSpan::EMPTY.exclude(&unicode_span!('2'..='7')),
            expected
        );
    }

    #[test]
    fn test_unicode_span_format() {
        assert_eq!(format!("{}", UnicodeSpan::EMPTY), "(empty)");
        assert_eq!(format!("{}", unicode_span!('A')), "A");
        assert_eq!(format!("{}", unicode_span!('0'..='9')), "0..9");
    }

    #[test]
    fn test_unicode_set_contains() {
        let digit = unicode_set!['0'..='9'];
        let upper = unicode_set!['A'..='Z'];
        let lower = unicode_set!['a'..='z'];
        let upper_alnum = unicode_set!['0'..='9', 'A'..='Z'];
        let lower_alnum = unicode_set!['0'..='9', 'a'..='z'];
        assert!(lower_alnum.contains(&digit));
        assert!(lower_alnum.contains(&digit));
        assert!(lower_alnum.contains(&lower));
        assert!(lower_alnum.contains(&lower_alnum));
        assert!(!lower_alnum.contains(&upper));
        assert!(!lower_alnum.contains(&upper_alnum));
        assert!(lower_alnum.merge_eof().contains(&digit));
        assert!(!lower_alnum.contains(&digit.merge_eof()));
        assert!(lower_alnum.merge_eof().contains(&digit.merge_eof()));
    }

    #[test]
    fn test_unicode_set_merge() {
        let digit = unicode_set!['0'..='9'];
        let alpha = unicode_set!['a'..='z'];
        let alnum = unicode_set!['0'..='9', 'a'..='z'];
        assert_eq!(digit.merge(&unicode_set![]), digit);
        assert_eq!(digit.merge(&alpha), alnum);
        assert_eq!(alpha.merge(&digit), alnum);
        assert_eq!(
            alnum.merge(&unicode_set!['5'..='k']),
            unicode_set!['0'..='z']
        );
    }

    #[test]
    fn test_unicode_set_intersect() {
        let digit = unicode_set!['0'..='9'];
        let alpha = unicode_set!['a'..='z'];
        let alnum = unicode_set!['0'..='9', 'a'..='z'];
        assert_eq!(digit.intersect(&unicode_set![]), unicode_set![]);
        assert_eq!(unicode_set![].intersect(&digit), unicode_set![]);
        assert_eq!(digit.intersect(&digit), digit);
        assert_eq!(digit.intersect(&alpha), unicode_set![]);
        assert_eq!(alnum.intersect(&digit), digit);
        assert_eq!(
            alnum.intersect(&unicode_set!['5'..='k']),
            unicode_set!['5'..='9', 'a'..='k']
        );
    }

    #[test]
    fn test_unicode_set_exclude() {
        let digit = unicode_set!['0'..='9'];
        let alpha = unicode_set!['a'..='z'];
        let alnum = unicode_set!['0'..='9', 'a'..='z'];
        assert_eq!(digit.exclude(&UnicodeSet::empty()), digit);
        assert_eq!(digit.exclude(&alpha), digit);
        assert_eq!(alnum.exclude(&digit), alpha);
        assert_eq!(
            digit.exclude(&unicode_set!['4'..='6']),
            unicode_set!['0'..='3', '7'..='9']
        );
        assert_eq!(
            alnum.exclude(&unicode_set!['5'..='k']),
            unicode_set!['0'..='4', 'l'..='z']
        );
    }

    #[test]
    fn test_unicode_set_format() {
        let digit = unicode_set!['0'..='9'];
        let alnum = unicode_set!['0'..='9', 'a'..='z'];
        assert_eq!(format!("{}", digit), "[0..9]");
        assert_eq!(format!("{}", alnum), "[0..9, a..z]");
        assert_eq!(format!("{}", UnicodeSet::empty().merge_eof()), "[(eof)]");
        assert_eq!(format!("{}", alnum.merge_eof()), "[0..9, a..z, (eof)]");
    }

    #[test]
    fn test_unicode_sets_builder_contains() {
        let mut builder = UnicodeSetsBuilder::default();
        let space = unicode_set![' '];
        assert!(!builder.contains(&space));
        builder.add(&space);
        assert!(builder.contains(&space));
        assert!(!builder.contains(&unicode_set!['A']));
    }

    #[test]
    fn test_unicode_sets_builder_add() {
        let mut builder = UnicodeSetsBuilder::default();

        let alnum = unicode_set!['0'..='9', 'A'..='Z', 'a'..='z'];
        builder.add(&alnum);
        assert_eq!(builder.sets.len(), 1);
        assert!(builder.contains(&alnum));

        builder.add(&unicode_set![]);
        assert_eq!(builder.sets.len(), 1);
        assert!(builder.contains(&alnum));

        let space = unicode_set![' '];
        builder.add(&space);
        assert_eq!(builder.sets.len(), 2);
        assert!(builder.contains(&space));
        assert!(builder.contains(&alnum));

        let digit = unicode_set!['0'..='9'];
        builder.add(&digit);
        assert_eq!(builder.sets.len(), 3);
        assert!(builder.contains(&space));
        assert!(builder.contains(&digit));
        assert!(builder.contains(&unicode_set!['A'..='Z', 'a'..='z']));

        builder.add(&unicode_set!['5'..='K']);
        assert_eq!(builder.sets.len(), 6);
        assert!(builder.contains(&space));
        assert!(builder.contains(&unicode_set!['0'..='4']));
        assert!(builder.contains(&unicode_set!['5'..='9']));
        assert!(builder.contains(&unicode_set![':'..='@']));
        assert!(builder.contains(&unicode_set!['A'..='K']));
        assert!(builder.contains(&unicode_set!['L'..='Z', 'a'..='z']));
    }

    #[test]
    fn test_unicode_sets_builder_format() {
        let mut builder = UnicodeSetsBuilder::default();
        assert_eq!(format!("{}", builder), "[]");
        builder.add(&unicode_set!['a']);
        assert_eq!(format!("{}", builder), "[[a]]");
    }
}
