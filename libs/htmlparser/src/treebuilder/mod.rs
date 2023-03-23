#[macro_use]
mod macros;

mod comment;
mod doctype;
mod eof;
mod foreign;
mod tags;
mod text;

use std::fmt::Debug;

use bee_htmltokenizer::token::*;
use bee_htmltokenizer::Error;
use bee_htmltokenizer::InitialState;

use crate::localnames::LocalName;

/// A trait used for building a DOM tree.
///
/// The instance implementing this trait needs to implement some kind of stack
/// machine that supports the following operations
pub trait DomTreeBuilder {
    type Node: Clone + Copy + Debug + Eq + PartialEq;

    /// Gets the document node.
    fn get_document(&mut self) -> Self::Node;

    /// Gets the root node to which created nodes will be added.
    ///
    /// It's a document node when parsing an HTML document.  When parsing an
    /// HTML fragment, it's a root node (normally, an 'html' element node).
    fn get_root(&mut self) -> Self::Node;

    /// Creates a DocumentType node.
    fn create_doctype(&mut self, doctype: &Doctype<'_>) -> Self::Node;

    /// Creates an Element node.
    fn create_element(&mut self, name: &str, ns: Namespace) -> Self::Node;

    /// Create a Text node.
    fn create_text(&mut self, data: &str) -> Self::Node;

    /// Create a Comment node.
    fn create_comment(&mut self, data: &str) -> Self::Node;

    /// Sets an attribute to a node.
    fn set_attribute<'a, I>(&mut self, node: Self::Node, attrs: I, overwrite: bool)
    where
        I: Iterator<Item = (&'a str, &'a str)>;

    /// Clones a node.
    fn clone_node(&mut self, node: Self::Node) -> Self::Node;

    /// Appends a node as a last child node of a parent node.
    fn append_child(&mut self, parent: Self::Node, node: Self::Node);

    /// Inserts a node before a sibling node into the child node list of a parent node.
    fn insert_before(&mut self, parent: Self::Node, node: Self::Node, sibling: Self::Node);

    /// Removes a node from the child node list of a parent node.
    fn remove_child(&mut self, parent: Self::Node, node: Self::Node);

    /// Moves all child nodes of a node to a new parent node.
    fn move_child_nodes(&mut self, node: Self::Node, new_parent: Self::Node);

    ///
    fn end(&mut self);

    ///
    fn print_tree(&self);

    ///
    fn has_same_name(&mut self, node: Self::Node, name: &str) -> bool;
}

pub struct TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    inner: T,
    mode: InsertionMode,
    original_mode: Option<InsertionMode>,
    quirks_mode: QuirksMode,

    html_element: Option<T::Node>,
    head_element: Option<T::Node>,
    body_element: Option<T::Node>,
    text: String,
    pending_table_text: String,

    context_stack: Vec<TreeBuildContext<T::Node>>,
    active_formatting_element_list: ActiveFormattingElementList<T::Node>,

    iframe_srcdoc: bool,
    quirks_mode_changeable: bool,
    frameset_ok: bool,
    ignore_lf: bool,
    foster_parenting: bool,
    scripting: bool,
    pending_table_text_contains_non_whitespace: bool,
}

#[derive(Debug)]
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
    pub fn new(mut inner: T) -> Self {
        let open_element = OpenElement::with_html(LocalName::Unknown, inner.get_root());
        let context = TreeBuildContext::new(open_element);
        TreeBuilder {
            inner,
            mode: mode!(Initial),
            original_mode: None,
            quirks_mode: QuirksMode::NoQuirks,
            html_element: None,
            head_element: None,
            body_element: None,
            text: String::with_capacity(INITIAL_TEXT_CAPACITY),
            pending_table_text: String::with_capacity(INITIAL_TEXT_CAPACITY),
            context_stack: vec![context],
            active_formatting_element_list: Default::default(),
            iframe_srcdoc: false,
            quirks_mode_changeable: true,
            frameset_ok: true,
            ignore_lf: false,
            foster_parenting: false,
            scripting: false,
            pending_table_text_contains_non_whitespace: false,
        }
    }

    pub fn set_quirks_mode(&mut self, quirks_mode: QuirksMode) {
        self.quirks_mode = quirks_mode;
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn set_scripting(&mut self, scripting: bool) {
        self.scripting = scripting;
        tracing::debug!(scripting);
    }

    pub fn set_context_element(
        &mut self,
        local_name: LocalName,
        namespace: Namespace,
        node: T::Node,
    ) {
        debug_assert_eq!(self.context_stack.len(), 1);
        self.context_stack[0] = TreeBuildContext {
            open_element: OpenElement {
                namespace,
                local_name,
                node,
            },
            reset_mode: InsertionMode::InBody,
            foster_parenting_insertion_point: FosterParentingInsertionPoint::LastChild(node),
            element_in_scope: Default::default(),
            element_in_list_item_scope: Default::default(),
            element_in_button_scope: Default::default(),
            element_in_table_scope: Default::default(),
            element_in_select_scope: Default::default(),
            // TODO
            flags: Default::default(),
        };
        self.switch_to(match local_name {
            tag!(Select) => mode!(InSelect),
            tag!(Td, Th) => mode!(InCell),
            tag!(Tr) => mode!(InRow),
            tag!(Tbody, Thead, Tfoot) => mode!(InTableBody),
            tag!(Caption) => mode!(InCaption),
            tag!(Colgroup) => mode!(InColumnGroup),
            tag!(Table) => mode!(InTable),
            tag!(Head) => mode!(InHead),
            tag!(Body) => mode!(InBody),
            tag!(Frameset) => mode!(InFrameset),
            tag!(Html) => mode!(BeforeHead),
            _ => mode!(InBody),
        });
        tracing::debug!(
            context.pos = self.context_stack.len() - 1,
            context.element = ?self.context().open_element,
        );
        // TODO: Set the parser's form element pointer to the nearest node to the context element that is a form element (going straight up the ancestor chain, and including the element itself, if it is a form element), if any. (If there is no such form element, the form element pointer keeps its initial value, null.)
    }

    pub fn in_html_namespace(&self) -> bool {
        self.context().is_html()
    }

    #[tracing::instrument(level = "debug", skip(self))]
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
        self.foster_parenting = true;
        tracing::debug!(foster_parenting = true);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn disable_foster_parenting(&mut self) {
        self.foster_parenting = false;
        tracing::debug!(foster_parenting = false);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_marker_to_active_formatting_contexts(&mut self) {
        tracing::debug!(marker = ?self.context().open_element);
        self.active_formatting_element_list.push_marker();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_element_to_active_formatting_contexts(&mut self) {
        let open_element = &self.context().open_element;
        tracing::debug!(element = ?open_element);
        self.active_formatting_element_list
            .push_element(open_element.local_name, open_element.node);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn reconstruct_active_formatting_elements(&mut self) {
        if self.active_formatting_element_list.is_empty() {
            return;
        }
        let mut i = self.active_formatting_element_list.len() - 1; // last
        match self.active_formatting_element_list.get(i) {
            ActiveFormattingContext::Marker => return,
            ActiveFormattingContext::Element { ref node, .. } => {
                if self.find_element_in_stack(*node).is_some() {
                    return;
                }
            }
            ActiveFormattingContext::Removed => unreachable!(),
        }
        while i > 0 {
            i -= 1;
            match self.active_formatting_element_list.get(i) {
                ActiveFormattingContext::Marker => {
                    i += 1;
                    break;
                }
                ActiveFormattingContext::Element { ref node, .. } => {
                    if self.find_element_in_stack(*node).is_some() {
                        i += 1;
                        break;
                    }
                }
                ActiveFormattingContext::Removed => unreachable!(),
            }
        }
        while i < self.active_formatting_element_list.len() {
            let node = self.active_formatting_element_list.get_element(i);
            let local_name = self.active_formatting_element_list.get_local_name(i);
            let new_node = self.inner.clone_node(node);
            self.insert_html_element(OpenElement::with_html(local_name, new_node));
            self.active_formatting_element_list.set_element(i, new_node);
            i += 1;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn pop_active_formatting_elements_up_to_marker(&mut self) {
        self.active_formatting_element_list
            .clear_up_to_last_marker();
    }

    // A naive implementation of the adoption agency algorithm.
    #[tracing::instrument(level = "debug", skip_all)]
    fn perform_adoption_agency_algorithm(&mut self, tag: &Tag<'_>) {
        self.append_text_if_exists();

        self.inner.print_tree();
        tracing::debug!(?self.active_formatting_element_list);

        const MAX_OUTER_ITERATIONS: usize = 8;
        const MAX_INNER_ITERATIONS: usize = 3;

        let subject = LocalName::lookup(tag.name);
        tracing::debug!(?subject, "step#1");

        // step#2
        let context = self.context();
        let step2_cond = context.is_html()
            && context.open_element.local_name == subject
            && self
                .active_formatting_element_list
                .find_element(context.open_element.node)
                .is_none();
        tracing::debug!(
            step2_cond,
            context.pos = self.context_stack.len() - 1,
            context.element = ?context.open_element,
            "step#2"
        );
        if step2_cond {
            self.pop_element();
            return;
        }

        for outer_loop_counter in 0..MAX_OUTER_ITERATIONS {
            // step#4.3
            let list_pos = match self
                .active_formatting_element_list
                .find_last_element(subject)
            {
                Some(i) => i,
                None => {
                    let _ = self.handle_end_any_other(tag);
                    break;
                }
            };
            let element = self.active_formatting_element_list.get_element(list_pos);
            tracing::debug!(
                outer_loop_counter,
                element.list.pos = list_pos,
                element.list.node = ?element,
                "step#4.3"
            );

            let stack_pos = match self.find_element_in_scope(element) {
                Err(false) => {
                    // step#4.4
                    // TODO: Parse error.
                    self.active_formatting_element_list.remove(list_pos);
                    tracing::debug!(outer_loop_counter, list_pos, "step#4.4");
                    break;
                }
                Err(true) => {
                    // step#4.5
                    // TODO: Parse error.
                    tracing::debug!(outer_loop_counter, "step#4.5");
                    break;
                }
                Ok(pos) => {
                    // step#4.6
                    tracing::debug!(
                        outer_loop_counter,
                        element.context.pos = pos,
                        element.context.element = ?self.context_stack[pos].open_element,
                        "step#4.6"
                    );
                    if self.context().open_element.node != element {
                        // TODO: Parse error.
                    }
                    pos
                }
            };

            // step#4.7
            let furthest_block_pos = match self.find_furthest_block(stack_pos) {
                Some(pos) => pos,
                None => {
                    //  step#4.8
                    tracing::debug!(outer_loop_counter, "step#4.8");
                    while self.context_stack.len() > stack_pos {
                        self.pop_element();
                    }
                    self.active_formatting_element_list.remove(list_pos);
                    break;
                }
            };
            let furthest_block = self.context_stack[furthest_block_pos].open_element.node;
            tracing::debug!(
                outer_loop_counter,
                furthest_block.context.pos = furthest_block_pos,
                furthest_block.context.element = ?self.context_stack[furthest_block_pos].open_element,
                "step#4.7"
            );

            // The furthest block will be always reparented.
            let furthest_block_parent =
                self.context_stack[furthest_block_pos - 1].open_element.node;
            self.inner
                .remove_child(furthest_block_parent, furthest_block);

            // step#4.9
            let common_ancestor_stack_pos = stack_pos - 1;
            tracing::debug!(
                outer_loop_counter,
                common_ancestor.context.pos = common_ancestor_stack_pos,
                common_ancestor.context.element = ?self.context_stack[common_ancestor_stack_pos].open_element,
                "step#4.9"
            );

            // step#4.10
            let mut bookmark = list_pos;
            tracing::debug!(outer_loop_counter, bookmark, "step#4.10");

            // step#4.11
            let mut node_stack_pos = furthest_block_pos;
            let mut last_node = furthest_block;
            tracing::debug!(
                outer_loop_counter,
                node.context.pos = node_stack_pos,
                node.context.element = ?self.context_stack[node_stack_pos].open_element,
                ?last_node,
                "step#4.11"
            );

            // step#4.12
            let mut inner_loop_counter = 0;

            loop {
                // step#4.13.1
                inner_loop_counter += 1;
                tracing::debug!(outer_loop_counter, inner_loop_counter, "step#4.13.1");

                // step#4.13.2
                node_stack_pos -= 1;
                let node = self.context_stack[node_stack_pos].open_element.node;
                tracing::debug!(
                    outer_loop_counter,
                    inner_loop_counter,
                    node.context.pos = node_stack_pos,
                    node.context.element = ?self.context_stack[node_stack_pos].open_element,
                    "step#4.13.2"
                );

                // step#4.13.3
                if node_stack_pos == stack_pos {
                    tracing::debug!(outer_loop_counter, inner_loop_counter, "step#4.13.3");
                    break;
                }

                // step#4.13.4
                let mut node_list_pos = self.active_formatting_element_list.find_element(node);
                tracing::debug!(
                    outer_loop_counter,
                    inner_loop_counter,
                    node.context.pos = node_stack_pos,
                    node.context.element = ?self.context_stack[node_stack_pos].open_element,
                    node.list.pos = ?node_list_pos,
                    "step#4.13.4"
                );
                if let Some(pos) = node_list_pos {
                    if inner_loop_counter > MAX_INNER_ITERATIONS {
                        self.active_formatting_element_list.remove(pos);
                        node_list_pos = None;
                    }
                }

                // step#4.13.5
                let node_list_pos = match node_list_pos {
                    Some(pos) => pos,
                    None => {
                        tracing::debug!(outer_loop_counter, inner_loop_counter, "step#4.13.5");
                        self.context_stack[node_stack_pos].flags |= flags!(Removed);
                        continue;
                    }
                };

                // step#4.13.6
                //
                // The HTML5 specification requires to keep the original token
                // for which the element node was created.  But we simply clone
                // the element node.
                let cloned = self.inner.clone_node(node);
                self.active_formatting_element_list
                    .set_element(node_list_pos, cloned);
                self.context_stack[node_stack_pos].open_element.node = cloned;
                let node = cloned;
                tracing::debug!(
                    outer_loop_counter,
                    inner_loop_counter,
                    node.context.pos = node_stack_pos,
                    node.context.element = ?self.context_stack[node_stack_pos].open_element,
                    node.list.pos = node_list_pos,
                    "step#4.13.6"
                );

                // step#4.13.7
                if last_node == furthest_block {
                    bookmark = node_list_pos + 1;
                    tracing::debug!(
                        outer_loop_counter,
                        inner_loop_counter,
                        bookmark,
                        "step#4.13.7"
                    );
                }

                // step#4.13.8
                tracing::debug!(
                    outer_loop_counter,
                    inner_loop_counter,
                    node.context.pos = node_stack_pos,
                    node.context.element = ?self.context_stack[node_stack_pos].open_element,
                    ?last_node,
                    "step#4.13.8"
                );
                self.inner.append_child(node, last_node);

                // step#4.13.9
                last_node = node;
                tracing::debug!(
                    outer_loop_counter,
                    inner_loop_counter,
                    ?last_node,
                    "step#4.13.9"
                );
            }

            // step#4.14
            tracing::debug!(
                outer_loop_counter,
                ?last_node,
                common_ancestor.context.pos = common_ancestor_stack_pos,
                common_ancestor.context.element = ?self.context_stack[common_ancestor_stack_pos].open_element,
                "step#4.14"
            );
            self.insert_node_with_context(last_node, common_ancestor_stack_pos);

            // step#4.15
            let new_element = self.inner.clone_node(element);
            tracing::debug!(outer_loop_counter, ?element, ?new_element, "step#4.15");

            // step#4.16
            tracing::debug!(
                outer_loop_counter,
                furthest_block.context.pos = furthest_block_pos,
                furthest_block.context.element = ?self.context_stack[furthest_block_pos].open_element,
                ?new_element,
                "step#4.16"
            );
            self.inner.move_child_nodes(furthest_block, new_element);

            // step#4.17
            tracing::debug!(
                outer_loop_counter,
                furthest_block.context.pos = furthest_block_pos,
                furthest_block.context.element = ?self.context_stack[furthest_block_pos].open_element,
                ?new_element,
                "step#4.17"
            );
            self.insert_node_with_context(new_element, furthest_block_pos);

            // step#4.18
            tracing::debug!(
                outer_loop_counter,
                element.list.pos = list_pos,
                bookmark,
                "step#4.18"
            );
            self.active_formatting_element_list.remove(list_pos);
            self.active_formatting_element_list
                .insert_element(bookmark, subject, new_element);

            // step#4.19
            tracing::debug!(outer_loop_counter, stack_pos, "step#4.19");
            let mut context = self.context_stack[stack_pos].clone();
            context.open_element.node = new_element;
            context.reset_mode = self.context_stack[furthest_block_pos].reset_mode;
            context.foster_parenting_insertion_point =
                self.context_stack[furthest_block_pos].foster_parenting_insertion_point;
            context.flags = self.context_stack[furthest_block_pos].flags;
            self.context_stack[stack_pos].flags |= flags!(Removed);
            self.context_stack.insert(furthest_block_pos + 1, context);

            self.inner.print_tree();
        }

        self.context_stack
            .retain(|context| !context.flags.contains(flags!(Removed)));
        self.active_formatting_element_list.clean();
    }

    fn find_element_in_stack(&self, element: T::Node) -> Option<usize> {
        self.context_stack
            .iter()
            .rposition(|context| !context.is_removed() && context.open_element.node == element)
    }

    fn find_element_in_scope(&self, element: T::Node) -> Result<usize, bool> {
        for (i, context) in self.context_stack.iter().enumerate().rev() {
            if context.is_removed() {
                continue;
            }
            if context.open_element.node == element {
                return Ok(i);
            }
            match context.open_element.namespace {
                Namespace::Html => match context.open_element.local_name {
                    tag!(Applet, Caption, Html, Table, Td, Th, Marquee, Object, Template) => {
                        return Err(true)
                    }
                    _ => (),
                },
                Namespace::MathMl => match context.open_element.local_name {
                    tag!(Mi, Mo, Mn, Ms, Mtext, AnnotationXml) => return Err(true),
                    _ => (),
                },
                Namespace::Svg => match context.open_element.local_name {
                    tag!(ForeignObject, Desc, Title) => return Err(true),
                    _ => (),
                },
            }
        }
        Err(false)
    }

    fn find_furthest_block(&self, pos: usize) -> Option<usize> {
        self.context_stack
            .iter()
            .enumerate()
            .skip(pos + 1)
            .find(|(_, context)| {
                if context.is_removed() {
                    return false;
                }
                match context.local_name() {
                    tag!(Mi, Mo, Mn, Ms, Mtext, AnnotationXml) => context.is_mathml(),
                    tag!(ForeignObject, Desc, Title) => context.is_svg(),
                    local_name => local_name.is_special(),
                }
            })
            .map(|(i, _)| i)
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
        let node = self.inner.create_doctype(doctype);
        self.inner
            .append_child(self.context().open_element.node, node);
    }

    #[inline(always)]
    fn push_html_a_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::A);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_applet_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Applet);
        let context = self.context_mut();
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_aside_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Aside);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Aside;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_b_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::B);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_basefont_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Basefont);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_bgsound_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Bgsound);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_body_element(&mut self, tag: &Tag<'_>) {
        debug_assert!(self.body_element.is_none());
        self.push_html_element(tag, LocalName::Body);
        let context = self.context_mut();
        context.reset_mode = mode!(InBody);
        context.element_in_scope |= ElementInScope::Body;
        context.element_in_select_scope.clear();
        self.body_element = Some(context.open_element.node);
    }

    #[inline(always)]
    fn push_html_br_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Br);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_caption_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Caption);
        let context = self.context_mut();
        context.reset_mode = mode!(InCaption);
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_select_scope.clear();
        context.element_in_table_scope |= ElementInTableScope::Caption;
    }

    #[inline(always)]
    fn push_html_center_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Center);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Center;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_col_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Col);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_colgroup_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Colgroup);
        let context = self.context_mut();
        context.reset_mode = mode!(InColumnGroup);
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_dd_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Dd);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Dd;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_div_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Div);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Div;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_dl_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Dl);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Dl;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_dt_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Dt);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Dt;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_em_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Em);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_font_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Font);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_form_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Form);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_frame_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Frame);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_frameset_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Frameset);
        let context = self.context_mut();
        context.reset_mode = mode!(InFrameset);
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_head_element(&mut self, tag: &Tag<'_>) {
        debug_assert!(self.head_element.is_none());
        self.push_html_element(tag, LocalName::Head);
        let context = self.context_mut();
        context.reset_mode = mode!(InHead);
        context.element_in_select_scope.clear();
        self.head_element = Some(context.open_element.node);
    }

    #[inline(always)]
    fn push_html_hr_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Hr);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_html_element(&mut self, tag: &Tag<'_>) {
        debug_assert!(self.html_element.is_none());
        self.push_html_element(tag, LocalName::Html);
        let context = self.context_mut();
        context.reset_mode = mode!(AfterHead);
        context.foster_parenting_insertion_point =
            FosterParentingInsertionPoint::LastChild(context.open_element.node);
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_table_scope.clear();
        context.element_in_select_scope.clear();
        self.html_element = Some(context.open_element.node);
    }

    #[inline(always)]
    fn push_html_i_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::I);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_img_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Img);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_input_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Input);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_li_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Li);
        let context = self.context_mut();
        context.element_in_list_item_scope |= ElementInListItemScope::Li;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_link_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Link);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_main_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Main);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Main;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_marquee_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Marquee);
        let context = self.context_mut();
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_meta_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Meta);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_nobr_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Nobr);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Nobr;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_noframes_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Nobr);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_noscript_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Noscript);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_ol_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Ol);
        let context = self.context_mut();
        context.element_in_list_item_scope.clear();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_optgroup_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Optgroup);
    }

    #[inline(always)]
    fn push_html_option_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Option);
    }

    #[inline(always)]
    fn push_html_p_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::P);
        let context = self.context_mut();
        context.element_in_button_scope |= ElementInButtonScope::P;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_plaintext_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Plaintext);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_pre_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Pre);
        let context = self.context_mut();
        context.element_in_scope |= ElementInScope::Pre;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_script_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Script);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_select_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Select);
        // TODO
        let context = self.context_mut();
        context.reset_mode = mode!(InSelect);
        context.element_in_select_scope.clear();
        context.element_in_select_scope |= ElementInSelectScope::Select;
    }

    #[inline(always)]
    fn push_html_style_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Style);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_table_element(&mut self, tag: &Tag<'_>) {
        let parent = self.context().open_element.node;
        self.push_html_element(tag, LocalName::Table);
        let context = self.context_mut();
        context.reset_mode = mode!(InTable);
        context.foster_parenting_insertion_point =
            FosterParentingInsertionPoint::Before(parent, context.open_element.node);
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_table_scope.clear();
        context.element_in_table_scope |= ElementInTableScope::Table;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_tbody_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Tbody);
        let context = self.context_mut();
        context.reset_mode = mode!(InTableBody);
        context.element_in_table_scope |= ElementInTableScope::Tbody;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_td_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Td);
        let context = self.context_mut();
        context.reset_mode = mode!(InCell);
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_table_scope |= ElementInTableScope::Td;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_template_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Template);
        // TODO: switch the insertion mode to the current template insertion mode
        let context = self.context_mut();
        context.foster_parenting_insertion_point =
            FosterParentingInsertionPoint::LastChild(context.open_element.node);
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_table_scope.clear();
        context.element_in_select_scope.clear();
        context.flags |= flags!(HasTemplateElement);
    }

    #[inline(always)]
    fn push_html_textarea_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Textarea);
    }

    #[inline(always)]
    fn push_html_tfoot_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Tfoot);
        let context = self.context_mut();
        context.reset_mode = mode!(InTableBody);
        context.element_in_table_scope |= ElementInTableScope::Tfoot;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_th_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Th);
        let context = self.context_mut();
        context.reset_mode = mode!(InCell);
        context.element_in_scope.clear();
        context.element_in_list_item_scope.clear();
        context.element_in_button_scope.clear();
        context.element_in_table_scope |= ElementInTableScope::Th;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_thead_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Thead);
        let context = self.context_mut();
        context.reset_mode = mode!(InTableBody);
        context.element_in_table_scope |= ElementInTableScope::Thead;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_title_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Title);
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_tr_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Tr);
        let context = self.context_mut();
        context.reset_mode = mode!(InRow);
        context.element_in_table_scope |= ElementInTableScope::Tr;
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_ul_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::Ul);
        let context = self.context_mut();
        context.element_in_list_item_scope.clear();
        context.element_in_select_scope.clear();
    }

    #[inline(always)]
    fn push_html_unknown_element(&mut self, tag: &Tag<'_>) {
        self.push_html_element(tag, LocalName::lookup(tag.name));
        let context = self.context_mut();
        context.element_in_select_scope.clear();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_html_element(&mut self, tag: &Tag<'_>, local_name: LocalName) {
        tracing::debug!(
            context.pos = self.context_stack.len() - 1,
            context.element = ?self.context().open_element,
        );
        self.append_text_if_exists();
        let node = self.inner.create_element(tag.name, Namespace::Html);
        self.inner.set_attribute(node, tag.attrs(), true);
        self.insert_html_element(OpenElement::with_html(local_name, node))
    }

    #[inline(always)]
    fn context(&self) -> &TreeBuildContext<T::Node> {
        self.nth_context(0)
    }

    #[inline(always)]
    fn nth_context(&self, n: usize) -> &TreeBuildContext<T::Node> {
        debug_assert!(n < self.context_stack.len());
        let pos = self.context_stack.len() - 1 - n;
        &self.context_stack[pos]
    }

    #[inline(always)]
    fn context_mut(&mut self) -> &mut TreeBuildContext<T::Node> {
        self.nth_context_mut(0)
    }

    #[inline(always)]
    fn nth_context_mut(&mut self, n: usize) -> &mut TreeBuildContext<T::Node> {
        debug_assert!(n < self.context_stack.len());
        let pos = self.context_stack.len() - 1 - n;
        &mut self.context_stack[pos]
    }

    fn insert_html_element(&mut self, open_element: OpenElement<T::Node>) {
        self.insert_element(open_element);
        self.context_mut().flags -= flags!(
            MathmlTextIntegrationPoint,
            SvgIntegrationPoint,
            SvgScript,
            HtmlIntegrationPoint
        );
    }

    fn insert_element(&mut self, open_element: OpenElement<T::Node>) {
        self.insert_node(open_element.node);
        let mut context = self.context().clone();
        context.open_element = open_element;
        self.context_stack.push(context);
        tracing::debug!(
            context.pos = self.context_stack.len() - 1,
            context.element = ?self.context().open_element,
        );
    }

    fn insert_node(&mut self, node: T::Node) {
        self.insert_node_with_context(node, self.context_stack.len() - 1);
    }

    fn insert_node_with_context(&mut self, node: T::Node, stack_pos: usize) {
        let context = &self.context_stack[stack_pos];
        tracing::debug!(
            context.pos = stack_pos,
            context.element = ?context.open_element,
        );
        if self.foster_parenting
            && context.is_one_of_html_elements(&tags![Table, Tbody, Tfoot, Thead, Tr])
        {
            match context.foster_parenting_insertion_point {
                FosterParentingInsertionPoint::None => unreachable!(),
                FosterParentingInsertionPoint::LastChild(parent) => {
                    self.inner.append_child(parent, node);
                }
                FosterParentingInsertionPoint::Before(parent, sibling) => {
                    self.inner.insert_before(parent, node, sibling);
                }
            }
        } else {
            self.inner.append_child(context.open_element.node, node);
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn push_mathml_element(&mut self, tag: &Tag<'_>, local_name: LocalName) {
        self.append_text_if_exists();
        let node = self.inner.create_element(tag.name, Namespace::MathMl);
        self.inner.set_attribute(
            node,
            tag.attrs().map(|(name, value)| {
                // TODO: adjust MathML attributes
                // TODO: adjust foreign attributes
                (name, value)
            }),
            true,
        );
        self.insert_element(OpenElement::with_mathml(local_name, node));
        let context = self.context_mut();
        match local_name {
            tag!(mathml: Mi, Mo, Mn, Ms, Mtext) => {
                context.element_in_scope.clear();
                context.element_in_list_item_scope.clear();
                context.element_in_button_scope.clear();
                context.flags -= flags!(SvgIntegrationPoint, SvgScript, HtmlIntegrationPoint,);
                context.flags |= flags!(MathmlTextIntegrationPoint);
            }
            tag!(mathml: AnnotationXml) => {
                context.element_in_scope.clear();
                context.element_in_list_item_scope.clear();
                context.element_in_button_scope.clear();
                context.flags -=
                    flags!(MathmlTextIntegrationPoint, SvgScript, HtmlIntegrationPoint);
                context.flags |= flags!(SvgIntegrationPoint);
            }
            _ => {
                context.flags -= flags!(
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
        let node = self.inner.create_element(tag_name, Namespace::Svg);
        self.inner.set_attribute(
            node,
            tag.attrs().map(|(name, value)| {
                // TODO: adjust foreign attributes
                (name, value)
            }),
            true,
        );
        self.insert_element(OpenElement::with_svg(local_name, node));
        let context = self.context_mut();
        context.flags -= flags!(MathmlTextIntegrationPoint, SvgIntegrationPoint);
        match local_name {
            tag!(svg: Script) => {
                context.flags -= flags!(HtmlIntegrationPoint);
                context.flags |= flags!(SvgScript);
            }
            tag!(svg: ForeignObject, Desc, Title) => {
                context.element_in_scope.clear();
                context.element_in_list_item_scope.clear();
                context.element_in_button_scope.clear();
                context.flags -= flags!(SvgScript);
                context.flags |= flags!(HtmlIntegrationPoint);
            }
            _ => {
                context.flags -= flags!(SvgScript, HtmlIntegrationPoint);
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn reopen_head_element(&mut self) {
        self.append_text_if_exists();
        debug_assert!(self.head_element.is_some());
        let node = self.head_element.expect("<head> must exists");
        self.context_stack.push(TreeBuildContext {
            open_element: OpenElement::with_html(tag!(Head), node),
            reset_mode: InsertionMode::InHead,
            foster_parenting_insertion_point: FosterParentingInsertionPoint::None,
            element_in_scope: Default::default(),
            element_in_list_item_scope: Default::default(),
            element_in_button_scope: Default::default(),
            element_in_table_scope: Default::default(),
            element_in_select_scope: Default::default(),
            flags: Default::default(),
        });
        tracing::debug!(
            context.pos = self.context_stack.len() - 1,
            context.element = ?self.context().open_element,
        );
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_head_element(&mut self) {
        debug_assert!(self.head_element.is_some());
        let node = self.head_element.expect("<head> must exists");
        self.context_stack
            .retain(|context| context.open_element.node != node);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_implied_tags(&mut self) {
        loop {
            match self.context().open_element.local_name {
                tag!(Dd, Dt, Li, Optgroup, Option, P, Rb, Rp, Rt, Rtc) => {
                    self.pop_element();
                }
                _ => return,
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_implied_tags_except_for(&mut self, local_name: LocalName) {
        loop {
            if self.context().open_element.local_name == local_name {
                return;
            }
            match self.context().open_element.local_name {
                tag!(Dd, Dt, Li, Optgroup, Option, P, Rb, Rp, Rt, Rtc) => {
                    self.pop_element();
                }
                _ => return,
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_all_implied_tags(&mut self) {
        loop {
            match self.context().open_element.local_name {
                tag!(
                    Caption, Colgroup, Dd, Dt, Li, Optgroup, Option, P, Rb, Rp, Rt, Rtc, Tbody, Td,
                    Tfoot, Th, Thead, Tr
                ) => {
                    self.pop_element();
                }
                _ => return,
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_p_element(&mut self) {
        const NAMES: &[LocalName] = &tags![Dd, Dt, Li, Optgroup, Option, Rb, Rp, Rt, Rtc];
        self.close_elements(NAMES);
        if self.context().local_name() != LocalName::P {
            // TODO: Parse error.
        }
        while self.context().local_name() != LocalName::P {
            self.pop_element();
        }
        self.pop_element(); // pop a <p>
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close_elements(&mut self, names: &[LocalName]) {
        while names.contains(&self.context().local_name()) {
            self.pop_element();
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn remove_element(&mut self) {
        self.append_text_if_exists();
        let node = self.context().open_element.node;
        self.context_stack.pop();
        self.inner
            .remove_child(self.context().open_element.node, node);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn pop_element(&mut self) {
        self.append_text_if_exists();
        tracing::debug!(
            context.pos = self.context_stack.len() - 1,
            context.element = ?self.context().open_element,
        );
        self.context_stack.pop();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_char(&mut self, c: char) {
        self.text.push(c);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn insert_comment(&mut self, comment: &Comment<'_>) {
        self.append_text_if_exists();
        let node = self.inner.create_comment(comment.data);
        self.insert_node(node);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn end(&mut self) {
        self.append_text_if_exists();
        self.inner.end();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_text_if_exists(&mut self) {
        if !self.text.is_empty() {
            let node = self.inner.create_text(self.text.as_str());
            self.insert_node(node);
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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Namespace {
    Html,
    MathMl,
    Svg,
}

impl Debug for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Html => write!(f, "html"),
            Self::MathMl => write!(f, "mathml"),
            Self::Svg => write!(f, "svg"),
        }
    }
}

#[derive(Clone, Debug)]
struct TreeBuildContext<T> {
    open_element: OpenElement<T>,
    reset_mode: InsertionMode,
    foster_parenting_insertion_point: FosterParentingInsertionPoint<T>,
    element_in_scope: flagset::FlagSet<ElementInScope>,
    element_in_list_item_scope: flagset::FlagSet<ElementInListItemScope>,
    element_in_button_scope: flagset::FlagSet<ElementInButtonScope>,
    element_in_table_scope: flagset::FlagSet<ElementInTableScope>,
    element_in_select_scope: flagset::FlagSet<ElementInSelectScope>,
    flags: flagset::FlagSet<TreeBuildFlags>,
}

impl<T> TreeBuildContext<T>
where
    T: Clone + Copy + Debug + Eq + PartialEq,
{
    fn new(open_element: OpenElement<T>) -> Self {
        TreeBuildContext {
            open_element,
            reset_mode: mode!(InBody),
            foster_parenting_insertion_point: FosterParentingInsertionPoint::None,
            element_in_scope: Default::default(),
            element_in_list_item_scope: Default::default(),
            element_in_button_scope: Default::default(),
            element_in_table_scope: Default::default(),
            element_in_select_scope: Default::default(),
            flags: Default::default(),
        }
    }

    #[inline(always)]
    fn local_name(&self) -> LocalName {
        self.open_element.local_name
    }

    #[inline(always)]
    fn is_html(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.open_element.is_html()
    }

    #[inline(always)]
    fn is_mathml(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.open_element.is_mathml()
    }

    #[inline(always)]
    fn is_svg(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.open_element.is_svg()
    }

    #[inline(always)]
    fn is_html_element(&self, local_name: LocalName) -> bool {
        debug_assert!(!self.is_removed());
        self.open_element.is_html_element(local_name)
    }

    #[inline(always)]
    fn is_one_of_html_elements(&self, local_names: &[LocalName]) -> bool {
        debug_assert!(!self.is_removed());
        self.open_element.is_html() && local_names.contains(&self.open_element.local_name)
    }

    #[inline(always)]
    fn is_html_heading_element(&self) -> bool {
        const NAMES: &[LocalName] = &tags![H1, H2, H3, H4, H5, H6];
        debug_assert!(!self.is_removed());
        self.is_one_of_html_elements(NAMES)
    }

    #[inline(always)]
    fn is_removed(&self) -> bool {
        self.flags.contains(flags!(Removed))
    }

    #[inline(always)]
    fn is_mathml_text_integration_point(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.flags.contains(flags!(MathmlTextIntegrationPoint))
    }

    #[inline(always)]
    fn is_svg_integration_point(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.flags.contains(flags!(SvgIntegrationPoint))
    }

    #[inline(always)]
    fn is_svg_script(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.flags.contains(flags!(SvgScript))
    }

    #[inline(always)]
    fn is_html_integration_point(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.flags.contains(flags!(HtmlIntegrationPoint))
    }

    #[inline(always)]
    fn has_template_element(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.flags.contains(flags!(HasTemplateElement))
    }

    #[inline(always)]
    fn has_aside_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Aside)
    }

    #[inline(always)]
    fn has_body_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Body)
    }

    #[inline(always)]
    fn has_center_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Center)
    }

    #[inline(always)]
    fn has_dd_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Dd)
    }

    #[inline(always)]
    fn has_div_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Div)
    }

    #[inline(always)]
    fn has_dl_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Dl)
    }

    #[inline(always)]
    fn has_dt_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Dt)
    }

    #[inline(always)]
    fn has_form_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Form)
    }

    #[inline(always)]
    fn has_main_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Main)
    }

    #[inline(always)]
    fn has_nobr_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Nobr)
    }

    #[inline(always)]
    fn has_ol_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Ol)
    }

    #[inline(always)]
    fn has_pre_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Pre)
    }

    #[inline(always)]
    fn has_ul_element_in_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_scope.contains(ElementInScope::Ul)
    }

    #[inline(always)]
    fn has_li_element_in_list_item_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_list_item_scope
            .contains(ElementInListItemScope::Li)
    }

    #[inline(always)]
    fn has_p_element_in_button_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_button_scope
            .contains(ElementInButtonScope::P)
    }

    #[inline(always)]
    fn has_table_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Table)
    }

    #[inline(always)]
    fn has_caption_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Caption)
    }

    #[inline(always)]
    fn has_rowgroup_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        !self.element_in_table_scope.is_disjoint(
            ElementInTableScope::Tbody | ElementInTableScope::Tfoot | ElementInTableScope::Thead,
        )
    }

    #[inline(always)]
    fn has_tbody_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Tbody)
    }

    #[inline(always)]
    fn has_tfoot_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Tfoot)
    }

    #[inline(always)]
    fn has_thead_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Thead)
    }

    #[inline(always)]
    fn has_tr_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Tr)
    }

    #[inline(always)]
    fn has_cell_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        !self
            .element_in_table_scope
            .is_disjoint(ElementInTableScope::Td | ElementInTableScope::Th)
    }

    #[inline(always)]
    fn has_td_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Td)
    }

    #[inline(always)]
    fn has_th_element_in_table_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_table_scope
            .contains(ElementInTableScope::Th)
    }

    #[inline(always)]
    fn has_select_element_in_select_scope(&self) -> bool {
        debug_assert!(!self.is_removed());
        self.element_in_select_scope
            .contains(ElementInSelectScope::Select)
    }
}

#[derive(Clone)]
struct OpenElement<T> {
    namespace: Namespace,
    local_name: LocalName,
    node: T,
}

impl<T> OpenElement<T> {
    fn with_html(local_name: LocalName, node: T) -> Self {
        OpenElement {
            namespace: Namespace::Html,
            local_name,
            node,
        }
    }

    fn with_mathml(local_name: LocalName, node: T) -> Self {
        OpenElement {
            namespace: Namespace::MathMl,
            local_name,
            node,
        }
    }

    fn with_svg(local_name: LocalName, node: T) -> Self {
        OpenElement {
            namespace: Namespace::Svg,
            local_name,
            node,
        }
    }

    #[inline(always)]
    fn is_html(&self) -> bool {
        self.namespace == Namespace::Html
    }

    #[inline(always)]
    fn is_mathml(&self) -> bool {
        self.namespace == Namespace::MathMl
    }

    #[inline(always)]
    fn is_svg(&self) -> bool {
        self.namespace == Namespace::Svg
    }

    #[inline(always)]
    fn is_html_element(&self, local_name: LocalName) -> bool {
        self.namespace == Namespace::Html && self.local_name == local_name
    }
}

impl<T> Debug for OpenElement<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}:<{:?}:{:?}>",
            self.node, self.namespace, self.local_name
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum FosterParentingInsertionPoint<T> {
    None,
    LastChild(T),
    Before(T, T),
}

flagset::flags! {
    enum ElementInScope: u16 {
        Aside,
        Body,
        Center,
        Dd,
        Div,
        Dl,
        Dt,
        Form,
        Main,
        Nobr,
        Ol,
        Pre,
        Ul,
    }

    enum ElementInListItemScope: u8 {
        Li,
    }

    enum ElementInButtonScope: u8 {
        P,
    }

    enum ElementInTableScope: u8 {
        Table,
        Caption,
        Tbody,
        Tfoot,
        Thead,
        Tr,
        Td,
        Th,
    }

    enum ElementInSelectScope: u8 {
        Select,
    }

    enum TreeBuildFlags: u8 {
        Removed,
        MathmlTextIntegrationPoint,
        SvgIntegrationPoint,
        SvgScript,
        HtmlIntegrationPoint,
        HasTemplateElement,
    }
}

#[derive(PartialEq)]
enum ActiveFormattingContext<T> {
    Marker,
    Element { local_name: LocalName, node: T },
    Removed,
}

impl<T: Debug> Debug for ActiveFormattingContext<T>
where
    T: Copy + Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Marker => write!(f, "$marker"),
            Self::Element { local_name, node } => write!(f, "{:?}:<{:?}>", node, local_name),
            Self::Removed => write!(f, "$removed"),
        }
    }
}

struct ActiveFormattingElementList<T>(Vec<ActiveFormattingContext<T>>);

impl<T> ActiveFormattingElementList<T>
where
    T: Clone + Copy + Debug + Eq + PartialEq,
{
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn remove(&mut self, i: usize) {
        tracing::debug!(index = i, item = ?self.0[i]);
        self.0[i] = ActiveFormattingContext::Removed;
    }

    fn clean(&mut self) {
        self.0.retain(|context| match context {
            ActiveFormattingContext::Removed => false,
            _ => true,
        });
    }

    fn get(&self, i: usize) -> &ActiveFormattingContext<T> {
        self.0.get(i).unwrap()
    }

    fn clear_up_to_last_marker(&mut self) {
        while let Some(context) = self.0.pop() {
            if let ActiveFormattingContext::Marker = context {
                break;
            }
        }
    }

    fn set_element(&mut self, i: usize, element: T) {
        match self.0.get_mut(i).unwrap() {
            ActiveFormattingContext::Marker => unreachable!(),
            ActiveFormattingContext::Element { ref mut node, .. } => *node = element,
            ActiveFormattingContext::Removed => (),
        }
    }

    fn get_element(&self, i: usize) -> T {
        match self.0.get(i).unwrap() {
            ActiveFormattingContext::Element { node, .. } => *node,
            _ => unreachable!(),
        }
    }

    fn get_local_name(&self, i: usize) -> LocalName {
        match self.0.get(i).unwrap() {
            ActiveFormattingContext::Element { local_name, .. } => *local_name,
            _ => unreachable!(),
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn insert_element(&mut self, i: usize, local_name: LocalName, node: T) {
        self.0
            .insert(i, ActiveFormattingContext::Element { local_name, node });
        tracing::debug!(index = i, item = ?self.0[i]);
    }

    fn push_marker(&mut self) {
        self.0.push(ActiveFormattingContext::Marker);
    }

    fn push_element(&mut self, local_name: LocalName, node: T) {
        self.0
            .push(ActiveFormattingContext::Element { local_name, node });
    }

    fn find_element(&self, element: T) -> Option<usize> {
        for (i, context) in self.0.iter().enumerate().rev() {
            match context {
                ActiveFormattingContext::Marker => {
                    return None;
                }
                ActiveFormattingContext::Element { node: v, .. } => {
                    if element == *v {
                        return Some(i);
                    }
                }
                ActiveFormattingContext::Removed => continue,
            }
        }
        None
    }

    fn find_last_element(&self, local_name: LocalName) -> Option<usize> {
        for (i, context) in self.0.iter().enumerate().rev() {
            match context {
                ActiveFormattingContext::Marker => {
                    return None;
                }
                ActiveFormattingContext::Element { local_name: v, .. } => {
                    if local_name == *v {
                        return Some(i);
                    }
                }
                ActiveFormattingContext::Removed => continue,
            }
        }
        None
    }
}

impl<T> Default for ActiveFormattingElementList<T>
where
    T: Copy + Clone + Debug,
{
    fn default() -> Self {
        ActiveFormattingElementList(vec![])
    }
}

impl<T> Debug for ActiveFormattingElementList<T>
where
    T: Copy + Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
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
pub enum QuirksMode {
    NoQuirks,
    Quirks,
    LimitedQuirks,
}
