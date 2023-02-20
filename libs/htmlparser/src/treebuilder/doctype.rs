use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_doctype(&mut self, doctype: Doctype<'_>) -> Control {
        tracing::debug!(mode = ?self.mode, ?doctype);
        match self.mode {
            mode!(Initial) => {
                if let Some("html") = doctype.name {
                    // TODO: parse error
                } else if let Some(_) = doctype.public_id {
                    // TODO: parse error
                } else if let Some(system_id) = doctype.system_id {
                    if system_id != "about:legacy-compat" {
                        // TODO: parse error
                    }
                }
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
