use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_start_style(&mut self, tag: &Tag<'_>) -> Control {
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
                mode!(
                    InHead,
                    InHeadNoscript,
                    InBody,
                    InTable,
                    InCaption,
                    InTableBody,
                    InRow,
                    InCell,
                    InTemplate
                ) => {
                    let ctrl = {
                        self.push_html_element(tag);
                        self.save_and_switch_to(mode!(Text));
                        Control::SwitchTo(bee_htmltokenizer::InitialState::Rawtext, "style")
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterHead) => {
                    let ctrl = {
                        // TODO: Parse error.
                        self.reopen_head_element();
                        self.push_html_element(tag);
                        self.save_and_switch_to(mode!(Text));
                        Control::SwitchTo(bee_htmltokenizer::InitialState::Rawtext, "style")
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
                        if self.context.local_name != LocalName::Colgroup {
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
                mode!(
                    InSelect,
                    InSelectInTable,
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
    pub fn handle_end_style(&mut self, tag: &Tag<'_>) -> Control {
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
                    InSelectInTable,
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
                mode!(InBody, InCaption, InCell) => {
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
                mode!(InTable, InTableBody, InRow) => {
                    let ctrl = {
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
                mode!(InColumnGroup) => {
                    let ctrl = {
                        if self.context.local_name != LocalName::Colgroup {
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
