#[macro_use]
mod macros;

mod comment;
mod doctype;
mod eof;
mod foreign;
mod tags;
mod text;

use crate::localnames::LocalName;
use bee_htmltokenizer::token::*;
use bee_htmltokenizer::Error;
use bee_htmltokenizer::InitialState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Namespace {
    Html,
    MathMl,
    Svg,
}

#[derive(Clone, Debug)]
pub struct DomTreeBuildContext {
    reset_mode: InsertionMode,
    namespace: Namespace,
    local_name: LocalName,
    mathml_text_integration_point: bool,
    svg_integration_point: bool,
    svg_script: bool,
    html_integration_pont: bool,
    has_p_element_in_button_scope: bool,
    has_select_element_in_select_scope: bool,
}

impl Default for DomTreeBuildContext {
    fn default() -> Self {
        DomTreeBuildContext {
            reset_mode: mode!(InBody),
            namespace: Namespace::Html,
            local_name: LocalName::Unknown,
            mathml_text_integration_point: false,
            svg_integration_point: false,
            svg_script: false,
            html_integration_pont: false,
            has_p_element_in_button_scope: false,
            has_select_element_in_select_scope: false,
        }
    }
}

/// A trait used for building a DOM tree.
///
/// The instance implementing this trait needs to implement some kind of stack
/// machine that supports the following operations
pub trait DomTreeBuilder {
    /// Enable the foster parenting.
    ///
    /// Initially, the foster parenting is disabled.
    fn enable_foster_parenting(&mut self);

    /// Disable the foster parenting.
    fn disable_foster_parenting(&mut self);

    /// Creates a node for a doctype and append it as a child node.
    fn append_doctype(&mut self, doctype: &Doctype<'_>);

    /// Creates a node for a tag as a child node of the current node
    /// and push it onto the stack.
    fn push_element(&mut self, name: &str, namespace: Namespace, context: DomTreeBuildContext);

    fn set_attribute(&mut self, name: &str, value: &str);

    /// Reopen the "head" element which has already been closed.
    fn reopen_head_element(&mut self);

    /// Removes a node.
    fn remove_element(&mut self) -> DomTreeBuildContext;

    /// Pops a node from the stack.
    fn pop_element(&mut self) -> DomTreeBuildContext;

    /// Creates a node for a text and append it as a child node.
    fn append_text(&mut self, text: &str);

    /// Creates a node for a comment and append it as a child node.
    fn append_comment(&mut self, comment: &Comment<'_>);

    ///
    fn end(&mut self);
}

pub struct TreeBuilder<T> {
    inner: T,
    mode: InsertionMode,
    original_mode: Option<InsertionMode>,
    quirks_mode: QuirksMode,

    context: DomTreeBuildContext,
    text: String,

    iframe_srcdoc: bool,
    quirks_mode_changeable: bool,
    frameset_ok: bool,
    ignore_lf: bool,
}

pub enum Control {
    Reprocess,
    Continue,
    SwitchTo(bee_htmltokenizer::InitialState, &'static str),
    ExecuteScript,
    Done,
}

const INITIAL_TEXT_CAPACITY: usize = 4096;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn new(inner: T) -> Self {
        TreeBuilder {
            inner,
            mode: mode!(Initial),
            original_mode: None,
            quirks_mode: QuirksMode::NoQuirks,
            context: Default::default(),
            text: String::with_capacity(INITIAL_TEXT_CAPACITY),
            iframe_srcdoc: false,
            quirks_mode_changeable: true,
            frameset_ok: true,
            ignore_lf: false,
        }
    }

    pub fn in_html_namespace(&self) -> bool {
        self.context.namespace == Namespace::Html
    }

    pub fn handle_token(&mut self, token: Token<'_>) -> Control {
        if self.is_in_foreign_content(&token) {
            self.handle_foreign(token)
        } else {
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
                Token::End => self.handle_eof(),
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn handle_error(&mut self, _error: Error) -> Control {
        // Ignore the error.
        Control::Continue
    }

    // helpers

    #[tracing::instrument(level = "debug", skip_all)]
    fn switch_to(&mut self, mode: InsertionMode) {
        tracing::debug!(old_mode = ?self.mode, new_mode = ?mode);
        self.mode = mode;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn save_and_switch_to(&mut self, mode: InsertionMode) {
        self.original_mode = Some(self.mode);
        self.switch_to(mode);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn switch_to_original_mode(&mut self) {
        let mode = self.original_mode.take().unwrap();
        self.switch_to(mode);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn enable_foster_parenting(&mut self) {
        self.inner.enable_foster_parenting();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn disable_foster_parenting(&mut self) {
        self.inner.disable_foster_parenting();
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
        self.inner.append_doctype(doctype);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_html_element(&mut self, tag: &Tag<'_>) {
        self.append_text_if_exists();
        self.inner
            .push_element(tag.name, Namespace::Html, self.context.clone());
        for (name, value) in tag.attrs() {
            self.inner.set_attribute(name, value);
        }
        self.context.namespace = Namespace::Html;
        self.context.local_name = LocalName::lookup(tag.name);
        self.context.mathml_text_integration_point = false;
        self.context.svg_integration_point = false;
        self.context.svg_script = false;
        self.context.html_integration_pont = false;
        match self.context.local_name {
            tag!(Applet) => {
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(Body) => {
                self.context.reset_mode = mode!(InBody);
            }
            tag!(Caption) => {
                self.context.reset_mode = mode!(InCaption);
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(Colgroup) => {
                self.context.reset_mode = mode!(InColumnGroup);
            }
            tag!(Select) => {
                // TODO
                self.context.reset_mode = mode!(InSelect);
                self.context.has_select_element_in_select_scope = true;
            }
            tag!(Table) => {
                self.context.reset_mode = mode!(InTable);
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(Td, Th) => {
                self.context.reset_mode = mode!(InCell);
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(Template) => {
                // TODO: switch the insertion mode to the current template insertion mode
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(Tr) => {
                self.context.reset_mode = mode!(InRow);
            }
            tag!(Tbody, Thead, Tfoot) => {
                self.context.reset_mode = mode!(InTableBody);
            }
            tag!(Marquee) => {
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(Head) => {
                self.context.reset_mode = mode!(InHead);
            }
            tag!(Optgroup) => {
                self.context.has_select_element_in_select_scope = false;
            }
            tag!(Option) => {
                self.context.has_select_element_in_select_scope = false;
            }
            tag!(P) => {
                self.context.has_p_element_in_button_scope = true;
            }
            tag!(Frameset) => {
                self.context.reset_mode = mode!(InFrameset);
            }
            tag!(Html) => {
                self.context.reset_mode = mode!(AfterHead);
                self.context.has_p_element_in_button_scope = false;
            }
            _ => {}
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_mathml_element(&mut self, tag: &Tag<'_>) {
        self.append_text_if_exists();
        self.inner
            .push_element(tag.name, Namespace::MathMl, self.context.clone());
        for (name, value) in tag.attrs() {
            // TODO: adjust MathML attributes
            // TODO: adjust foreign attributes
            self.inner.set_attribute(name, value);
        }
        self.context.namespace = Namespace::MathMl;
        self.context.local_name = LocalName::lookup(tag.name);
        match self.context.local_name {
            tag!(mathml: Mi, Mo, Mn, Ms, Mtext) => {
                self.context.mathml_text_integration_point = true;
                self.context.svg_integration_point = false;
                self.context.svg_script = false;
                self.context.html_integration_pont = false;
                self.context.has_p_element_in_button_scope = false;
            }
            tag!(mathml: AnnotationXml) => {
                self.context.mathml_text_integration_point = false;
                self.context.svg_integration_point = true;
                self.context.svg_script = false;
                self.context.html_integration_pont = false;
            }
            _ => {
                self.context.mathml_text_integration_point = false;
                self.context.svg_integration_point = false;
                self.context.svg_script = false;
                self.context.html_integration_pont = false;
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_svg_element(&mut self, tag: &Tag<'_>, local_name: LocalName) {
        self.append_text_if_exists();
        let tag_name = match local_name {
            LocalName::Unknown => tag.name,
            _ => local_name.name(),
        };
        self.inner
            .push_element(tag_name, Namespace::Svg, self.context.clone());
        for (name, value) in tag.attrs() {
            // TODO: adjust foreign attributes
            self.inner.set_attribute(name, value);
        }
        self.context.namespace = Namespace::Svg;
        self.context.local_name = LocalName::lookup(tag.name);
        self.context.mathml_text_integration_point = false;
        self.context.svg_integration_point = false;
        match self.context.local_name {
            tag!(svg: Script) => {
                self.context.svg_script = true;
                self.context.html_integration_pont = false;
            }
            tag!(svg: ForeignObject, Desc, Title) => {
                self.context.svg_script = false;
                self.context.html_integration_pont = true;
                self.context.has_p_element_in_button_scope = false;
            }
            _ => {
                self.context.svg_script = false;
                self.context.html_integration_pont = false;
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn reopen_head_element(&mut self) {
        self.append_text_if_exists();
        self.reopen_head_element();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_p_element(&mut self) {
        const NAMES: &[LocalName] = &tags![Dd, Dt, Li, Optgroup, Option, Rb, Rp, Rt, Rtc];
        self.close_elements(NAMES);
        if self.context.local_name != LocalName::P {
            // TODO: Parse error.
        }
        while self.context.local_name != LocalName::P {
            self.pop_element();
        }
        self.pop_element(); // pop a <p>
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_elements(&mut self, names: &[LocalName]) {
        while names.contains(&self.context.local_name) {
            self.pop_element();
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn remove_element(&mut self) {
        self.append_text_if_exists();
        self.context = self.inner.remove_element();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn pop_element(&mut self) {
        self.append_text_if_exists();
        self.context = self.inner.pop_element();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_char(&mut self, c: char) {
        self.text.push(c);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_comment(&mut self, comment: &Comment<'_>) {
        self.append_text_if_exists();
        self.inner.append_comment(comment);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn end(&mut self) {
        self.append_text_if_exists();
        self.inner.end();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_text_if_exists(&mut self) {
        if !self.text.is_empty() {
            self.inner.append_text(self.text.as_str());
            self.text.clear();
        }
    }

    fn insert_text_to_foster_parent(&mut self) {
        debug_assert!(!self.text.is_empty());
        self.enable_foster_parenting();
        self.append_text_if_exists();
        self.disable_foster_parenting();
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
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum QuirksMode {
    NoQuirks,
    Quirks,
    LimitedQuirks,
}
