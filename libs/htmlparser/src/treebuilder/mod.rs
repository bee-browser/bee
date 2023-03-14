#[macro_use]
mod macros;

mod comment;
mod doctype;
mod eof;
mod foreign;
mod tags;
mod text;

use bee_htmltokenizer::token::*;
use bee_htmltokenizer::Error;
use bee_htmltokenizer::InitialState;

use crate::localnames::LocalName;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Namespace {
    Html,
    MathMl,
    Svg,
}

flagset::flags! {
    enum DomTreeBuildContextFlags: u64 {
        MathmlTextIntegrationPoint,
        SvgIntegrationPoint,
        SvgScript,
        HtmlIntegrationPoint,
        HasTemplateElement,
        HasDivElementInScope,
        HasFormElementInScope,
        HasPreElementInScope,
        HasPElementInButtonScope,
        HasSelectElementInSelectScope,
        HasTableElementInTableScope,
        HasCaptionElementInTableScope,
        HasTbodyElementInTableScope,
        HasTfootElementInTableScope,
        HasTheadElementInTableScope,
        HasTrElementInTableScope,
        HasTdElementInTableScope,
        HasThElementInTableScope,
    }
}

#[derive(Clone, Debug)]
pub struct DomTreeBuildContext {
    reset_mode: InsertionMode,
    namespace: Namespace,
    local_name: LocalName,
    flags: flagset::FlagSet<DomTreeBuildContextFlags>,
}

impl DomTreeBuildContext {
    #[inline(always)]
    fn is_mathml_text_integration_point(&self) -> bool {
        self.flags.contains(flags!(MathmlTextIntegrationPoint))
    }

    #[inline(always)]
    fn is_svg_integration_point(&self) -> bool {
        self.flags.contains(flags!(SvgIntegrationPoint))
    }

    #[inline(always)]
    fn is_svg_script(&self) -> bool {
        self.flags.contains(flags!(SvgScript))
    }

    #[inline(always)]
    fn is_html_integration_point(&self) -> bool {
        self.flags.contains(flags!(HtmlIntegrationPoint))
    }

    #[inline(always)]
    fn has_template_element(&self) -> bool {
        self.flags.contains(flags!(HasTemplateElement))
    }

    #[inline(always)]
    fn has_div_element_in_scope(&self) -> bool {
        self.flags.contains(flags!(HasDivElementInScope))
    }

    #[inline(always)]
    fn has_form_element_in_scope(&self) -> bool {
        self.flags.contains(flags!(HasFormElementInScope))
    }

    #[inline(always)]
    fn has_pre_element_in_scope(&self) -> bool {
        self.flags.contains(flags!(HasPreElementInScope))
    }

    #[inline(always)]
    fn has_p_element_in_button_scope(&self) -> bool {
        self.flags.contains(flags!(HasPElementInButtonScope))
    }

    #[inline(always)]
    fn has_select_element_in_select_scope(&self) -> bool {
        self.flags.contains(flags!(HasSelectElementInSelectScope))
    }

    #[inline(always)]
    fn has_table_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasTableElementInTableScope))
    }

    #[inline(always)]
    fn has_caption_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasCaptionElementInTableScope))
    }

    #[inline(always)]
    fn has_rowgroup_element_in_table_scope(&self) -> bool {
        !self.flags.is_disjoint(flags!(
            HasTbodyElementInTableScope,
            HasTfootElementInTableScope,
            HasTheadElementInTableScope
        ))
    }

    #[inline(always)]
    fn has_tbody_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasTbodyElementInTableScope))
    }

    #[inline(always)]
    fn has_tfoot_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasTfootElementInTableScope))
    }

    #[inline(always)]
    fn has_thead_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasTheadElementInTableScope))
    }

    #[inline(always)]
    fn has_tr_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasTrElementInTableScope))
    }

    #[inline(always)]
    fn has_cell_element_in_table_scope(&self) -> bool {
        !self
            .flags
            .is_disjoint(flags!(HasTdElementInTableScope, HasThElementInTableScope))
    }

    #[inline(always)]
    fn has_td_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasTdElementInTableScope))
    }

    #[inline(always)]
    fn has_th_element_in_table_scope(&self) -> bool {
        self.flags.contains(flags!(HasThElementInTableScope))
    }
}

impl Default for DomTreeBuildContext {
    fn default() -> Self {
        DomTreeBuildContext {
            reset_mode: mode!(InBody),
            namespace: Namespace::Html,
            local_name: LocalName::Unknown,
            flags: Default::default(),
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

    //
    fn push_marker_to_active_formatting_element_list(&mut self);

    //
    fn push_element_to_active_formatting_element_list(&mut self);

    //
    fn reconstruct_active_formatting_elements(&mut self);

    //
    fn pop_active_formatting_elements_up_to_marker(&mut self);

    //
    fn run_adoption_agency_algorithm(&mut self, tag: &Tag<'_>);

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
    fn push_marker_to_active_formatting_element_list(&mut self) {
        self.inner.push_marker_to_active_formatting_element_list();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_element_to_active_formatting_element_list(&mut self) {
        self.inner.push_element_to_active_formatting_element_list();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn reconstruct_active_formatting_elements(&mut self) {
        self.inner.reconstruct_active_formatting_elements();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn pop_active_formatting_elements_up_to_marker(&mut self) {
        self.inner.pop_active_formatting_elements_up_to_marker();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn run_adoption_agency_algorithm(&mut self, tag: &Tag<'_>) {
        self.inner.run_adoption_agency_algorithm(tag);
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

    #[inline(always)]
    fn push_html_applet_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.flags -= flags!(HasPElementInButtonScope);
    }

    #[inline(always)]
    fn push_html_b_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_body_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InBody);
    }

    #[inline(always)]
    fn push_html_caption_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InCaption);
        self.context.flags -= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope
        );
        self.context.flags |= flags!(HasCaptionElementInTableScope);
    }

    #[inline(always)]
    fn push_html_colgroup_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InColumnGroup);
    }

    #[inline(always)]
    fn push_html_div_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_form_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_frameset_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InFrameset);
    }

    #[inline(always)]
    fn push_html_head_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InHead);
    }

    #[inline(always)]
    fn push_html_html_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(AfterHead);
        self.context.flags -= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope,
            HasTableElementInTableScope,
            HasCaptionElementInTableScope,
            HasTbodyElementInTableScope,
            HasTfootElementInTableScope,
            HasTheadElementInTableScope,
            HasTrElementInTableScope,
            HasTdElementInTableScope,
            HasThElementInTableScope
        );
    }

    #[inline(always)]
    fn push_html_i_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_input_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_marquee_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.flags |= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope
        );
    }

    #[inline(always)]
    fn push_html_optgroup_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.flags -= flags!(HasSelectElementInSelectScope);
    }

    #[inline(always)]
    fn push_html_option_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.flags -= flags!(HasSelectElementInSelectScope);
    }

    #[inline(always)]
    fn push_html_p_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.flags |= flags!(HasPElementInButtonScope);
    }

    #[inline(always)]
    fn push_html_plaintext_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_pre_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.flags |= flags!(HasPreElementInScope);
    }

    #[inline(always)]
    fn push_html_script_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_select_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        // TODO
        self.context.reset_mode = mode!(InSelect);
        self.context.flags |= flags!(HasSelectElementInSelectScope);
    }

    #[inline(always)]
    fn push_html_style_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_table_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InTable);
        self.context.flags -= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope,
            HasCaptionElementInTableScope,
            HasTbodyElementInTableScope,
            HasTfootElementInTableScope,
            HasTheadElementInTableScope,
            HasTrElementInTableScope,
            HasTdElementInTableScope,
            HasThElementInTableScope
        );
        self.context.flags |= flags!(HasTableElementInTableScope);
    }

    #[inline(always)]
    fn push_html_tbody_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InTableBody);
        self.context.flags |= flags!(HasTbodyElementInTableScope);
    }

    #[inline(always)]
    fn push_html_td_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InCell);
        self.context.flags -= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope
        );
        self.context.flags |= flags!(HasTdElementInTableScope);
    }

    #[inline(always)]
    fn push_html_template_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        // TODO: switch the insertion mode to the current template insertion mode
        self.context.flags -= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope,
            HasTableElementInTableScope,
            HasCaptionElementInTableScope,
            HasTbodyElementInTableScope,
            HasTfootElementInTableScope,
            HasTheadElementInTableScope,
            HasTrElementInTableScope,
            HasTdElementInTableScope,
            HasThElementInTableScope
        );
        self.context.flags |= flags!(HasTemplateElement);
    }

    #[inline(always)]
    fn push_html_textarea_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_tfoot_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InTableBody);
        self.context.flags |= flags!(HasTfootElementInTableScope);
    }

    #[inline(always)]
    fn push_html_th_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InCell);
        self.context.flags -= flags!(
            HasDivElementInScope,
            HasFormElementInScope,
            HasPreElementInScope,
            HasPElementInButtonScope
        );
        self.context.flags |= flags!(HasThElementInTableScope);
    }

    #[inline(always)]
    fn push_html_thead_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InTableBody);
        self.context.flags |= flags!(HasTheadElementInTableScope);
    }

    #[inline(always)]
    fn push_html_title_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
    }

    #[inline(always)]
    fn push_html_tr_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
        self.context.reset_mode = mode!(InRow);
        self.context.flags |= flags!(HasTrElementInTableScope);
    }

    #[inline(always)]
    fn push_html__element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag);
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
        self.context.flags -= flags!(
            MathmlTextIntegrationPoint,
            SvgIntegrationPoint,
            SvgScript,
            HtmlIntegrationPoint
        );
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
                self.context.flags |= flags!(MathmlTextIntegrationPoint);
                self.context.flags -= flags!(
                    SvgIntegrationPoint,
                    SvgScript,
                    HtmlIntegrationPoint,
                    HasPElementInButtonScope
                );
            }
            tag!(mathml: AnnotationXml) => {
                self.context.flags |= flags!(SvgIntegrationPoint);
                self.context.flags -=
                    flags!(MathmlTextIntegrationPoint, SvgScript, HtmlIntegrationPoint);
            }
            _ => {
                self.context.flags -= flags!(
                    MathmlTextIntegrationPoint,
                    SvgIntegrationPoint,
                    SvgScript,
                    HtmlIntegrationPoint
                );
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
        self.context.flags -= flags!(MathmlTextIntegrationPoint, SvgIntegrationPoint);
        match self.context.local_name {
            tag!(svg: Script) => {
                self.context.flags |= flags!(SvgScript);
                self.context.flags -= flags!(HtmlIntegrationPoint);
            }
            tag!(svg: ForeignObject, Desc, Title) => {
                self.context.flags |= flags!(HtmlIntegrationPoint);
                self.context.flags -= flags!(SvgScript, HasPElementInButtonScope);
            }
            _ => {
                self.context.flags -= flags!(SvgScript, HtmlIntegrationPoint);
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn reopen_head_element(&mut self) {
        self.append_text_if_exists();
        self.reopen_head_element();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_implied_tags(&mut self) {
        // TODO
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_implied_tags_except_for(&mut self, local_name: LocalName) {
        // TODO
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_all_implied_tags(&mut self) {
        // TODO
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
