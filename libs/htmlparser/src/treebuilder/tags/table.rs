use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_start_table(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if !self.iframe_srcdoc {
                            // TODO: Parse error.
                        }
                        self.change_quirks_mode_if_changeable(QuirksMode::Quirks);
                        self.switch_to(mode!(BeforeHtml));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(BeforeHtml) => {
                    let ctrl = {
                        //debug_assert!(self.writer.is_empty());
                        self.push_html_element(&Tag::with_no_attrs("html"));
                        self.switch_to(mode!(BeforeHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(BeforeHead) => {
                    let ctrl = {
                        self.push_html_element(&Tag::with_no_attrs("head"));
                        // TODO: Set the head element pointer to the newly created head element.
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InHead) => {
                    let ctrl = {
                        debug_assert!(self.context.local_name == LocalName::Head);
                        self.pop_element();
                        self.switch_to(mode!(AfterHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InHeadNoscript) => {
                    let ctrl = {
                        // TODO: Parse error.
                        debug_assert!(self.context.local_name == LocalName::Noscript);
                        self.pop_element();
                        debug_assert!(self.context.local_name == LocalName::Head);
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterHead) => {
                    let ctrl = {
                        self.push_html_element(&Tag::with_no_attrs("body"));
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InBody, InCaption, InCell) => {
                    let ctrl = {
                        if self.quirks_mode != QuirksMode::Quirks {
                            self.close_p_element_in_button_scope();
                        }
                        self.push_html_element(tag);
                        self.frameset_ok = false;
                        self.switch_to(mode!(InTable));
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTable, InTableBody, InRow) => {
                    let ctrl = {
                        // TODO: Parse error.
                        // TODO: If the stack of open elements does not have a table element in table scope, ignore the token.
                        // TODO: Otherwise
                        // TODO: Pop elements from this stack until a table element has been popped from the stack.
                        // TODO: Reset the insertion mode appropriately.
                        // TODO: Reprocess the token.
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        // TODO
                        if !self.text.is_empty() {
                            self.insert_text_to_foster_parent();
                        }
                        self.switch_to_original_mode();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InColumnGroup) => {
                    let ctrl = {
                        if self.context.local_name == LocalName::Colgroup {
                            // TODO: Parse error.
                            // Ignore the token.
                            Control::Continue
                        } else {
                            self.pop_element();
                            self.switch_to(mode!(InTable));
                            Control::Reprocess
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InSelect, InFrameset, AfterFrameset, AfterAfterFrameset) => {
                    let ctrl = {
                        // TODO: Parse error.
                        // Ignore the token.
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InSelectInTable) => {
                    let ctrl = {
                        // TODO: Parse error.
                        while self.context.local_name != LocalName::Select {
                            self.pop_element();
                        }
                        self.pop_element();
                        self.reset_insertion_mode_appropriately();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTemplate) => {
                    let ctrl = {
                        // TODO: Pop the current template insertion mode off the stack of template insertion modes.
                        // TODO: Push "in body" onto the stack of template insertion modes so that it is the new current template insertion mode.
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterBody, AfterAfterBody) => {
                    let ctrl = {
                        // TODO: Parse error.
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(Text) => {
                    unreachable!();
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_end_table(&mut self, tag: &Tag<'_>) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode, ?tag);
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if !self.iframe_srcdoc {
                            // TODO: Parse error.
                        }
                        self.change_quirks_mode_if_changeable(QuirksMode::Quirks);
                        self.switch_to(mode!(BeforeHtml));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(
                    BeforeHtml,
                    BeforeHead,
                    InHead,
                    InHeadNoscript,
                    AfterHead,
                    InSelect,
                    InTemplate,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterFrameset
                ) => {
                    let ctrl = {
                        // TODO: Parse error.
                        // Ignore the token.
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InBody, InCell) => {
                    let ctrl = {
                        // TODO
                        self.pop_element();
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(Text) => {
                    let ctrl = {
                        self.pop_element();
                        self.switch_to_original_mode();
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTable) => {
                    let ctrl = {
                        // TODO
                        // TODO: Parse error.
                        self.enable_foster_parenting();
                        let ctrl = {
                            // TODO
                            self.pop_element();
                            Control::Continue
                        };
                        self.disable_foster_parenting();
                        ctrl
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        // TODO
                        if !self.text.is_empty() {
                            self.insert_text_to_foster_parent();
                        }
                        self.switch_to_original_mode();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InCaption) => {
                    let ctrl = {
                        // TODO
                        self.switch_to(mode!(InTable));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InColumnGroup) => {
                    let ctrl = {
                        if self.context.local_name == LocalName::Colgroup {
                            // TODO: Parse error.
                            // Ignore the token.
                            Control::Continue
                        } else {
                            self.pop_element();
                            self.switch_to(mode!(InTable));
                            Control::Reprocess
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTableBody) => {
                    let ctrl = {
                        // TODO: If the stack of open elements does not have a tbody, thead, or tfoot element in table scope, this is a parse error; ignore the token.
                        self.clear_stack_back_to_table_body_context();
                        self.pop_element();
                        self.switch_to(mode!(InTable));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InRow) => {
                    let ctrl = {
                        // TODO: If the stack of open elements does not have a tr element in table scope, this is a parse error; ignore the token.
                        self.clear_stack_back_to_table_row_context();
                        debug_assert_eq!(self.context.local_name, LocalName::Tr);
                        self.pop_element();
                        self.switch_to(mode!(InTableBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InSelectInTable) => {
                    let ctrl = {
                        // TODO: Parse error.
                        // TODO: If the stack of open elements does not have an element in table scope that is an HTML element with the same tag name as that of the token, then ignore the token.
                        while self.context.local_name != LocalName::Select {
                            self.pop_element();
                        }
                        self.pop_element();
                        self.reset_insertion_mode_appropriately();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterBody, AfterAfterBody) => {
                    let ctrl = {
                        // TODO: Parse error.
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
            }
        }
    }
}
