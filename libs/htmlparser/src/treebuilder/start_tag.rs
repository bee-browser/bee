use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    pub fn handle_start_tag(&mut self, tag: Tag<'_>) -> Control {
        self.ignore_lf = false;
        match LocalName::lookup(tag.name) {
            tag!(Html) => self.handle_start_html(&tag),
            tag!(Head) => self.handle_start_head(&tag),
            tag!(Title) => self.handle_start_title(&tag),
            tag!(Style) => self.handle_start_style(&tag),
            tag!(Script) => self.handle_start_script(&tag),
            tag!(Body) => self.handle_start_body(&tag),
            tag!(B) => self.handle_start_b(&tag),
            tag!(P) => self.handle_start_p(&tag),
            tag!(Pre) => self.handle_start_pre(&tag),
            tag!(Input) => self.handle_start_input(&tag),
            tag!(Textarea) => self.handle_start_textarea(&tag),
            tag!(Table) => self.handle_start_table(&tag),
            tag!(Colgroup) => self.handle_start_colgroup(&tag),
            tag!(Tr) => self.handle_start_tr(&tag),
            tag!(Td) => self.handle_start_td(&tag),
            tag!(Plaintext) => self.handle_start_plaintext(&tag),
            tag!(Frameset) => self.handle_start_frameset(&tag),
            local_name @ tag!(svg: Svg) => self.handle_start_svg(&tag, local_name),
            tag!(mathml: Math) => self.handle_start_math(&tag),
            _ => loop {
                logger::debug!(mode = ?self.mode, ?tag);
                match self.handle_any_other_start_tag(&tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            },
        }
    }

    fn handle_start_html(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(BeforeHtml) => {
                    // TODO: Create an element for the token in the HTML namespace, with the Document as the intended parent.
                    // TODO: Append it to the Document object. Put this element in the stack of open elements.
                    self.push_html_element(&tag);
                    self.switch_to(mode!(BeforeHead));
                    return Control::Continue;
                }
                mode!(
                    BeforeHead,
                    InHead,
                    InHeadNoscript,
                    AfterHead,
                    InBody,
                    InCaption,
                    InColumnGroup,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    AfterBody,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterBody,
                    AfterAfterFrameset
                ) => {
                    // TODO: Parse error.
                    // TODO: If there is a template element on the stack of open elements, then ignore the token.
                    // TODO: Otherwise, for each attribute on the token, check to see if the attribute is already present on the top element of the stack of open elements. If it is not, add the attribute and its corresponding value to that element.
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            };
        }
    }

    fn handle_start_head(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(BeforeHead) => {
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(&tag);
                    // TODO: Set the head element pointer to the newly created head element.
                    self.switch_to(mode!(InHead));
                    return Control::Continue;
                }
                mode!(
                    InHead,
                    InHeadNoscript,
                    AfterHead,
                    InSelect,
                    InSelectInTable,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterFrameset
                ) => {
                    return self.ignore_token();
                }
                mode!(InTable, InTableBody, InRow) => {
                    return self.apply_any_other_start_tag_rule_on_in_table(tag, Namespace::Html);
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_title(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InHead, InBody) => {
                    return self.apply_generic_rcdata_element_rule(tag);
                }
                mode!(AfterHead) => {
                    // TODO: Parse error.
                    // TODO: Push the node pointed to by the head element pointer onto the stack of open elements.
                    // TODO: Process the token using the rules for the "in head" insertion mode.
                    // TODO: Remove the node pointed to by the head element pointer from the stack of open elements. (It might not be the current node at this point.)
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_style(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InHead, InHeadNoscript, InBody, InTable, InTemplate) => {
                    return self.apply_generic_raw_text_element_rule(tag);
                }
                mode!(AfterHead) => {
                    // TODO: Parse error.
                    self.reopen_head_element();
                    // Process the token using the rules for the "in head" insertion mode.
                    return self.apply_generic_raw_text_element_rule(tag);
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_script(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InHead) => {
                    // In the "in head" insertion mode, the "appropriate place for
                    // inserting a node" is always after the last child node of
                    // the "head" at this point.  This means that we can simply
                    // call `self.push_html_element(tag)`.
                    //
                    // We don't take care of the "script" element's "parser
                    // document" and "force async" properties here.  They will
                    // be set appropriately in push_html_element().
                    self.push_html_element(tag);
                    // TODO: If the parser was created as part of the HTML fragment parsing algorithm, then set the script element's already started to true. (fragment case)
                    // TODO: If the parser was invoked via the document.write() or document.writeln() methods, then optionally set the script element's already started to true. (For example, the user agent might use this clause to prevent execution of cross-origin scripts inserted via document.write() under slow network conditions, or when the page has already taken a long time to load.)
                    self.save_and_switch_to(mode!(Text));
                    return Control::SwitchTo(InitialState::ScriptData);
                }
                mode!(AfterHead) => {
                    // TODO: Parse error.
                    self.reopen_head_element();
                    // Process the token using the rules for the "in head" insertion mode.
                    self.push_html_element(tag);
                    self.save_and_switch_to(mode!(Text));
                    return Control::SwitchTo(InitialState::ScriptData);
                }
                mode!(InBody, InTable, InSelect, InTemplate) => {
                    // Process the token using the rules for the "in head" insertion mode.
                    self.push_html_element(tag);
                    self.save_and_switch_to(mode!(Text));
                    return Control::SwitchTo(InitialState::ScriptData);
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_body(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(AfterHead) => {
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(&tag);
                    self.frameset_ok = false;
                    self.switch_to(mode!(InBody));
                    return Control::Continue;
                }
                mode!(InBody, InCaption, InCell) => {
                    // TODO: Parse error.
                    // TODO: If the second element on the stack of open elements is not a body element, if the stack of open elements has only one node on it, or if there is a template element on the stack of open elements, then ignore the token. (fragment case)
                    // TODO: Otherwise, set the frameset-ok flag to "not ok"; then, for each attribute on the token, check to see if the attribute is already present on the body element (the second element) on the stack of open elements, and if it is not, add the attribute and its corresponding value to that element.
                    self.frameset_ok = false;
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_b(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    if self.context.has_p_element_in_button_scope {
                        self.close_p_element();
                    }
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(tag);
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_p(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    if self.context.has_p_element_in_button_scope {
                        self.close_p_element();
                    }
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(tag);
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_pre(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(tag);
                    self.ignore_lf = true;
                    self.frameset_ok = false;
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_input(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: Reconstruct the active formatting elements, if any.
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(tag);
                    self.pop_element();
                    if Self::is_visible_input(tag) {
                        self.frameset_ok = false;
                    }
                    return Control::Continue;
                }
                mode!(InTable) => {
                    if Self::is_visible_input(tag) {
                        // act as described in the "anything else" entry below.
                        self.foster_parenting = true;
                        // TODO: Reconstruct the active formatting elements, if any.
                        // TODO: Insert an HTML element for the token.
                        self.push_html_element(tag);
                        self.pop_element();
                        self.frameset_ok = false;
                        self.foster_parenting = false;
                        return Control::Continue;
                    }
                    // TODO: Parse error.
                    self.push_html_element(tag);
                    self.pop_element();
                    if tag.self_closing {
                        // TODO: Acknowledge the token's self-closing flag
                    }
                    return Control::Continue;
                }
                mode!(InSelect) => {
                    // TODO: Parse error.
                    if !self.context.has_select_element_in_select_scope {
                        // Ignore the token.
                        return Control::Continue;
                    }
                    loop {
                        match self.context.local_name {
                            tag!(Select) => {
                                self.pop_element();
                                break;
                            }
                            _ => {
                                self.pop_element();
                            }
                        }
                    }
                    self.reset_insertion_mode_appropriately();
                    // Reprocess the token.
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn is_visible_input(tag: &Tag<'_>) -> bool {
        debug_assert!(tag.name == "input");
        for (name, value) in tag.attrs() {
            if name == "type" {
                if value.eq_ignore_ascii_case("hidden") {
                    return false;
                }
                return true;
            }
        }
        true
    }

    fn handle_start_textarea(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(tag);
                    self.ignore_lf = true;
                    self.frameset_ok = false;
                    self.save_and_switch_to(mode!(Text));
                    return Control::SwitchTo(InitialState::Rcdata);
                }
                mode!(InSelect) => {
                    // TODO: Parse error.
                    // TODO: If the stack of open elements does not have a select element in select scope, ignore the token. (fragment case)
                    loop {
                        match self.context.local_name {
                            LocalName::Select => {
                                self.pop_element();
                                break;
                            }
                            _ => {
                                self.pop_element();
                            }
                        }
                    }
                    self.reset_insertion_mode_appropriately();
                    // Reprocess the token.
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_table(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    if self.quirks_mode != QuirksMode::Quirks {
                        self.close_p_element_in_button_scope();
                    }
                    self.push_html_element(tag);
                    self.frameset_ok = false;
                    self.switch_to(mode!(InTable));
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn close_p_element_in_button_scope(&mut self) {
        // TODO
    }

    fn handle_start_colgroup(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TOOD: Parse error.
                    // Ignore the token.
                    return Control::Continue;
                }
                mode!(InTable) => {
                    self.clear_stack_back_to_table_context();
                    self.push_html_element(tag);
                    self.switch_to(mode!(InColumnGroup));
                    return Control::Continue;
                }
                mode!(InCaption) => {
                    // TODO
                    self.switch_to(mode!(InTable));
                    // Reprocess the token.
                }
                mode!(InTableBody) => {
                    // TODO
                    self.clear_stack_back_to_table_body_context();
                    self.pop_element();
                    self.switch_to(mode!(InTable));
                    // Reprocess the token.
                }
                mode!(InRow) => {
                    // TODO
                    self.clear_stack_back_to_table_row_context();
                    self.pop_element();
                    self.switch_to(mode!(InTableBody));
                    // Reprocess the token.
                }
                mode!(InCell) => {
                    // TODO
                    self.close_cell();
                    // Reprocess the token.
                }
                mode!(InTemplate) => {
                    // TODO
                    self.switch_to(mode!(InTable));
                    // Reprocess the token.
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_tr(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TOOD: Parse error.
                    // Ignore the token.
                    return Control::Continue;
                }
                mode!(InTable) => {
                    self.clear_stack_back_to_table_context();
                    self.push_html_element(&Tag::with_no_attrs("tbody"));
                    self.switch_to(mode!(InTableBody));
                    // Reprocess the token.
                }
                mode!(InCaption) => {
                    // TODO: If the stack of open elements does not have a caption element in table scope, this is a parse error; ignore the token. (fragment case)
                    // TODO: Generate implied end tags.
                    // TODO: Now, if the current node is not a caption element, then this is a parse error.
                    // TODO: Pop elements from this stack until a caption element has been pop_elementped from the stack.
                    // TODO: Clear the list of active formatting elements up to the last marker.
                    self.switch_to(mode!(InTable));
                    // Reprocess the token.
                }
                mode!(InTableBody) => {
                    self.clear_stack_back_to_table_body_context();
                    // TODO: Insert an HTML element for the token
                    self.push_html_element(tag);
                    self.switch_to(mode!(InRow));
                    return Control::Continue;
                }
                mode!(InRow) => {
                    // TODO: If the stack of open elements does not have a tr element in table scope, this is a parse error; ignore the token.
                    self.clear_stack_back_to_table_row_context();
                    self.pop_element();
                    self.switch_to(mode!(InTableBody));
                    // Reprocess the token.
                }
                mode!(InCell) => {
                    // TODO: If the stack of open elements does not have a td or th element in table scope, then this is a parse error; ignore the token. (fragment case)
                    self.close_cell();
                    // Reprocess the token.
                }
                mode!(InSelectInTable) => {
                    // TODO: Parse error.
                    loop {
                        match self.context.local_name {
                            tag!(Select) => {
                                self.pop_element();
                                break;
                            }
                            _ => {
                                self.pop_element();
                            }
                        }
                    }
                    self.reset_insertion_mode_appropriately();
                    // Reprocess the token.
                }
                mode!(InTemplate) => {
                    // TODO
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_td(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InRow) => {
                    self.clear_stack_back_to_table_row_context();
                    self.push_html_element(tag);
                    self.switch_to(mode!(InCell));
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn reset_insertion_mode_appropriately(&mut self) {
        self.switch_to(self.context.reset_mode);
    }

    fn clear_stack_back_to_table_context(&mut self) {
        loop {
            match self.context.local_name {
                tag!(Html, Table, Template) => break,
                _ => self.remove_element(),
            }
        }
    }

    fn clear_stack_back_to_table_body_context(&mut self) {
        loop {
            match self.context.local_name {
                tag!(Tbody, Tfoot, Thead) => break,
                _ => self.remove_element(),
            }
        }
    }

    fn clear_stack_back_to_table_row_context(&mut self) {
        loop {
            match self.context.local_name {
                tag!(Html, Template, Tr) => break,
                _ => self.remove_element(),
            }
        }
    }

    fn close_cell(&mut self) {
        // TODO: Generate implied end tags.
        loop {
            match self.context.local_name {
                tag!(Td, Th) => {
                    self.pop_element();
                    break;
                }
                _ => {
                    // TODO: Parse error.
                    self.pop_element();
                }
            }
        }
        // TODO: Clear the list of active formatting elements up to the last marker.
        self.switch_to(mode!(InRow));
    }

    fn handle_start_plaintext(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                    self.push_html_element(tag);
                    return Control::SwitchTo(InitialState::Plaintext);
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_frameset(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(AfterHead) => {
                    // TODO: Insert an HTML element for the token.
                    self.push_html_element(tag);
                    self.switch_to(mode!(InFrameset));
                    return Control::Continue;
                }
                mode!(InBody, InCaption, InCell) => {
                    // TODO: Parse error.
                    // TODO: Ignore the token (fragment case)
                    if !self.frameset_ok {
                        // Ignore the token
                        return Control::Continue;
                    }
                    // TODO: Otherwise, run the following steps:
                    loop {
                        if let LocalName::Html = self.context.local_name {
                            break;
                        }
                        self.remove_element();
                    }
                    self.push_html_element(tag);
                    self.switch_to(mode!(InFrameset));
                    return Control::Continue;
                }
                mode!(InFrameset) => {
                    self.push_html_element(tag);
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Html) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_math(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: Reconstruct the active formatting elements, if any.
                    // TODO: Adjust MathML attributes for the token. (This fixes the case of MathML attributes that are not all lowercase.)
                    // TODO: Adjust foreign attributes for the token. (This fixes the use of namespaced attributes, in particular XLink.)
                    // TODO: Insert a foreign element for the token, in the MathML namespace.
                    self.push_mathml_element(tag);
                    // TODO: If the token has its self-closing flag set, pop_element the current node off the stack of open elements and acknowledge the token's self-closing flag.
                    if tag.self_closing {
                        self.pop_element();
                    }
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::MathMl) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_start_svg(&mut self, tag: &Tag<'_>, local_name: LocalName) -> Control {
        loop {
            logger::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: Reconstruct the active formatting elements, if any.
                    // TODO: Adjust SVG attributes for the token. (This fixes the case of SVG attributes that are not all lowercase.)
                    // TODO: Adjust foreign attributes for the token. (This fixes the use of namespaced attributes, in particular XLink in SVG.)
                    // TODO: Insert a foreign element for the token, in the SVG namespace.
                    self.push_svg_element(tag, local_name);
                    // TODO: If the token has its self-closing flag set, pop_element the current node off the stack of open elements and acknowledge the token's self-closing flag.
                    if tag.self_closing {
                        self.pop_element();
                    }
                    return Control::Continue;
                }
                _ => match self.handle_any_other_start_tag(tag, Namespace::Svg) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_any_other_start_tag(&mut self, tag: &Tag<'_>, namespace: Namespace) -> Control {
        match self.mode {
            mode!(InBody, InCaption, InCell) => {
                self.apply_any_other_start_tag_rule_on_in_body(tag, namespace)
            }
            mode!(InTable, InTableBody, InRow) => {
                self.apply_any_other_start_tag_rule_on_in_table(tag, namespace)
            }
            mode!(InTemplate) => self.apply_any_other_start_tag_rule_on_in_template(),
            _ => self.handle_anything_else(),
        }
    }

    fn apply_any_other_start_tag_rule_on_in_body(
        &mut self,
        tag: &Tag<'_>,
        namespace: Namespace,
    ) -> Control {
        // TODO: Reconstruct the active formatting elements, if any.
        // TODO: Insert an HTML element for the token.
        match namespace {
            Namespace::Html => self.push_html_element(tag),
            Namespace::MathMl => self.push_mathml_element(tag),
            Namespace::Svg => self.push_svg_element(tag, LocalName::Svg),
        }
        Control::Continue
    }

    fn apply_any_other_start_tag_rule_on_in_table(
        &mut self,
        tag: &Tag<'_>,
        namespace: Namespace,
    ) -> Control {
        // TODO: Parse error.
        self.foster_parenting = true;
        let ctrl = self.apply_any_other_start_tag_rule_on_in_body(tag, namespace);
        self.foster_parenting = false;
        ctrl
    }

    fn apply_any_other_start_tag_rule_on_in_template(&mut self) -> Control {
        // TODO: Pop the current template insertion mode off the stack of template insertion modes.
        // TODO: Push "in body" onto the stack of template insertion modes so that it is the new current template insertion mode.
        self.switch_to(mode!(InBody));
        Control::Reprocess
    }

    fn apply_generic_rcdata_element_rule(&mut self, tag: &Tag<'_>) -> Control {
        self.push_html_element(tag);
        self.save_and_switch_to(mode!(Text));
        Control::SwitchTo(InitialState::Rcdata)
    }

    fn apply_generic_raw_text_element_rule(&mut self, tag: &Tag<'_>) -> Control {
        self.push_html_element(tag);
        self.save_and_switch_to(mode!(Text));
        Control::SwitchTo(InitialState::Rawtext)
    }
}
