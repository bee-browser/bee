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

    /// Removes a node.
    fn remove_element(&mut self);

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
    mode: InsertionMode,
    original_mode: Option<InsertionMode>,
    quirks_mode: QuirksMode,
    text: String,

    iframe_srcdoc: bool,
    quirks_mode_changeable: bool,
    frameset_ok: bool,
}

pub enum Control {
    Reprocess,
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
            mode: mode!(Initial),
            original_mode: None,
            quirks_mode: QuirksMode::NoQuirks,
            text: String::with_capacity(INITIAL_TEXT_CAPACITY),
            iframe_srcdoc: false,
            quirks_mode_changeable: true,
            frameset_ok: true,
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
            tracing::debug!(?self.mode);
            match self.mode {
                mode!(Initial) => {
                    self.switch_to(mode!(BeforeHtml));
                }
                mode!(BeforeHtml) => {
                    self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                    self.switch_to(mode!(BeforeHead));
                }
                mode!(BeforeHead) => {
                    self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                    self.switch_to(mode!(InHead));
                }
                mode!(InHead) => {
                    self.pop();
                    self.switch_to(mode!(AfterHead));
                }
                mode!(AfterHead) => {
                    self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                    self.switch_to(mode!(InBody));
                }
                mode!(InBody) => {
                    break;
                }
                mode!(Text) => {
                    // TODO: Parse error.
                    // TODO: If the current node is a script element, then set its already started to true.
                    self.pop();
                    self.switch_to_original_mode();
                }
                mode!(AfterBody) => {
                    // TODO: Parse error. Switch the insertion mode to "in body" and reprocess the token.
                    break;
                }
                _ => unimplemented!(),
            }
        }
        self.end();
        Control::Done
    }

    // common rules

    fn handle_anything_else(&mut self) -> Control {
        match self.mode {
            mode!(Initial) => {
                if !self.iframe_srcdoc {
                    // TODO: parse error
                }
                self.change_quirks_mode_if_changeable(QuirksMode::Quirks);
                self.reprocess_on(mode!(BeforeHtml))
            }
            mode!(BeforeHtml) => {
                // TODO: Create an html element whose node document is the Document object.
                // TODO: Append it to the Document object.
                // TODO: Put this element in the stack of open elements.
                self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                self.switch_to(mode!(BeforeHead));
                Control::Reprocess
            }
            mode!(BeforeHead) => {
                // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                // TODO: Set the head element pointer to the newly created head element.
                self.switch_to(mode!(InHead));
                Control::Reprocess
            }
            mode!(InHead) => {
                // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                self.pop();
                self.switch_to(mode!(AfterHead));
                Control::Reprocess
            }
            mode!(InHeadNoscript) => {
                // TODO: Parse error.
                // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                self.switch_to(mode!(InHead));
                Control::Reprocess
            }
            mode!(AfterHead) => {
                // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                self.switch_to(mode!(InBody));
                Control::Reprocess
            }
            mode!(InColumnGroup) => {
                // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                // TODO: Otherwise, pop the current node from the stack of open elements.
                self.switch_to(mode!(InTable));
                Control::Reprocess
            }
            mode!(InSelect, InSelectInTable, InFrameset, AfterFrameset, AfterAfterFrameset) => {
                // TODO: Parse error.
                // Ignore the token.
                Control::Continue
            }
            mode!(AfterBody, AfterAfterBody) => {
                // TODO: Parse error.
                self.switch_to(mode!(InBody));
                Control::Reprocess
            }
            mode!(
                InBody,
                Text,
                InTable,
                InTableText,
                InCaption,
                InTableBody,
                InRow,
                InCell,
                InTemplate,
                InForeignContent
            ) => {
                unreachable!("{:?}", self.mode);
            }
        }
    }

    fn reprocess_on(&mut self, mode: InsertionMode) -> Control {
        self.switch_to(mode);
        Control::Reprocess
    }

    fn ignore_token(&mut self) -> Control {
        // TODO: Parse error.
        // Ignore the token.
        Control::Continue
    }

    // helpers

    #[tracing::instrument(level = "debug", skip_all)]
    fn switch_to(&mut self, mode: InsertionMode) {
        self.mode = mode;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn save_and_switch_to(&mut self, mode: InsertionMode) {
        self.original_mode = Some(self.mode);
        self.mode = mode;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn switch_to_original_mode(&mut self) {
        self.mode = self.original_mode.take().unwrap();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn change_quirks_mode_if_changeable(&mut self, quirks_mode: QuirksMode) {
        if self.quirks_mode_changeable {
            tracing::debug!(
                old_quirks_mode = ?self.quirks_mode,
                new_quirks_mode = ?quirks_mode
            );
            self.quirks_mode = quirks_mode;
        }
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
    fn remove_element(&mut self) {
        self.append_text_if_exists();
        self.writer.remove_element();
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

#[derive(Debug, Clone, Copy)]
enum QuirksMode {
    NoQuirks,
    Quirks,
    LimitedQuirks,
}
