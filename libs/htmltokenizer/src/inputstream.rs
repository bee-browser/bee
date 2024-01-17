use crate::Location;

pub(crate) struct InputStream {
    data: Vec<u16>,
    pos: usize,
    end: bool,
    maybe_dos_eol: bool,
    location: Location,
}

impl InputStream {
    pub(crate) fn new() -> Self {
        InputStream {
            data: vec![],
            pos: 0,
            end: false,
            maybe_dos_eol: false,
            location: Default::default(),
        }
    }

    pub(crate) fn next_code_point(&mut self) -> Option<(CodePoint, Location)> {
        loop {
            match Self::decode(&self.data[self.pos..]) {
                (Some(CodePoint::Scalar(0x000A)), _) => {
                    if self.maybe_dos_eol {
                        self.maybe_dos_eol = false;
                        self.consume_dos_eol();
                    } else {
                        return self.consume_eol();
                    }
                }
                (Some(CodePoint::Scalar(0x000D)), _) => {
                    self.maybe_dos_eol = true;
                    return self.consume_eol();
                }
                (Some(CodePoint::Scalar(cp)), nunits) => {
                    self.maybe_dos_eol = false;
                    return self.consume(cp, nunits);
                }
                (Some(CodePoint::Surrogate(cp)), 1) => {
                    self.maybe_dos_eol = false;
                    return self.surrogate_in_input_stream(cp);
                }
                _ => {
                    if self.end {
                        return self.eof();
                    } else {
                        return self.insufficient_input();
                    }
                }
            }
        }
    }

    pub(crate) fn feed_data(&mut self, data: &[u16]) {
        self.data.extend_from_slice(data);
    }

    pub(crate) fn feed_end(&mut self) {
        self.end = true;
    }

    #[inline]
    fn decode(data: &[u16]) -> (Option<CodePoint>, usize) {
        if data.is_empty() {
            return (None, 0);
        }

        let first_unit = data[0] as u32;
        match first_unit {
            // high surrogate
            0xD800..=0xDBFF => {
                if data.len() < 2 {
                    (Some(CodePoint::Surrogate(first_unit)), 1)
                } else {
                    let second_unit = data[1] as u32;
                    match second_unit {
                        // low surrogate
                        0xDC00..=0xDFFF => {
                            let v =
                                (((first_unit & 0x3FF) << 10) | (second_unit & 0x3FF)) + 0x10000;
                            (Some(CodePoint::Scalar(v)), 2)
                        }
                        _ => (Some(CodePoint::Surrogate(first_unit)), 1),
                    }
                }
            }
            // low surrogate
            0xDC00..=0xDFFF => (Some(CodePoint::Surrogate(first_unit)), 1),
            _ => (Some(CodePoint::Scalar(first_unit)), 1),
        }
    }

    #[inline]
    fn consume(&mut self, cp: u32, nunits: usize) -> Option<(CodePoint, Location)> {
        let location = self.location;
        self.pos += nunits;
        self.location.incr();
        Some((CodePoint::Scalar(cp), location))
    }

    #[inline]
    fn consume_eol(&mut self) -> Option<(CodePoint, Location)> {
        let location = self.location;
        self.pos += 1;
        self.location.incr_line();
        Some((CodePoint::Scalar('\n' as u32), location))
    }

    #[inline]
    fn consume_dos_eol(&mut self) {
        self.pos += 1;
    }

    #[inline]
    fn eof(&self) -> Option<(CodePoint, Location)> {
        Some((CodePoint::Eof, self.location))
    }

    #[inline]
    fn surrogate_in_input_stream(&mut self, cp: u32) -> Option<(CodePoint, Location)> {
        let location = self.location;
        self.pos += 1;
        self.location.incr_line();
        Some((CodePoint::Surrogate(cp), location))
    }

    #[inline]
    fn insufficient_input(&self) -> Option<(CodePoint, Location)> {
        None
    }
}

#[derive(Debug)]
pub(crate) enum CodePoint {
    Scalar(u32),
    Surrogate(u32),
    Eof,
}
