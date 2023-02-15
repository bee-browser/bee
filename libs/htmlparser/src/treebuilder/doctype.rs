use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_doctype(&mut self, doctype: Doctype<'_>) -> Control {
        tracing::debug!(?self.insertion_mode);
        tracing::debug!(?doctype);
        match self.insertion_mode {
            mode!(Initial) => {
                self.append_doctype(&doctype);
                self.determine_quirks_mode(&doctype);
                self.switch_to(mode!(BeforeHtml));
            }
            mode!(
                BeforeHtml,
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
                AfterAfterFrameset,
                InForeignContent
            ) => {
                // TODO: Parse error.
                // Ignore the token.
            }
            mode!(Text, InTableText) => {
                unreachable!();
            }
        }
        Control::Continue
    }

    fn determine_quirks_mode(&mut self, doctype: &Doctype<'_>) {
        // TODO
    }
}
