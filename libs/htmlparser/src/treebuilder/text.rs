use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_text(&mut self, text: Text<'_>) -> Control {
        for c in text.data.chars() {
            match c {
                '\0' => self.handle_null(),
                whitespace!() => self.handle_whitespace(c),
                _ => self.handle_other(c),
            }
        }
        Control::Continue
    }

    fn handle_null(&mut self) {
        loop {
            tracing::debug!(mode = ?self.mode, c = "NULL");
            match self.mode {
                mode!(
                    InBody,
                    InTableText,
                    InCaption,
                    InCell,
                    InSelect,
                    InSelectInTable,
                    InTemplate,
                    InFrameset,
                    AfterFrameset,
                    AfterAfterFrameset
                ) => {
                    // TODO: Parse error
                    // Ignore the token.
                    return;
                }
                mode!(Text) => {
                    self.append_char('\0');
                    return;
                }
                mode!(InTable, InTableBody, InRow) => {
                    // TODO
                    return;
                }
                _ => match self.handle_anything_else() {
                    Control::Reprocess => (),
                    _ => return,
                },
            }
        }
    }

    fn handle_whitespace(&mut self, c: char) {
        tracing::debug!(mode = ?self.mode, c = "Whitespace");
        if self.ignore_lf && c == '\n' {
            self.ignore_lf = false;
            return;
        }
        match self.mode {
            mode!(Initial, BeforeHtml, BeforeHead) => {
                // Ignore
            }
            mode!(
                InHead,
                InHeadNoscript,
                AfterHead,
                Text,
                InColumnGroup,
                InSelect,
                InSelectInTable,
                InFrameset,
                AfterFrameset
            ) => {
                self.append_char(c);
            }
            mode!(
                InBody,
                InCaption,
                InCell,
                InTemplate,
                AfterBody,
                AfterAfterBody,
                AfterAfterFrameset
            ) => {
                // TODO: Reconstruct the active formatting elements, if any.
                // TODO: Insert the token's character.
                self.append_char(c);
            }
            mode!(InTable, InTableBody, InRow) => {
                // TODO
            }
            mode!(InTableText) => {
                // TODO: Append the character token to the pending table character tokens list.
            }
        }
    }

    fn handle_other(&mut self, c: char) {
        loop {
            tracing::debug!(mode = ?self.mode, ?c);
            match self.mode {
                mode!(InBody, InCaption, InCell, InTemplate) => {
                    // TODO: Reconstruct the active formatting elements, if any.
                    // TODO: Insert the token's character.
                    self.append_char(c);
                    self.frameset_ok = false;
                    return;
                }
                mode!(Text) => {
                    self.append_char(c);
                    return;
                }
                mode!(InTable, InTableBody, InRow) => {
                    // TODO
                    return;
                }
                mode!(InTableText) => {
                    // TODO: Append the character token to the pending table character tokens list.
                    return;
                }
                mode!(InSelect, InSelectInTable) => {
                    // TODO: Insert the token's character.
                    self.append_char(c);
                    return;
                }
                _ => match self.handle_anything_else() {
                    Control::Reprocess => (),
                    _ => return,
                },
            }
        }
    }
}
