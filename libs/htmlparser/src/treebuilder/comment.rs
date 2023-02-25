use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_comment(&mut self, comment: Comment<'_>) -> Control {
        tracing::debug!(mode = ?self.mode, ?comment);
        self.ignore_lf = false;
        match self.mode {
            mode!(
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
                self.insert_comment(&comment);
            }
            mode!(Initial, BeforeHtml, AfterAfterBody, AfterAfterFrameset) => {
                self.append_comment(&comment);
            }
            mode!(AfterBody) => {
                self.append_comment_to_root_element(&comment);
            }
            mode!(Text, InTableText) => {
                unreachable!();
            }
        }
        Control::Continue
    }

    // Implement the "Insert a comment" algorithm.
    fn insert_comment(&mut self, comment: &Comment<'_>) {
        // TODO
        self.append_comment(comment);
    }

    // Implement the "TODO: Insert a comment as the last child of the first element in the stack of open elements (the html element)" algorithm.
    fn append_comment_to_root_element(&mut self, comment: &Comment<'_>) {
        // TODO
        self.append_comment(comment);
    }
}
