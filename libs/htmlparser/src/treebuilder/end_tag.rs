use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_end_tag(&mut self, tag: Tag<'_>) -> Control {
        tracing::debug!(?tag);
        loop {
            tracing::debug!(?self.insertion_mode);
            match self.insertion_mode {
                InsertionMode::Initial => {
                    // TODO: If the document is not an iframe srcdoc document, then this is a parse error
                    // TODO: if the parser cannot change the mode flag is false, set the Document to quirks mode.
                    self.switch_to(InsertionMode::BeforeHtml);
                    // Reprocess the token.
                }
                InsertionMode::BeforeHtml => match tag.name {
                    tag!(HEAD, BODY, HTML, BR) => {
                        // TODO: Create an html element whose node document is the Document object
                        // TODO: Append it to the Document object. Put this element in the stack of open elements.
                        self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                        self.switch_to(InsertionMode::BeforeHead);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                },
                InsertionMode::BeforeHead => match tag.name {
                    tag!(HEAD, BODY, HTML, BR) => {
                        // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                        // TODO: Set the head element pointer to the newly created head element.
                        self.switch_to(InsertionMode::InHead);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                },
                InsertionMode::InHead => match tag.name {
                    tag!(HEAD) => {
                        // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                        self.pop();
                        self.switch_to(InsertionMode::AfterHead);
                        return Control::Continue;
                    }
                    tag!(BODY, HTML, BR) => {
                        // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                        self.pop();
                        self.switch_to(InsertionMode::AfterHead);
                        // Reprocess the token.
                    }
                    tag!(TEMPLATE) => {
                        // TODO: If there is no template element on the stack of open elements, then this is a parse error; ignore the token.
                        // TODO: Otherwise, run these steps:
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                },
                InsertionMode::InHeadNoscript => match tag.name {
                    tag!(NOSCRIPT) => {
                        // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                        self.pop();
                        self.switch_to(InsertionMode::InHead);
                        return Control::Continue;
                    }
                    tag!(BR) => {
                        // TODO: Parse error.
                        // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                        self.pop();
                        self.switch_to(InsertionMode::InHead);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                },
                InsertionMode::AfterHead => match tag.name {
                    tag!(TEMPLATE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        return Control::Continue;
                    }
                    tag!(BODY, HTML, BR) => {
                        // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                        self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                        self.switch_to(InsertionMode::InBody);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                },
                InsertionMode::InBody => match tag.name {
                    tag!(TEMPLATE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        return Control::Continue;
                    }
                    tag!(BODY) => {
                        // TODO: If the stack of open elements does not have a body element in scope, this is a parse error; ignore the token.
                        // TODO: Otherwise
                        self.pop();
                        self.switch_to(InsertionMode::AfterBody);
                        return Control::Continue;
                    }
                    tag!(HTML) => {
                        // TODO: If the stack of open elements does not have a body element in scope, this is a parse error; ignore the token.
                        // TODO: Otherwise
                        self.switch_to(InsertionMode::AfterBody);
                        return Control::Continue;
                    }
                    // TODO: Introduce tag classes
                    tag!(
                        ADDRESS, ARTICLE, ASIDE, BLOCKQUOTE, BUTTON, CENTER, DETAILS, DIALOG, DIR,
                        DIV, DL, FIELDSET, FIGCAPTION, FIGURE, FOOTER, HEADER, HGROUP, MAIN, MENU,
                        NAV, OL, PRE, SECTION, SUMMARY, UL
                    ) => {
                        // TODO: If the stack of open elements does not have an element in scope that is an HTML element with the same tag name as that of the token, then this is a parse error; ignore the token.
                        // TODO: Otherwise
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(FORM) => {
                        // TODO
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(P) => {
                        // TODO: If the stack of open elements does not have a p element in button scope, then this is a parse error; insert an HTML element for a "p" start tag token with no attributes.
                        // TODO: Close a p element.
                        return Control::Continue;
                    }
                    tag!(LI) => {
                        // TODO
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(DD, DT) => {
                        // TODO
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(H1, H2, H3, H4, H5, H6) => {
                        // TODO
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(SARCASM) => {
                        // TODO: Take a deep breath, then act as described in the "any other end tag" entry below.
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(A, B, BIG, CODE, EM, FONT, I, NOBR, S, SMALL, STRIKE, STRONG, TT, U) => {
                        // TODO: Run the adoption agency algorithm for the token.
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(APPLET, MARQUEE, OBJECT) => {
                        // TODO
                        self.pop();
                        return Control::Continue;
                    }
                    tag!(BR) => {
                        // TODO: Parse error. Drop the attributes from the token, and act as described in the next entry; i.e. act as if this was a "br" start tag token with no attributes, rather than the end tag token that it actually is.
                        return Control::Continue;
                    }
                    _ => {
                        // TODO
                        self.pop();
                        return Control::Continue;
                    }
                },
                InsertionMode::Text => match tag.name {
                    tag!(SCRIPT) => {
                        // TODO
                        self.pop();
                        self.switch_to_original_mode();
                        return Control::Continue;
                    }
                    _ => {
                        self.pop();
                        self.switch_to_original_mode();
                        return Control::Continue;
                    }
                },
                _ => {
                    // TODO
                    self.pop();
                    return Control::Continue;
                }
            }
        }
    }
}
