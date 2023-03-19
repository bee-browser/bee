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
                    AfterBody,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterBody,
                    AfterAfterFrameset
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
                mode!(Text) => {
                    unreachable!();
                }
            }
        }
    }

    // Implement the "Insert a comment" algorithm.
    fn insert_comment(&mut self, comment: &Comment<'_>) {
        // TODO
        self.append_comment(comment);
    }
}
