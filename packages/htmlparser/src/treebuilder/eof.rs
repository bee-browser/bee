// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// codegen.js --no-escape --input-stdin eof.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_eof(&mut self) -> Control {
        loop {
            tracing::debug!(mode = ?self.mode);
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if !self.iframe_srcdoc {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                        }
                        self.change_quirks_mode_if_changeable(QuirksMode::Quirks);
                        self.switch_to(mode!(BeforeHtml));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(BeforeHtml) => {
                    let ctrl = {
                        //debug_assert!(self.writer.is_empty());
                        self.push_html_html_element(&Tag::with_no_attrs("html"));
                        self.switch_to(mode!(BeforeHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(BeforeHead) => {
                    let ctrl = {
                        self.push_html_head_element(&Tag::with_no_attrs("head"));
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InHead) => {
                    let ctrl = {
                        debug_assert!(self.context().is_html_element(tag!(Head)));
                        self.pop_element();
                        self.switch_to(mode!(AfterHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InHeadNoscript) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        debug_assert!(self.context().is_html_element(tag!(Noscript)));
                        self.pop_element();
                        debug_assert!(self.context().is_html_element(tag!(Head)));
                        self.switch_to(mode!(InHead));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(AfterHead) => {
                    let ctrl = {
                        self.push_html_body_element(&Tag::with_no_attrs("body"));
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(
                    InBody,
                    InTable,
                    InCaption,
                    InColumnGroup,
                    InTableBody,
                    InRow,
                    InCell,
                    InSelect,
                    InSelectInTable
                ) => {
                    let ctrl = {
                        if !self.template_mode_stack.is_empty() {
                            if !self.context().has_template_element() {
                                // TODO: Stop parsing.
                                Control::Done
                            } else {
                                // TODO: Parse error.
                                tracing::debug!("Parse error");
                                while !self.context().is_html_element(tag!(Template)) {
                                    self.pop_element();
                                }
                                self.pop_element(); // pop an html template element
                                self.active_formatting_element_list
                                    .clear_up_to_last_marker();
                                self.pop_template_mode();
                                self.reset_insertion_mode_appropriately();
                                Control::Reprocess
                            }
                        } else {
                            // TODO: If there is a node in the stack of open elements that is not either a dd element, a dt element, an li element, an optgroup element, an option element, a p element, an rb element, an rp element, an rt element, an rtc element, a tbody element, a td element, a tfoot element, a th element, a thead element, a tr element, the body element, or the html element, then this is a parse error.
                            // TODO: Stop parsing.
                            Control::Done
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(Text) => {
                    let ctrl = {
                        // TODO: Parse error.
                        tracing::debug!("Parse error");
                        if self.context().is_html_element(tag!(Script)) {
                            // TODO: set its already started to true.
                        }
                        self.pop_element();
                        self.switch_to_original_mode();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        if self.pending_table_text_contains_non_whitespace {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            self.enable_foster_parenting();
                            self.reconstruct_active_formatting_elements();
                            let node = self.inner.create_text(self.pending_table_text.as_str());
                            self.insert_node(node);
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                            self.disable_foster_parenting();
                        } else {
                            let node = self.inner.create_text(self.pending_table_text.as_str());
                            self.insert_node(node);
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                        }
                        self.switch_to_original_mode();
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InTemplate) => {
                    let ctrl = {
                        if !self.context().has_template_element() {
                            // TODO: Stop parsing.
                            Control::Done
                        } else {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            while !self.context().is_html_element(tag!(Template)) {
                                self.pop_element();
                            }
                            self.pop_element(); // pop an html template element
                            self.active_formatting_element_list
                                .clear_up_to_last_marker();
                            self.pop_template_mode();
                            self.reset_insertion_mode_appropriately();
                            Control::Reprocess
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(AfterBody, AfterFrameset, AfterAfterBody, AfterAfterFrameset) => {
                    let ctrl = { Control::Done };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InFrameset) => {
                    let ctrl = {
                        if !self.context().is_html_element(tag!(Html)) {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                        }
                        Control::Done
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
            }
        }
        self.end();
        Control::Done
    }
}
