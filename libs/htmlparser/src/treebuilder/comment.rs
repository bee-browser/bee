// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// bee-tools-codegen.js --no-escape --input-stdin comment.rs.hbs

use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_comment(&mut self, comment: Comment<'_>) -> Control {
        self.ignore_lf = false;
        loop {
            tracing::debug!(mode = ?self.mode);
            match self.mode {
                mode!(
                    Initial,
                    BeforeHead,
                    InHead,
                    InHeadNoscript,
                    AfterHead,
                    InBody,
                    InTable,
                    InCaption,
                    InColumnGroup,
                    InTableBody,
                    InRow,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    InTemplate,
                    InFrameset,
                    AfterFrameset
                ) => {
                    let ctrl = {
                        self.insert_comment(&comment);
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(BeforeHtml) => {
                    let ctrl = {
                        //debug_assert!(self.writer.is_empty());
                        self.insert_comment(&comment);
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(InTableText) => {
                    let ctrl = {
                        if self.pending_table_text_contains_non_whitespace {
                            // TODO: Parse error.
                            tracing::debug!("Parse error");
                            self.enable_foster_parenting();
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
                        _ => return ctrl,
                    }
                }
                mode!(AfterBody) => {
                    let ctrl = {
                        self.append_comment_to_document_element(&comment);
                        Control::Continue
                    };
                    match ctrl {
                        Control::Reprocess => continue,
                        _ => return ctrl,
                    }
                }
                mode!(AfterAfterBody, AfterAfterFrameset) => {
                    let ctrl = {
                        self.append_comment_to_document(&comment);
                        Control::Continue
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
    fn append_comment_to_document_element(&mut self, comment: &Comment<'_>) {
        self.append_text_if_exists();
        let node = self.inner.create_comment(comment.data);
        let parent = self.context_stack[1].open_element.node;
        self.inner.append_child(parent, node);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn append_comment_to_document(&mut self, comment: &Comment<'_>) {
        self.append_text_if_exists();
        let node = self.inner.create_comment(comment.data);
        let document = self.inner.get_document();
        self.inner.append_child(document, node);
    }
}
