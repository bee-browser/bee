use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_start_html(&mut self, tag: &Tag<'_>) -> Control {
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
                        self.push_html_element(tag);
                        self.switch_to(mode!(BeforeHead));
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
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
                    let ctrl = {
                        // TODO: Parse error.
                        if self.has_template_element() {
                            // Ignore the token.
                        } else {
                            self.set_attributes_to_html_element(tag);
                        }
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
                        self.enable_foster_parenting();
                        let ctrl = {
                            // TODO: Parse error.
                            if self.has_template_element() {
                                // Ignore the token.
                            } else {
                                self.set_attributes_to_html_element(tag);
                            }
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
                mode!(Text) => {
                    unreachable!();
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_end_html(&mut self, tag: &Tag<'_>) -> Control {
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
                mode!(
                    InHeadNoscript,
                    InTable,
                    InCaption,
                    InTableBody,
                    InRow,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    InTemplate,
                    InFrameset,
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
                mode!(InBody) => {
                    let ctrl = {
                        // TODO
                        self.switch_to(mode!(AfterBody));
                        Control::Reprocess
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
                mode!(AfterBody) => {
                    let ctrl = {
                        // TODO: If the parser was created as part of the HTML fragment parsing algorithm, this is a parse error; ignore the token. (fragment case)
                        self.switch_to(mode!(AfterAfterBody));
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterFrameset) => {
                    let ctrl = {
                        self.switch_to(mode!(AfterAfterFrameset));
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterAfterBody) => {
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
