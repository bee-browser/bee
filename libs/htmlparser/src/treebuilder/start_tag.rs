use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_start_tag(&mut self, tag: Tag<'_>) -> Control {
        tracing::debug!(?tag);
        loop {
            tracing::debug!(?self.insertion_mode);
            match self.insertion_mode {
                InsertionMode::Initial => {
                    // TODO: If the document is not an iframe srcdoc document, then this is a parse error;
                    // TODO: if the parser cannot change the mode flag is false, set the Document to quirks mode.
                    self.switch_to(InsertionMode::BeforeHtml);
                    // Reprocess the token.
                }
                InsertionMode::BeforeHtml => match tag.name {
                    tag!(HTML) => {
                        // TODO: Create an element for the token in the HTML namespace, with the Document as the intended parent.
                        // TODO: Append it to the Document object. Put this element in the stack of open elements.
                        self.push_element(&tag);
                        self.switch_to(InsertionMode::BeforeHead);
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Create an html element whose node document is the Document object.
                        // TODO: Append it to the Document object. Put this element in the stack of open elements.
                        self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                        self.switch_to(InsertionMode::BeforeHead);
                        // Reprocess the token.
                    }
                },
                InsertionMode::BeforeHead => match tag.name {
                    tag!(HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    tag!(HEAD) => {
                        // TODO: Insert an HTML element for the token.
                        self.push_element(&tag);
                        // TODO: Set the head element pointer to the newly created head element.
                        self.switch_to(InsertionMode::InHead);
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                        // TODO: Set the head element pointer to the newly created head element.
                        self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                        self.switch_to(InsertionMode::InHead);
                        // Reprocess the token.
                    }
                },
                InsertionMode::InHead => match tag.name {
                    // TODO: We can improve the performance of the following pattern matching by introducing flags.
                    tag!(HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    tag!(BASE, BASEFONT, BGSOUND, LINK) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Immediately pop the current node off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                        return Control::Continue;
                    }
                    tag!(META) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Immediately pop the current node off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                        // TODO: active speculative HTML parser
                        return Control::Continue;
                    }
                    tag!(TITLE) => {
                        return self.apply_generic_rcdata_element_rule(&tag);
                    }
                    tag!(NOFRAMES, STYLE) => {
                        // TODO: Follow the generic raw text element parsing algorithm.
                        return Control::Continue;
                    }
                    tag!(NOSCRIPT) /* if the scripting flag is enabled */ => {
                        if /* the scripting flag is enabled */ true {
                            // TODO: Follow the generic raw text element parsing algorithm.
                        } else {
                            // TODO: Insert an HTML element for the token.
                            self.switch_to(InsertionMode::InHeadNoscript);
                        }
                        return Control::Continue;
                    }
                    tag!(SCRIPT) => {
                        // TODO
                        self.switch_to(InsertionMode::Text);
                        return Control::Continue;
                    }
                    tag!(TEMPLATE) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(HEAD) => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                        self.pop();
                        self.switch_to(InsertionMode::AfterHead);
                        // Reprocess the token.
                    }
                },
                InsertionMode::InHeadNoscript => match tag.name {
                    tag!(HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                        return Control::Continue;
                    }
                    tag!(HEAD, NOSCRIPT) => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Parse error.
                        // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                        self.switch_to(InsertionMode::InHead);
                        // Reprocess the token.
                    }
                },
                InsertionMode::AfterHead => match tag.name {
                    tag!(HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                        return Control::Continue;
                    }
                    tag!(BODY) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Set the frameset-ok flag to "not ok".
                        self.push_element(&tag);
                        self.switch_to(InsertionMode::InBody);
                    }
                    tag!(FRAMESET) => {
                        // TODO: Insert an HTML element for the token.
                        self.switch_to(InsertionMode::InFrameset);
                    }
                    tag!(
                        BASE, BASEFONT, BGSOUND, LINK, META, NOFRAMES, SCRIPT, STYLE, TEMPLATE,
                        TITLE
                    ) => {
                        // TODO: Parse error.
                        // TODO: Push the node pointed to by the head element pointer onto the stack of open elements.
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        // TODO: Remove the node pointed to by the head element pointer from the stack of open elements. (It might not be the current node at this point.)
                        return Control::Continue;
                    }
                    tag!(HEAD) => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                        self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                        self.switch_to(InsertionMode::InBody);
                        // Reprocess the token.
                    }
                },
                InsertionMode::InBody => match tag.name {
                    tag!(HTML) => {
                        // TODO: Parse error.
                        // TODO: If there is a template element on the stack of open elements, then ignore the token.
                        // TODO: Otherwise, for each attribute on the token, check to see if the attribute is already present on the top element of the stack of open elements. If it is not, add the attribute and its corresponding value to that element.
                        return Control::Continue;
                    }
                    tag!(
                        BASE, BASEFONT, BGSOUND, LINK, META, NOFRAMES, SCRIPT, STYLE, TEMPLATE,
                        TITLE
                    ) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        return Control::Continue;
                    }
                    tag!(BODY) => {
                        // TODO: Parse error.
                        // TODO: If the second element on the stack of open elements is not a body element, if the stack of open elements has only one node on it, or if there is a template element on the stack of open elements, then ignore the token. (fragment case)
                        // TODO: Otherwise, set the frameset-ok flag to "not ok"; then, for each attribute on the token, check to see if the attribute is already present on the body element (the second element) on the stack of open elements, and if it is not, add the attribute and its corresponding value to that element.
                        return Control::Continue;
                    }
                    tag!(FRAMESET) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(
                        ADDRESS, ARTICLE, ASIDE, BLOCKQUOTE, CENTER, DETAILS, DIALOG, DIR, DIV, DL,
                        FIELDSET, FIGCAPTION, FIGURE, FOOTER, HEADER, HGROUP, MAIN, MENU, NAV, OL,
                        P, SECTION, SUMMARY, UL
                    ) => {
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: Insert an HTML element for the token.
                        return Control::Continue;
                    }
                    tag!(H1, H2, H3, H4, H5, H6) => {
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: If the current node is an HTML element whose tag name is one of "h1", "h2", "h3", "h4", "h5", or "h6", then this is a parse error; pop the current node off the stack of open elements.
                        // TODO: Insert an HTML element for the token.
                        return Control::Continue;
                    }
                    tag!(PRE) | TagKind::Other("listing") => {
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: Insert an HTML element for the token.
                        // TODO: If the next token is a U+000A LINE FEED (LF) character token, then ignore that token and move on to the next one. (Newlines at the start of pre blocks are ignored as an authoring convenience.)
                        // TODO: Set the frameset-ok flag to "not ok".
                        return Control::Continue;
                    }
                    tag!(FORM) => {
                        // TODO: If the form element pointer is not null, and there is no template element on the stack of open elements, then this is a parse error; ignore the token.
                        // TODO: Otherwise:
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: Insert an HTML element for the token, and, if there is no template element on the stack of open elements, set the form element pointer to point to the element created.
                        return Control::Continue;
                    }
                    tag!(LI) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(DD, DT) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(PLAINTEXT) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(BUTTON) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(A) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(B, BIG, CODE, EM, FONT, I, S, SMALL, STRIKE, STRONG, TT, U) => {
                        // TODO
                        self.push_element(&tag);
                        return Control::Continue;
                    }
                    tag!(NOBR) => {
                        // TODO
                        return Control::Continue;
                    }
                    tag!(APPLET, MARQUEE, OBJECT) => {
                        // TODO
                        return Control::Continue;
                    }
                    _ => {
                        // TODO: Reconstruct the active formatting elements, if any.
                        // TODO: Insert an HTML element for the token.
                        self.push_element(&tag);
                        return Control::Continue;
                    }
                },
                InsertionMode::Text => {
                    unreachable!();
                }
                InsertionMode::InTable => match tag.name {
                    tag!(CAPTION) => {
                        // TODO: Clear the stack back to a table context.
                        // TODO: Insert a marker at the end of the list of active formatting elements.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InCaption);
                        return Control::Continue;
                    }
                    tag!(COLGROUP) => {
                        // TODO: Clear the stack back to a table context. (See below.)
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InColumnGroup);
                        return Control::Continue;
                    }
                    tag!(COL) => {
                        // TODO: Clear the stack back to a table context. (See below.)
                        // TODO: Insert an HTML element for a "colgroup" start tag token with no attributes
                        self.switch_to(InsertionMode::InColumnGroup);
                        // Reprocess the current token.
                    }
                    tag!(TBODY, TFOOT, THEAD) => {
                        // TODO: Clear the stack back to a table context.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InTableBody);
                        return Control::Continue;
                    }
                    tag!(TD, TH, TR) => {
                        // TODO: Clear the stack back to a table context.
                        // TODO: Insert an HTML element for a "tbody" start tag token with no attributes
                        self.switch_to(InsertionMode::InTableBody);
                        // Reprocess the current token.
                    }
                    tag!(TABLE) => {
                        // TODO: Parse error.
                        // TODO: If the stack of open elements does not have a table element in table scope, ignore the token.
                        // TODO: Otherwise:
                        // TODO: Pop elements from this stack until a table element has been popped from the stack.
                        // TODO: Reset the insertion mode appropriately.
                        // Reprocess the current token.
                    }
                    tag!(STYLE, SCRIPT, TEMPLATE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        return Control::Continue;
                    }
                    tag!(INPUT) => {
                        // TODO: If the token does not have an attribute with the name "type", or if it does, but that attribute's value is not an ASCII case-insensitive match for the string "hidden", then: act as described in the "anything else" entry below.
                        // TODO: Otherwise:
                        // TODO: Parse error.
                        // TODO: Insert an HTML element for the token.
                        // TODO: Pop that input element off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                    }
                    tag!(FORM) => {
                        // TODO: Parse error.
                        // TODO: If there is a template element on the stack of open elements, or if the form element pointer is not null, ignore the token.
                        // TODO: Otherwise:
                        // TODO: Insert an HTML element for the token, and set the form element pointer to point to the element created.
                        // TODO: Pop that form element off the stack of open elements.
                    }
                    _ => {
                        // TODO: Parse error.
                        // TODO: Enable foster parenting, process the token using the rules for the "in body" insertion mode, and then disable foster parenting.
                        return Control::Continue;
                    }
                },
                InsertionMode::InTableText => {
                    // TODO: If any of the tokens in the pending table character tokens list are character tokens that are not ASCII whitespace, then this is a parse error: reprocess the character tokens in the pending table character tokens list using the rules given in the "anything else" entry in the "in table" insertion mode.
                    // TODO: Otherwise, insert the characters given by the pending table character tokens list.
                    // TODO: Switch the insertion mode to the original insertion mode and reprocess the token.
                }
                InsertionMode::InCaption => match tag.name {
                    tag!(CAPTION, COL, COLGROUP, TBODY, TD, TFOOT, TH, THEAD, TR) => {
                        // TODO: If the stack of open elements does not have a caption element in table scope, this is a parse error; ignore the token. (fragment case)
                        // TODO: Otherwise:
                        // TODO: Generate implied end tags.
                        // TODO: Now, if the current node is not a caption element, then this is a parse error.
                        // TODO: Pop elements from this stack until a caption element has been popped from the stack.
                        // TODO: Clear the list of active formatting elements up to the last marker.
                        self.switch_to(InsertionMode::InTable);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                },
                InsertionMode::InColumnGroup => match tag.name {
                    tag!(HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    tag!(COL) => {
                        // TODO: Insert an HTML element for the token. Immediately pop the current node off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                    }
                    tag!(TEMPLATE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                    }
                    _ => {
                        // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                        // TODO: Otherwise, pop the current node from the stack of open elements.
                        self.switch_to(InsertionMode::InTable);
                        // Reprocess the token.
                    }
                },
                InsertionMode::InTableBody => match tag.name {
                    tag!(TR) => {
                        // TODO: Clear the stack back to a table body context.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InRow);
                        return Control::Continue;
                    }
                    tag!(TH, TD) => {
                        // TODO: Parse error.
                        // TODO: Clear the stack back to a table body context.
                        // TODO: Insert an HTML element for a "tr" start tag token with no attributes
                        self.switch_to(InsertionMode::InRow);
                        // Reprocess the current token.
                    }
                    tag!(CAPTION, COL, COLGROUP, TBODY, TFOOT, THEAD) => {
                        // TODO: If the stack of open elements does not have a tbody, thead, or tfoot element in table scope, this is a parse error; ignore the token.
                        // TODO: Otherwise:
                        // TODO: Clear the stack back to a table body context. (See below.)
                        // TODO: Pop the current node from the stack of open elements.
                        self.switch_to(InsertionMode::InTable);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Process the token using the rules for the "in table" insertion mode.
                    }
                },
                InsertionMode::InRow => match tag.name {
                    tag!(TH, TD) => {
                        // TODO: Clear the stack back to a table row context.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InCell);
                        // TODO: Insert a marker at the end of the list of active formatting elements.
                        return Control::Continue;
                    }
                    tag!(CAPTION, COL, COLGROUP, TBODY, TFOOT, THEAD, TR) => {
                        // TODO: If the stack of open elements does not have a tr element in table scope, this is a parse error; ignore the token.
                        // TODO: Otherwise:
                        // TODO: Clear the stack back to a table row context. (See below.)
                        // TODO: Pop the current node (which will be a tr element) from the stack of open elements.
                        self.switch_to(InsertionMode::InTableBody);
                        // Reprocess the token.
                    }
                    _ => {
                        // TODO: Process the token using the rules for the "in table" insertion mode.
                    }
                },
                _ => {}
            }
        }
    }

    fn apply_generic_rcdata_element_rule(&mut self, tag: &Tag<'_>) -> Control {
        self.push_element(tag);
        self.save_and_switch_to(InsertionMode::Text);
        Control::SwitchTo(bee_htmltokenizer::InitialState::Rcdata)
    }
}
