use std::ops::Range;

use crate::error::Error;

pub enum Token<'a> {
    Doctype(Doctype<'a>),
    StartTag(Tag<'a>),
    EndTag(Tag<'a>),
    Comment(Comment<'a>),
    Null(Text<'a>),
    Whitespace(Text<'a>),
    Text(Text<'a>),
    Error(Error),
    End,
}

impl<'a> Token<'a> {
    pub(crate) fn new(token: TokenRange, buf: &'a str) -> Self {
        match token {
            TokenRange::Doctype(doctype) => Token::Doctype(Doctype::new(doctype, buf)),
            TokenRange::Tag(tag) => {
                if tag.start_tag {
                    Token::StartTag(Tag::new(tag, buf))
                } else {
                    Token::EndTag(Tag::new(tag, buf))
                }
            }
            TokenRange::Comment(data) => Token::Comment(Comment::new(data, buf)),
            TokenRange::Null(data) => Token::Null(Text::new(data, buf)),
            TokenRange::Whitespace(data) => Token::Whitespace(Text::new(data, buf)),
            TokenRange::Text(data) => Token::Text(Text::new(data, buf)),
            TokenRange::Error(err) => Token::Error(err),
        }
    }
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Doctype(ref doctype) => {
                write!(f, "<!DOCTYPE")?;
                if let Some(name) = doctype.name {
                    write!(f, " {}", name)?;
                }
                write!(f, ">")
            }
            Token::StartTag(ref tag) => {
                write!(f, "<{}", tag.name)?;
                if tag.self_closing {
                    write!(f, "/>")
                } else {
                    write!(f, ">")
                }
            }
            Token::EndTag(ref tag) => {
                write!(f, "</{}>", tag.name)
            }
            Token::Comment(ref comment) => {
                write!(f, "#comment:{}", comment.data.escape_debug())
            }
            Token::Null(ref text) | Token::Whitespace(ref text) | Token::Text(ref text) => {
                write!(f, "#text:{}", text.data.escape_debug())
            }
            Token::Error(ref err) => {
                write!(f, "{:?}", err)
            }
            Token::End => {
                write!(f, "eof")
            }
        }
    }
}

#[derive(Debug)]
pub struct Doctype<'a> {
    pub name: Option<&'a str>,
    pub public_id: Option<&'a str>,
    pub system_id: Option<&'a str>,
    pub force_quirks: bool,
}

impl<'a> Doctype<'a> {
    fn new(doctype: DoctypeRange, buf: &'a str) -> Self {
        Doctype {
            name: doctype.name.map(|range| &buf[range]),
            public_id: doctype.public_id.map(|range| &buf[range]),
            system_id: doctype.system_id.map(|range| &buf[range]),
            force_quirks: doctype.force_quirks,
        }
    }
}

#[derive(Debug)]
pub struct Tag<'a> {
    pub name: &'a str,
    attrs: AttrsHolder<'a>,
    pub self_closing: bool,
}

impl<'a> Tag<'a> {
    fn new(tag: TagRange, buf: &'a str) -> Self {
        Tag {
            name: &buf[tag.name],
            attrs: AttrsHolder::new(buf, tag.attrs),
            self_closing: tag.self_closing,
        }
    }

    pub fn with_no_attrs(name: &'a str) -> Self {
        Tag {
            name,
            attrs: AttrsHolder::empty(),
            self_closing: false,
        }
    }

    pub fn rename(&self, name: &'a str) -> Self {
        Tag {
            name,
            attrs: self.attrs.clone(),
            self_closing: self.self_closing,
        }
    }

    pub fn has_any_attributes(&self, names: &[&str]) -> bool {
        self.attrs().any(|(name, _)| names.contains(&name))
    }

    pub fn attrs(&self) -> Attrs<'a, '_> {
        Attrs::new(self.attrs.buffer, &self.attrs.attrs)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AttrsHolder<'a> {
    buffer: &'a str,
    attrs: Vec<AttrRange>,
}

impl<'a> AttrsHolder<'a> {
    pub(crate) fn new(buffer: &'a str, attrs: Vec<AttrRange>) -> Self {
        AttrsHolder { buffer, attrs }
    }

    pub fn empty() -> Self {
        AttrsHolder {
            buffer: "",
            attrs: vec![],
        }
    }
}

pub struct Attrs<'a, 'b> {
    buffer: &'a str,
    attrs: &'b [AttrRange],
    index: usize,
}

impl<'a, 'b> Attrs<'a, 'b> {
    fn new(buffer: &'a str, attrs: &'b [AttrRange]) -> Self {
        Attrs {
            buffer,
            attrs,
            index: 0,
        }
    }
}

impl<'a> Iterator for Attrs<'a, '_> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.index;
        while i < self.attrs.len() {
            if !self.attrs[i].duplicate {
                break;
            }
            i += 1;
        }

        let attr = self.attrs.get(i).map(|attr| {
            let name = &self.buffer[attr.name.clone()];
            let value = &self.buffer[attr.value.clone()];
            (name, value)
        });
        self.index = i + 1;
        attr
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub data: &'a str,
}

impl<'a> Text<'a> {
    fn new(data: Range<usize>, buf: &'a str) -> Self {
        Text { data: &buf[data] }
    }
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub data: &'a str,
}

impl<'a> Comment<'a> {
    fn new(data: Range<usize>, buf: &'a str) -> Self {
        Comment { data: &buf[data] }
    }
}

// private types

#[derive(Debug)]
pub(crate) enum TokenRange {
    Doctype(DoctypeRange),
    Tag(TagRange),
    Comment(Range<usize>),
    Null(Range<usize>),
    Whitespace(Range<usize>),
    Text(Range<usize>),
    Error(Error),
}

#[derive(Debug, Default)]
pub(crate) struct DoctypeRange {
    pub name: Option<Range<usize>>,
    pub public_id: Option<Range<usize>>,
    pub system_id: Option<Range<usize>>,
    pub force_quirks: bool,
}

#[derive(Debug)]
pub(crate) struct TagRange {
    pub name: Range<usize>,
    pub attrs: Vec<AttrRange>,
    pub self_closing: bool,
    pub start_tag: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct AttrRange {
    pub name: Range<usize>,
    pub value: Range<usize>,
    pub duplicate: bool,
}
