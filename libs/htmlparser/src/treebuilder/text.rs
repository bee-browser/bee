// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/htmlparser/src/treebuilder/text.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn handle_text(&mut self, text: Text<'_>) -> Control {
        if self.ignore_lf {
            self.ignore_lf = false;
        }
        loop {
            match self.mode {
                mode!(Initial) => {
                    let ctrl = {
                        if !self.iframe_srcdoc {
                            // TODO: Parse error.
                            logger::debug!("Parse error");
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
                        logger::debug!("Parse error");
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
                mode!(InBody, InCaption, InCell, InTemplate) => {
                    let ctrl = {
                        self.reconstruct_active_formatting_elements();
                        self.append_text(text.data);
                        self.frameset_ok = false;
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(Text, InSelect, InSelectInTable) => {
                    let ctrl = {
                        self.append_text(text.data);
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InTable, InTableBody, InRow) => {
                    let ctrl = {
                        if self.context().is_one_of_html_elements(&tags![
                            Table, Tbody, Template, Tfoot, Thead, Tr
                        ]) {
                            self.append_text_if_exists();
                            self.pending_table_text.clear();
                            self.pending_table_text_contains_non_whitespace = false;
                            self.save_and_switch_to(mode!(InTableText));
                            Control::Reprocess
                        } else {
                            // TODO: Parse error.
                            logger::debug!("Parse error");
                            self.enable_foster_parenting();
                            let ctrl = {
                                self.reconstruct_active_formatting_elements();
                                self.append_text(text.data);
                                self.frameset_ok = false;
                                Control::Continue
                            };
                            self.disable_foster_parenting();
                            ctrl
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        self.pending_table_text.push_str(text.data);
                        self.pending_table_text_contains_non_whitespace = true;
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InColumnGroup) => {
                    let ctrl = {
                        if !self.context().is_html_element(tag!(Colgroup)) {
                            // TODO: Parse error.
                            logger::debug!("Parse error");
                            // Ignore the token.
                            logger::debug!("Ignore the token");
                            Control::Continue
                        } else {
                            self.pop_element();
                            self.switch_to(mode!(InTable));
                            Control::Reprocess
                        }
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(AfterBody, AfterAfterBody) => {
                    let ctrl = {
                        // TODO: Parse error.
                        logger::debug!("Parse error");
                        self.switch_to(mode!(InBody));
                        Control::Reprocess
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
                mode!(InFrameset, AfterFrameset, AfterAfterFrameset) => {
                    let ctrl = {
                        // TODO: Parse error.
                        logger::debug!("Parse error");
                        // Ignore the token.
                        logger::debug!("Ignore the token");
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => break,
                    }
                }
            }
        }
        Control::Continue
    }
}
