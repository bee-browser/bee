use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_comment(&mut self, comment: Comment<'_>) -> Control {
        tracing::debug!(?comment);
        match self.insertion_mode {
            InsertionMode::BeforeHead
            | InsertionMode::InHead
            | InsertionMode::InHeadNoscript
            | InsertionMode::AfterHead
            | InsertionMode::InBody
            | InsertionMode::InTable
            | InsertionMode::InCaption
            | InsertionMode::InColumnGroup
            | InsertionMode::InTableBody
            | InsertionMode::InRow
            | InsertionMode::InCell
            | InsertionMode::InSelect
            | InsertionMode::InSelectInTable
            | InsertionMode::InTemplate
            | InsertionMode::InFrameset
            | InsertionMode::AfterFrameset
            | InsertionMode::InForeignContent => {
                self.insert_comment(&comment);
            }
            InsertionMode::Initial
            | InsertionMode::BeforeHtml
            | InsertionMode::AfterAfterBody
            | InsertionMode::AfterAfterFrameset => {
                self.append_comment(&comment);
            }
            InsertionMode::AfterBody => {
                self.append_comment_to_root_element(&comment);
            }
            InsertionMode::Text | InsertionMode::InTableText => {
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
