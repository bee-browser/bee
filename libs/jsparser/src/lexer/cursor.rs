use super::logger;

#[derive(Clone)]
pub struct SourceCursor<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> SourceCursor<'a> {
    pub fn new(src: &'a str) -> Self {
        SourceCursor { src, pos: 0 }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.src.len()
    }

    #[inline(always)]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline(always)]
    pub fn chars(&self) -> std::str::CharIndices<'a> {
        self.src.get(self.pos..).unwrap().char_indices()
    }

    #[inline(always)]
    pub fn lexeme(&self, end: usize) -> &'a str {
        self.src.get(self.pos..end).unwrap()
    }

    #[inline(always)]
    pub fn src(&self) -> &'a str {
        self.src
    }

    #[inline(always)]
    pub fn advance(&mut self, n: usize) {
        self.pos += n;
        logger::trace!(opcode = "advance", cursor.pos = self.pos);
    }

    #[inline(always)]
    pub fn eof(&self) -> bool {
        self.pos == self.src.len()
    }
}
