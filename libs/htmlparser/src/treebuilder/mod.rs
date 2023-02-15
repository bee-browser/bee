#[macro_use]
mod macros;

mod comment;
mod doctype;
mod end_tag;
mod start_tag;
mod text;

use bee_htmltokenizer::token::*;
use bee_htmltokenizer::Error;

/// A trait to operate on a Document object.
///
/// The instance implementing this trait needs to implement some kind of stack
/// machine that supports the following operations
pub trait DocumentWriter {
    /// Creates a node for a doctype and append it as a child node.
    fn append_doctype(&mut self, doctype: &Doctype<'_>);

    /// Creates a node for a tag as a child node of the current node
    /// and push it onto the stack.
    fn push_element(&mut self, tag: &Tag<'_>);

    /// Pops a node from the stack.
    fn pop(&mut self);

    /// Creates a node for a text and append it as a child node.
    fn append_text(&mut self, text: &str);

    /// Creates a node for a comment and append it as a child node.
    fn append_comment(&mut self, comment: &Comment<'_>);

    ///
    fn end(&mut self);
}

pub struct TreeBuilder<W> {
    writer: W,
    insertion_mode: InsertionMode,
    original_insertion_mode: Option<InsertionMode>,
    text: String,
}

pub enum Control {
    Continue,
    SwitchTo(bee_htmltokenizer::InitialState),
    Done,
}

const INITIAL_TEXT_CAPACITY: usize = 4096;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    pub fn new(writer: W) -> Self {
        TreeBuilder {
            writer,
            insertion_mode: InsertionMode::Initial,
            original_insertion_mode: None,
            text: String::with_capacity(INITIAL_TEXT_CAPACITY),
        }
    }

    pub fn handle_token(&mut self, token: Token<'_>) -> Control {
        // Many implementation call the handler of each insertion mode, then
        // branch for each token type in each handle.  This is the same way
        // the HTML5 specification does.
        //
        // However, our implementation calls the handler of each token type
        // first, then branch for each insertion mode in each handler.
        //
        // The reasons are listed below:
        //
        // * The insertion mode may be changed while handling the same token,
        //   but the token type is never changed
        // * The token may be changed in an insertion mode and reused in other
        //   insertion modes
        match token {
            Token::Doctype(doctype) => self.handle_doctype(doctype),
            Token::StartTag(tag) => self.handle_start_tag(tag),
            Token::EndTag(tag) => self.handle_end_tag(tag),
            Token::Text(text) => self.handle_text(text),
            Token::Comment(comment) => self.handle_comment(comment),
            Token::Error(error) => self.handle_error(error),
            Token::End => self.handle_end(),
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn handle_error(&mut self, error: Error) -> Control {
        // TODO
        Control::Continue
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn handle_end(&mut self) -> Control {
        loop {
            tracing::debug!(?self.insertion_mode);
            match self.insertion_mode {
                InsertionMode::Initial => {
                    self.switch_to(InsertionMode::BeforeHtml);
                }
                InsertionMode::BeforeHtml => {
                    self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                    self.switch_to(InsertionMode::BeforeHead);
                }
                InsertionMode::BeforeHead => {
                    self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                    self.switch_to(InsertionMode::InHead);
                }
                InsertionMode::InHead => {
                    self.pop();
                    self.switch_to(InsertionMode::AfterHead);
                }
                InsertionMode::AfterHead => {
                    self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                    self.switch_to(InsertionMode::InBody);
                }
                InsertionMode::InBody => {
                    break;
                }
                InsertionMode::Text => {
                    // TODO: Parse error.
                    // TODO: If the current node is a script element, then set its already started to true.
                    self.pop();
                    self.switch_to_original_mode();
                }
                InsertionMode::AfterBody => {
                    // TODO: Parse error. Switch the insertion mode to "in body" and reprocess the token.
                    break;
                }
                _ => unimplemented!(),
            }
        }
        self.end();
        Control::Done
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn switch_to(&mut self, insertion_mode: InsertionMode) {
        self.insertion_mode = insertion_mode;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn save_and_switch_to(&mut self, insertion_mode: InsertionMode) {
        self.original_insertion_mode = Some(self.insertion_mode);
        self.insertion_mode = insertion_mode;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn switch_to_original_mode(&mut self) {
        self.insertion_mode = self.original_insertion_mode.take().unwrap();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_doctype(&mut self, doctype: &Doctype<'_>) {
        self.append_text_if_exists();
        self.writer.append_doctype(doctype);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_element(&mut self, tag: &Tag<'_>) {
        self.append_text_if_exists();
        self.writer.push_element(tag);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn pop(&mut self) {
        self.append_text_if_exists();
        self.writer.pop();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_char(&mut self, c: char) {
        self.text.push(c);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_comment(&mut self, comment: &Comment<'_>) {
        self.append_text_if_exists();
        self.writer.append_comment(comment);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn end(&mut self) {
        self.append_text_if_exists();
        self.writer.end();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_text_if_exists(&mut self) {
        if !self.text.is_empty() {
            self.writer.append_text(self.text.as_str());
            self.text.clear();
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
    InForeignContent,
}
