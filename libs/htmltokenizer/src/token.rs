use std::ops::Range;

use bee_htmltags::HtmlTag;

use crate::error::Error;

pub enum Token<'a> {
    Doctype(Doctype<'a>),
    StartTag(Tag<'a>),
    EndTag(Tag<'a>),
    Text(Text<'a>),
    Comment(Comment<'a>),
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
            TokenRange::Text(data) => Token::Text(Text::new(data, buf)),
            TokenRange::Comment(data) => Token::Comment(Comment::new(data, buf)),
            TokenRange::Error(err) => Token::Error(err),
        }
    }
}

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

pub struct Tag<'a> {
    pub name: TagKind<'a>,
    attrs: AttrsHolder<'a>,
    pub self_closing: bool,
}

impl<'a> Tag<'a> {
    fn new(tag: TagRange, buf: &'a str) -> Self {
        let name = &buf[tag.name];
        let name = match HtmlTag::lookup(name) {
            Some(htmltag) => TagKind::Html(htmltag),
            None => TagKind::Other(name),
        };
        Tag {
            name,
            attrs: AttrsHolder::new(buf, tag.attrs),
            self_closing: tag.self_closing,
        }
    }

    pub fn is_html_tag(&self) -> bool {
        match self.name {
            TagKind::Html(_) => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &'a str {
        match self.name {
            TagKind::Html(htmltag) => htmltag.name(),
            TagKind::Other(name) => name,
        }
    }

    pub fn attrs(&self) -> Attrs<'a, '_> {
        Attrs::new(self.attrs.buffer, &self.attrs.attrs)
    }
}

pub enum TagKind<'a> {
    Html(HtmlTag),
    Other(&'a str),
}

pub(crate) struct AttrsHolder<'a> {
    buffer: &'a str,
    attrs: Vec<AttrRange>,
}

impl<'a> AttrsHolder<'a> {
    pub(crate) fn new(buffer: &'a str, attrs: Vec<AttrRange>) -> Self {
        AttrsHolder { buffer, attrs }
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

impl<'a, 'b> Iterator for Attrs<'a, 'b> {
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

pub struct Text<'a> {
    pub data: &'a str,
}

impl<'a> Text<'a> {
    fn new(data: Range<usize>, buf: &'a str) -> Self {
        Text { data: &buf[data] }
    }
}

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
    Text(Range<usize>),
    Comment(Range<usize>),
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

#[derive(Debug)]
pub(crate) struct AttrRange {
    pub name: Range<usize>,
    pub value: Range<usize>,
    pub duplicate: bool,
}
