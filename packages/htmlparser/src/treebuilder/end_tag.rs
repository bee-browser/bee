use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_end_tag(&mut self, tag: Tag<'_>) -> Control {
        self.ignore_lf = false;
        match LocalName::lookup(tag.name) {
            tag!(Html) => self.handle_end_html(&tag),
            tag!(Head) => self.handle_end_head(&tag),
            tag!(Script) => self.handle_end_script(&tag),
            tag!(Body) => self.handle_end_body(&tag),
            tag!(P) => self.handle_end_p(&tag),
            tag!(Colgroup) => self.handle_end_colgroup(&tag),
            tag!(Frameset) => self.handle_end_frameset(&tag),
            _ => loop {
                tracing::debug!(mode = ?self.mode, ?tag);
                match self.handle_any_other_end_tag(&tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                }
            },
        }
    }

    fn handle_end_html(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: If the stack of open elements does not have a body element in scope, this is a parse error; ignore the token.
                    // TODO: Otherwise
                    self.switch_to(mode!(AfterBody));
                    return Control::Continue;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_end_head(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InHead) => {
                    // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                    self.pop_element();
                    self.switch_to(mode!(AfterHead));
                    return Control::Continue;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_end_script(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(Text) => {
                    // TODO: If the active speculative HTML parser is null and the JavaScript execution context stack is empty, then perform a microtask checkpoint.
                    self.pop_element();
                    self.switch_to_original_mode();
                    return Control::ExecuteScript;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_end_body(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    // TODO: If the stack of open elements does not have a body element in scope, this is a parse error; ignore the token.
                    // TODO: Otherwise
                    self.pop_element();
                    self.switch_to(mode!(AfterBody));
                    return Control::Continue;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_end_p(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InBody) => {
                    if self.context.has_p_element_in_button_scope {
                        // TODO: Parse error.
                        self.push_html_element(&Tag::with_no_attrs("p"));
                    }
                    self.close_p_element();
                    return Control::Continue;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_end_colgroup(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InTable, InCaption, InTableBody, InRow, InCell) => {
                    // TODO: Parse error.
                    // Ignore the token.
                    return Control::Continue;
                }
                mode!(InColumnGroup) => {
                    match self.context.local_name {
                        LocalName::Colgroup => {
                            self.pop_element();
                            self.switch_to(mode!(InTable));
                        }
                        _ => {
                            // TODO: Parse error.
                            // Ignore the token.
                        }
                    }
                    return Control::Continue;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_end_frameset(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(InFrameset) => {
                    // TODO: If the current node is the root html element, then this is a parse error; ignore the token. (fragment case)
                    self.pop_element();
                    // TODO: If the parser was not created as part of the HTML fragment parsing algorithm (fragment case), and the current node is no longer a frameset element, then switch the insertion mode to "after frameset".
                    self.switch_to(mode!(AfterFrameset));
                    return Control::Continue;
                }
                _ => match self.handle_any_other_end_tag(tag) {
                    Control::Reprocess => (),
                    ctrl => return ctrl,
                },
            }
        }
    }

    fn handle_any_other_end_tag(&mut self, tag: &Tag<'_>) -> Control {
        match self.mode {
            mode!(
                BeforeHtml,
                BeforeHead,
                InHead,
                InHeadNoscript,
                AfterHead,
                InTemplate
            ) => {
                // TODO: Parse error.
                // Ignore the token.
                return Control::Continue;
            }
            mode!(InBody, InCaption, InCell) => {
                // TODO
                self.pop_element();
                return Control::Continue;
            }
            mode!(Text) => {
                self.pop_element();
                self.switch_to_original_mode();
                return Control::Continue;
            }
            mode!(InTable, InTableBody, InRow) => {
                // TODO: Parse error.
                // TODO: Enable foster parenting,
                // TODO: process the token using the rules for the "in body" insertion mode,
                // TODO: and then disable foster parenting.
                self.pop_element();
                return Control::Continue;
            }
            _ => self.handle_anything_else(),
        }
    }
}
