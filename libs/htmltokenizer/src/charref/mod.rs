mod trie;

#[derive(Default)]
pub(crate) struct CharRefResolver {
    node_index: usize,
    last_valid: usize,
}

impl CharRefResolver {
    pub(crate) fn reset(&mut self) {
        self.node_index = 0;
        self.last_valid = 0;
    }

    pub(crate) fn accept(&mut self, c: char) -> bool {
        match self.node().next(c) {
            0 => false,
            node_index => {
                self.node_index = node_index;
                if !self.node().chars.is_empty() {
                    self.last_valid = node_index;
                }
                true
            }
        }
    }

    pub(crate) fn end(&self) -> bool {
        self.node().end
    }

    pub(crate) fn resolve(&self) -> Option<(&'static str, &'static str)> {
        let node = self.last_valid_node();
        if node.chars.is_empty() {
            None
        } else {
            Some((node.buffer, node.chars))
        }
    }

    pub(crate) fn buffer(&self) -> &'static str {
        &self.node().buffer[..]
    }

    pub(crate) fn remaining(&self) -> &'static str {
        let len = self.last_valid_node().buffer.len();
        &self.node().buffer[len..]
    }

    fn node(&self) -> &'static TrieNode {
        &trie::NODES[self.node_index]
    }

    fn last_valid_node(&self) -> &'static TrieNode {
        &trie::NODES[self.last_valid]
    }
}

struct TrieNode {
    buffer: &'static str,
    next: [usize; 63],
    chars: &'static str,
    end: bool,
}

impl TrieNode {
    fn next(&self, c: char) -> usize {
        // Map a character to an index which works as a character class in an
        // automaton in order to reduce the size of a transition table in each
        // node in a trie.
        //
        // '0'..='9' => 0..=9
        // 'A'..='Z' => 10..=35
        // 'a'..='z' => 36..=61
        // ';' => 62

        const DIGIT_BASE: usize = '0' as usize;
        const UPPER_BASE: usize = 'A' as usize;
        const LOWER_BASE: usize = 'a' as usize;
        const DIGIT_OFFSET: usize = 0;
        const UPPER_OFFSET: usize = DIGIT_OFFSET + 10;
        const LOWER_OFFSET: usize = UPPER_OFFSET + 26;
        const SEMICOLON: usize = LOWER_OFFSET + 26;

        #[inline]
        fn digit_char_index(c: char) -> usize {
            (c as usize) - DIGIT_BASE + DIGIT_OFFSET
        }

        #[inline]
        fn upper_char_index(c: char) -> usize {
            (c as usize) - UPPER_BASE + UPPER_OFFSET
        }

        #[inline]
        fn lower_char_index(c: char) -> usize {
            (c as usize) - LOWER_BASE + LOWER_OFFSET
        }

        match c {
            '0'..='9' => self.next[digit_char_index(c)],
            'A'..='Z' => self.next[upper_char_index(c)],
            'a'..='z' => self.next[lower_char_index(c)],
            ';' => self.next[SEMICOLON],
            _ => 0,
        }
    }
}

//<coverage:exclude>
#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test() {
        let mut resolver = CharRefResolver::default();
        assert!(!resolver.end());
        assert_eq!(resolver.remaining(), "");
        assert_matches!(resolver.resolve(), None);

        assert!(resolver.accept('l'));
        assert!(!resolver.end());
        assert_eq!(resolver.remaining(), "l");
        assert_matches!(resolver.resolve(), None);

        assert!(resolver.accept('t'));
        assert!(!resolver.end());
        assert_eq!(resolver.remaining(), "");
        assert_matches!(resolver.resolve(), Some(("lt", "<")));

        assert!(resolver.accept(';'));
        assert!(resolver.end());
        assert_eq!(resolver.remaining(), "");
        assert_matches!(resolver.resolve(), Some(("lt;", "<")));

        assert!(!resolver.accept('x'));
    }
}
//</coverage:exclude>
