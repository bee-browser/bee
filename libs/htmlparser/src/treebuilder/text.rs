use super::*;

macro_rules! char_class {
    ($c:literal) => {
        $c
    };
    ($c:literal, $($more:literal),+) => {
        char_class!($c) | char_class!($($more),+)
    }
}

macro_rules! whitespace {
    () => {
        char_class!['\u{0009}', '\u{000A}', '\u{000C}', '\u{000D}', '\u{0020}']
    };
}

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
                mode!(Initial) => {
                    self.switch_to(mode!(BeforeHtml));
                    // reconsume
                }
                mode!(BeforeHtml) => {
                    // TODO: Create an html element whose node document is the Document object.
                    // TODO: Append it to the Document object.
                    // TODO: Put this element in the stack of open elements
                    self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                    self.switch_to(mode!(BeforeHead));
                    // reconsume
                }
                mode!(BeforeHead) => {
                    // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                    // TODO: Set the head element pointer to the newly created head element.
                    self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                    self.switch_to(mode!(InHead));
                    // reconsume
                }
                mode!(InHead) => {
                    // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                    self.pop();
                    self.switch_to(mode!(AfterHead));
                    // reconsume
                }
                mode!(InHeadNoscript) => {
                    // TODO: Parse error.
                    // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                    self.switch_to(mode!(InHead));
                    // reconsume
                }
                mode!(AfterHead) => {
                    // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                    self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                    self.switch_to(mode!(InBody));
                    // reconsume
                }
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
                mode!(InColumnGroup) => {
                    // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                    // TODO: Otherwise, pop the current node from the stack of open elements.
                    self.switch_to(mode!(InTable));
                    // reconsume
                }
                mode!(AfterBody, AfterAfterBody) => {
                    // TODO: Parse error.
                    self.switch_to(mode!(InBody));
                    // reconsume
                }
                mode!(InForeignContent) => {
                    // TODO: Parse error.
                    self.append_char('\u{FFFD}');
                    return;
                }
            }
        }
    }

    fn handle_whitespace(&mut self, c: char) {
        tracing::debug!(mode = ?self.mode, c = "Whitespace");
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
                AfterFrameset,
                InForeignContent
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
                mode!(Initial) => {
                    self.switch_to(mode!(BeforeHtml));
                    // reconsume
                }
                mode!(BeforeHtml) => {
                    // TODO: Create an html element whose node document is the Document object.
                    // TODO: Append it to the Document object.
                    // TODO: Put this element in the stack of open elements
                    self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                    self.switch_to(mode!(BeforeHead));
                    // reconsume
                }
                mode!(BeforeHead) => {
                    // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                    // TODO: Set the head element pointer to the newly created head element.
                    self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                    self.switch_to(mode!(InHead));
                    // reconsume
                }
                mode!(InHead) => {
                    // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                    self.pop();
                    self.switch_to(mode!(AfterHead));
                    // reconsume
                }
                mode!(InHeadNoscript) => {
                    // TODO: Parse error.
                    // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                    self.switch_to(mode!(InHead));
                    // reconsume
                }
                mode!(AfterHead) => {
                    // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                    self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                    self.switch_to(mode!(InBody));
                    // reconsume
                }
                mode!(InBody, InCaption, InCell, InTemplate) => {
                    // TODO: Reconstruct the active formatting elements, if any.
                    // TODO: Insert the token's character.
                    // TODO: Set the frameset-ok flag to "not ok".
                    self.append_char(c);
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
                mode!(InColumnGroup) => {
                    // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                    // TODO: Otherwise, pop the current node from the stack of open elements.
                    self.switch_to(mode!(InTable));
                    // reconsume
                }
                mode!(InSelect, InSelectInTable) => {
                    // TODO: Insert the token's character.
                    self.append_char(c);
                    return;
                }
                mode!(AfterBody, AfterAfterBody) => {
                    // TODO: Parse error.
                    self.switch_to(mode!(InBody));
                    // reconsume
                }
                mode!(InFrameset, AfterFrameset, AfterAfterFrameset) => {
                    // TODO: Parse error.
                    // Ignore the token.
                    return;
                }
                mode!(InForeignContent) => {
                    // TODO: Insert the token's character.
                    // TODO: Set the frameset-ok flag to "not ok".
                    self.append_char(c);
                    return;
                }
            }
        }
    }
}
