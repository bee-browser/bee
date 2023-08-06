#[derive(Clone)]
pub struct SourceCursor<'a> {
    src: &'a str,
    chars: Vec<(usize, char)>,
    pos: usize,
    next_pos: usize,
    token_end: usize,
}

impl<'a> SourceCursor<'a> {
    pub fn new(src: &'a str) -> Self {
        SourceCursor {
            src,
            chars: src.char_indices().collect(),
            pos: 0,
            next_pos: 0,
            token_end: 0,
        }
    }

    #[inline(always)]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline(always)]
    pub fn get(&self) -> Option<char> {
        self.chars.get(self.next_pos).map(|(_, ch)| *ch)
    }

    #[inline(always)]
    pub fn lexeme(&self) -> &'a str {
        self.src.get(self.pos..self.token_end).unwrap()
    }

    #[inline(always)]
    pub fn consume(&mut self) {
        self.next_pos += 1;
        self.token_end = self.next_pos;
        tracing::trace!(opcode = "consume", cursor.token_end = self.token_end);
    }

    #[inline(always)]
    pub fn lookahead(&mut self) {
        self.next_pos += 1;
        tracing::trace!(opcode = "lookahead", cursor.next_pos = self.next_pos);
    }

    #[inline(always)]
    pub fn advance(&mut self, n: usize) {
        self.pos += n;
        self.next_pos = self.pos;
        self.token_end = self.pos;
        tracing::trace!(opcode = "advance", cursor.pos = self.pos);
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.pos == self.chars.len()
    }
}
