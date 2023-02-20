use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_start_tag(&mut self, tag: Tag<'_>) -> Control {
        match tag.name {
            tag!(HTML) => self.handle_start_html(&tag),
            tag!(HEAD) => self.handle_start_head(&tag),
            tag!(BODY) => self.handle_start_body(&tag),
            tag!(TITLE) => self.handle_start_title(&tag),
            _ => loop {
                match self.handle_any_other_start_tag(&tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            }
        }
    }

    fn handle_start_html(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(BeforeHtml) => {
                    // TODO: Create an element for the token in the HTML namespace, with the Document as the intended parent.
                    // TODO: Append it to the Document object. Put this element in the stack of open elements.
                    self.push_element(&tag);
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
                _ => match self.handle_any_other_start_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            };
        }
    }

    fn handle_start_head(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            match self.mode {
                mode!(BeforeHead) => {
                    // TODO: Insert an HTML element for the token.
                    self.push_element(&tag);
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
                    return self.apply_any_other_start_tag_rule_on_in_table(tag);
                }
                _ => match self.handle_any_other_start_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            }
        }
    }

    fn handle_start_body(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            match self.mode {
                mode!(AfterHead) => {
                    // TODO: Insert an HTML element for the token.
                    self.push_element(&tag);
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
                _ => match self.handle_any_other_start_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            }
        }
    }

    fn handle_start_title(&mut self, tag: &Tag<'_>) -> Control {
        loop {
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
                _ => match self.handle_any_other_start_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            }
        }
    }

    fn handle_any_other_start_tag(&mut self, tag: &Tag<'_>) -> Control {
        match self.mode {
            mode!(InBody, InCaption, InCell) => {
                self.apply_any_other_start_tag_rule_on_in_body(tag)
            }
            mode!(InTable, InTableBody, InRow) => {
                self.apply_any_other_start_tag_rule_on_in_table(tag)
            }
            mode!(InTemplate) => {
                self.apply_any_other_start_tag_rule_on_in_template()
            }
            mode!(InForeignContent) => {
                self.apply_any_other_start_tag_rule_on_in_foreign_content()
            }
            _ => self.handle_anything_else(),
        }
    }

    fn apply_any_other_start_tag_rule_on_in_body(&mut self, tag: &Tag<'_>) -> Control {
        // TODO: Reconstruct the active formatting elements, if any.
        // TODO: Insert an HTML element for the token.
        self.push_element(&tag);
        Control::Continue
    }

    fn apply_any_other_start_tag_rule_on_in_table(&mut self, tag: &Tag<'_>) -> Control {
        // TODO: Parse error.
        // TODO: Enable foster parenting,
        let ctrl = self.apply_any_other_start_tag_rule_on_in_body(tag);
        // TODO: and then disable foster parenting.
        ctrl
    }

    fn apply_any_other_start_tag_rule_on_in_template(&mut self) -> Control {
        // TODO: Pop the current template insertion mode off the stack of template insertion modes.
        // TODO: Push "in body" onto the stack of template insertion modes so that it is the new current template insertion mode.
        self.switch_to(mode!(InBody));
        Control::Reprocess
    }

    fn apply_any_other_start_tag_rule_on_in_foreign_content(&mut self) -> Control {
        // TODO
        Control::Continue
    }

    fn apply_generic_rcdata_element_rule(&mut self, tag: &Tag<'_>) -> Control {
        self.push_element(tag);
        self.save_and_switch_to(mode!(Text));
        Control::SwitchTo(bee_htmltokenizer::InitialState::Rcdata)
    }
}
