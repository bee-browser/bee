use bee_htmltags::HtmlTag;
use bee_htmltokenizer::TokenHandler;
use bee_htmltokenizer::TagKind;

pub(crate) struct TreeBuilder {
    insertion_mode: InsertionMode,
}

impl TreeBuilder {
    pub(crate) fn new() -> Self {
        TreeBuilder {
            insertion_mode: InsertionMode::Initial,
        }
    }

    fn is_valid_doctype(&mut self, name: Option<&str>, public_id: Option<&str>, system_id: Option<&str>, force_quirks: bool) -> bool {
        true
    }

    fn append_doctype(&mut self, name: Option<&str>, public_id: Option<&str>, system_id: Option<&str>, force_quirks: bool) {
    }

    fn determine_quirks_mode(&mut self, name: Option<&str>, public_id: Option<&str>, system_id: Option<&str>, force_quirks: bool) {
    }

    // Implement the "Insert a comment" algorithm.
    fn insert_comment(&mut self, comment: &str) {
    }

    // Implement the "Insert a comment as the last child of the Document object" algorithm.
    fn append_comment(&mut self, comment: &str) {
    }

    // Implement the "TODO: Insert a comment as the last child of the first element in the stack of open elements (the html element)" algorithm.
    fn append_comment_to_root_element(&mut self, comment: &str) {
    }

    fn switch_to(&mut self, insertion_mode: InsertionMode) {
        self.insertion_mode = insertion_mode;
    }
}

impl TokenHandler for TreeBuilder {
    fn handle_doctype(&mut self, name: Option<&str>, public_id: Option<&str>, system_id: Option<&str>, force_quirks: bool) -> bool {
        match self.insertion_mode {
            InsertionMode::Initial => {
                if !self.is_valid_doctype(name, public_id, system_id, force_quirks) {
                    // TODO: parse error
                }
                self.append_doctype(name, public_id, system_id, force_quirks);
                self.determine_quirks_mode(name, public_id, system_id, force_quirks);
                self.insertion_mode = InsertionMode::BeforeHtml;
            }
            InsertionMode::BeforeHtml |
            InsertionMode::BeforeHead |
            InsertionMode::InHead |
            InsertionMode::InHeadNoscript |
            InsertionMode::AfterHead |
            InsertionMode::InBody |
            InsertionMode::InTable |
            InsertionMode::InCaption |
            InsertionMode::InColumnGroup |
            InsertionMode::InTableBody |
            InsertionMode::InRow |
            InsertionMode::InCell |
            InsertionMode::InSelect |
            InsertionMode::InSelectInTable |
            InsertionMode::InTemplate |
            InsertionMode::AfterBody |
            InsertionMode::InFrameset |
            InsertionMode::AfterFrameset |
            InsertionMode::AfterAfterBody |
            InsertionMode::AfterAfterFrameset |
            InsertionMode::InForeignContent => {
                // TODO: Parse error.
                // Ignore the token.
            }
            InsertionMode::Text |
            InsertionMode::InTableText => {
                unreachable!();
            }
        }
        true
    }

    fn handle_start_tag(&mut self, name: TagKind, attrs: bee_htmltokenizer::Attrs<'_>, self_closing: bool) -> bool {
        loop {
            match self.insertion_mode {
                InsertionMode::Initial => {
                    // TODO: If the document is not an iframe srcdoc document, then this is a parse error;
                    // TODO: if the parser cannot change the mode flag is false, set the Document to quirks mode.
                    self.insertion_mode = InsertionMode::BeforeHtml;
                    // Reprocess the token.
                }
                InsertionMode::BeforeHtml => match name {
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Create an element for the token in the HTML namespace, with the Document as the intended parent.
                        // TODO: Append it to the Document object. Put this element in the stack of open elements.
                        self.insertion_mode = InsertionMode::BeforeHead;
                        return true;
                    }
                    _ => {
                        // TODO: Create an html element whose node document is the Document object.
                        // TODO: Append it to the Document object. Put this element in the stack of open elements.
                        self.insertion_mode = InsertionMode::BeforeHead;
                        // Reprocess the token.
                    }
                }
                InsertionMode::BeforeHead => match name {
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    TagKind::Html(HtmlTag::HEAD) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Set the head element pointer to the newly created head element.
                        self.insertion_mode = InsertionMode::InHead;
                        return true;
                    }
                    _ => {
                        // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                        // TODO: Set the head element pointer to the newly created head element.
                        self.insertion_mode = InsertionMode::InHead;
                        // Reprocess the token.
                    }
                }
                InsertionMode::InHead => match name {
                    // TODO: We can improve the performance of the following pattern matching by introducing flags.
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    TagKind::Html(HtmlTag::BASE) |
                    TagKind::Html(HtmlTag::BASEFONT) |
                    TagKind::Html(HtmlTag::BGSOUND) |
                    TagKind::Html(HtmlTag::LINK) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Immediately pop the current node off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                        return true;
                    }
                    TagKind::Html(HtmlTag::META) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Immediately pop the current node off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                        // TODO: active speculative HTML parser
                        return true;
                    }
                    TagKind::Html(HtmlTag::TITLE) => {
                        // TODO: Follow the generic RCDATA element parsing algorithm.
                        return true;
                    }
                    TagKind::Html(HtmlTag::NOFRAMES) |
                    TagKind::Html(HtmlTag::STYLE) => {
                        // TODO: Follow the generic raw text element parsing algorithm.
                        return true;
                    }
                    TagKind::Html(HtmlTag::NOSCRIPT) /* if the scripting flag is enabled */ => {
                        if /* the scripting flag is enabled */ true {
                            // TODO: Follow the generic raw text element parsing algorithm.
                        } else {
                            // TODO: Insert an HTML element for the token.
                            self.insertion_mode = InsertionMode::InHeadNoscript;
                        }
                        return true;
                    }
                    TagKind::Html(HtmlTag::SCRIPT) => {
                        // TODO
                        self.insertion_mode = InsertionMode::Text;
                        return true;
                    }
                    TagKind::Html(HtmlTag::TEMPLATE) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::HEAD) => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return true;
                    }
                    _ => {
                        // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                        self.insertion_mode = InsertionMode::AfterHead;
                        // Reprocess the token.
                    }
                }
                InsertionMode::InHeadNoscript => match name {
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                        return true;
                    }
                    TagKind::Html(HtmlTag::HEAD) |
                    TagKind::Html(HtmlTag::NOSCRIPT) => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return true;
                    }
                    _ => {
                        // TODO: Parse error.
                        // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                        self.insertion_mode = InsertionMode::InHead;
                        // Reprocess the token.
                    }
                }
                InsertionMode::AfterHead => match name {
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                        return true;
                    }
                    TagKind::Html(HtmlTag::BODY) => {
                        // TODO: Insert an HTML element for the token.
                        // TODO: Set the frameset-ok flag to "not ok".
                        self.insertion_mode = InsertionMode::InBody;
                    }
                    TagKind::Html(HtmlTag::FRAMESET) => {
                        // TODO: Insert an HTML element for the token.
                        self.insertion_mode = InsertionMode::InFrameset;
                    }
                    TagKind::Html(HtmlTag::BASE) |
                    TagKind::Html(HtmlTag::BASEFONT) |
                    TagKind::Html(HtmlTag::BGSOUND) |
                    TagKind::Html(HtmlTag::LINK) |
                    TagKind::Html(HtmlTag::META) |
                    TagKind::Html(HtmlTag::NOFRAMES) |
                    TagKind::Html(HtmlTag::SCRIPT) |
                    TagKind::Html(HtmlTag::STYLE) |
                    TagKind::Html(HtmlTag::TEMPLATE) |
                    TagKind::Html(HtmlTag::TITLE) => {
                        // TODO: Parse error.
                        // TODO: Push the node pointed to by the head element pointer onto the stack of open elements.
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        // TODO: Remove the node pointed to by the head element pointer from the stack of open elements. (It might not be the current node at this point.)
                        return true;
                    }
                    TagKind::Html(HtmlTag::HEAD) => {
                        // TODO: Parse error.
                        // Ignore the token.
                        return true;
                    }
                    _ => {
                        // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                        self.insertion_mode = InsertionMode::InBody;
                        // Reprocess the token.
                    }
                }
                InsertionMode::InBody => match name {
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Parse error.
                        // TODO: If there is a template element on the stack of open elements, then ignore the token.
                        // TODO: Otherwise, for each attribute on the token, check to see if the attribute is already present on the top element of the stack of open elements. If it is not, add the attribute and its corresponding value to that element.
                        return true;
                    }
                    TagKind::Html(HtmlTag::BASE) |
                    TagKind::Html(HtmlTag::BASEFONT) |
                    TagKind::Html(HtmlTag::BGSOUND) |
                    TagKind::Html(HtmlTag::LINK) |
                    TagKind::Html(HtmlTag::META) |
                    TagKind::Html(HtmlTag::NOFRAMES) |
                    TagKind::Html(HtmlTag::SCRIPT) |
                    TagKind::Html(HtmlTag::STYLE) |
                    TagKind::Html(HtmlTag::TEMPLATE) |
                    TagKind::Html(HtmlTag::TITLE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        return true;
                    }
                    TagKind::Html(HtmlTag::BODY) => {
                        // TODO: Parse error.
                        // TODO: If the second element on the stack of open elements is not a body element, if the stack of open elements has only one node on it, or if there is a template element on the stack of open elements, then ignore the token. (fragment case)
                        // TODO: Otherwise, set the frameset-ok flag to "not ok"; then, for each attribute on the token, check to see if the attribute is already present on the body element (the second element) on the stack of open elements, and if it is not, add the attribute and its corresponding value to that element.
                        return true;
                    }
                    TagKind::Html(HtmlTag::FRAMESET) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::ADDRESS) |
                    TagKind::Html(HtmlTag::ARTICLE) |
                    TagKind::Html(HtmlTag::ASIDE) |
                    TagKind::Html(HtmlTag::BLOCKQUOTE) |
                    TagKind::Html(HtmlTag::CENTER) |
                    TagKind::Html(HtmlTag::DETAILS) |
                    TagKind::Html(HtmlTag::DIALOG) |
                    TagKind::Html(HtmlTag::DIR) |
                    TagKind::Html(HtmlTag::DIV) |
                    TagKind::Html(HtmlTag::DL) |
                    TagKind::Html(HtmlTag::FIELDSET) |
                    TagKind::Html(HtmlTag::FIGCAPTION) |
                    TagKind::Html(HtmlTag::FIGURE) |
                    TagKind::Html(HtmlTag::FOOTER) |
                    TagKind::Html(HtmlTag::HEADER) |
                    TagKind::Html(HtmlTag::HGROUP) |
                    TagKind::Html(HtmlTag::MAIN) |
                    TagKind::Html(HtmlTag::MENU) |
                    TagKind::Html(HtmlTag::NAV) |
                    TagKind::Html(HtmlTag::OL) |
                    TagKind::Html(HtmlTag::P) |
                    TagKind::Html(HtmlTag::SECTION) |
                    TagKind::Html(HtmlTag::SUMMARY) |
                    TagKind::Html(HtmlTag::UL) => {
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: Insert an HTML element for the token.
                        return true;
                    }
                    TagKind::Html(HtmlTag::H1) |
                    TagKind::Html(HtmlTag::H2) |
                    TagKind::Html(HtmlTag::H3) |
                    TagKind::Html(HtmlTag::H4) |
                    TagKind::Html(HtmlTag::H5) |
                    TagKind::Html(HtmlTag::H6) => {
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: If the current node is an HTML element whose tag name is one of "h1", "h2", "h3", "h4", "h5", or "h6", then this is a parse error; pop the current node off the stack of open elements.
                        // TODO: Insert an HTML element for the token.
                        return true;
                    }
                    TagKind::Html(HtmlTag::PRE) |
                    TagKind::Other("listing") => {
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: Insert an HTML element for the token.
                        // TODO: If the next token is a U+000A LINE FEED (LF) character token, then ignore that token and move on to the next one. (Newlines at the start of pre blocks are ignored as an authoring convenience.)
                        // TODO: Set the frameset-ok flag to "not ok".
                        return true;
                    }
                    TagKind::Html(HtmlTag::FORM) => {
                        // TODO: If the form element pointer is not null, and there is no template element on the stack of open elements, then this is a parse error; ignore the token.
                        // TODO: Otherwise:
                        // TODO: If the stack of open elements has a p element in button scope, then close a p element.
                        // TODO: Insert an HTML element for the token, and, if there is no template element on the stack of open elements, set the form element pointer to point to the element created.
                        return true;
                    }
                    TagKind::Html(HtmlTag::LI) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::DD) |
                    TagKind::Html(HtmlTag::DT) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::PLAINTEXT) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::BUTTON) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::A) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::B) |
                    TagKind::Html(HtmlTag::BIG) |
                    TagKind::Html(HtmlTag::CODE) |
                    TagKind::Html(HtmlTag::EM) |
                    TagKind::Html(HtmlTag::FONT) |
                    TagKind::Html(HtmlTag::I) |
                    TagKind::Html(HtmlTag::S) |
                    TagKind::Html(HtmlTag::SMALL) |
                    TagKind::Html(HtmlTag::STRIKE) |
                    TagKind::Html(HtmlTag::STRONG) |
                    TagKind::Html(HtmlTag::TT) |
                    TagKind::Html(HtmlTag::U) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::NOBR) => {
                        // TODO
                        return true;
                    }
                    TagKind::Html(HtmlTag::APPLET) |
                    TagKind::Html(HtmlTag::MARQUEE) |
                    TagKind::Html(HtmlTag::OBJECT) => {
                        // TODO
                        return true;
                    }
                    _ => {
                        // TODO: Reconstruct the active formatting elements, if any.
                        // TODO: Insert an HTML element for the token.
                        return true;
                    }
                }
                InsertionMode::Text => {
                    unreachable!();
                }
                InsertionMode::InTable => match name {
                    TagKind::Html(HtmlTag::CAPTION) => {
                        // TODO: Clear the stack back to a table context.
                        // TODO: Insert a marker at the end of the list of active formatting elements.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InCaption);
                        return true;
                    }
                    TagKind::Html(HtmlTag::COLGROUP) => {
                        // TODO: Clear the stack back to a table context. (See below.)
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InColumnGroup);
                        return true;
                    }
                    TagKind::Html(HtmlTag::COL) => {
                        // TODO: Clear the stack back to a table context. (See below.)
                        // TODO: Insert an HTML element for a "colgroup" start tag token with no attributes
                        self.switch_to(InsertionMode::InColumnGroup);
                        // Reprocess the current token.
                    }
                    TagKind::Html(HtmlTag::TBODY) |
                    TagKind::Html(HtmlTag::TFOOT) |
                    TagKind::Html(HtmlTag::THEAD) => {
                        // TODO: Clear the stack back to a table context.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InTableBody);
                        return true;
                    }
                    TagKind::Html(HtmlTag::TD) |
                    TagKind::Html(HtmlTag::TH) |
                    TagKind::Html(HtmlTag::TR) => {
                        // TODO: Clear the stack back to a table context.
                        // TODO: Insert an HTML element for a "tbody" start tag token with no attributes
                        self.switch_to(InsertionMode::InTableBody);
                        // Reprocess the current token.
                    }
                    TagKind::Html(HtmlTag::TABLE) => {
                        // TODO: Parse error.
                        // TODO: If the stack of open elements does not have a table element in table scope, ignore the token.
                        // TODO: Otherwise:
                        // TODO: Pop elements from this stack until a table element has been popped from the stack.
                        // TODO: Reset the insertion mode appropriately.
                        // Reprocess the current token.
                    }
                    TagKind::Html(HtmlTag::STYLE) |
                    TagKind::Html(HtmlTag::SCRIPT) |
                    TagKind::Html(HtmlTag::TEMPLATE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                        return true;
                    }
                    TagKind::Html(HtmlTag::INPUT) => {
                        // TODO: If the token does not have an attribute with the name "type", or if it does, but that attribute's value is not an ASCII case-insensitive match for the string "hidden", then: act as described in the "anything else" entry below.
                        // TODO: Otherwise:
                        // TODO: Parse error.
                        // TODO: Insert an HTML element for the token.
                        // TODO: Pop that input element off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                    }
                    TagKind::Html(HtmlTag::FORM) => {
                        // TODO: Parse error.
                        // TODO: If there is a template element on the stack of open elements, or if the form element pointer is not null, ignore the token.
                        // TODO: Otherwise:
                        // TODO: Insert an HTML element for the token, and set the form element pointer to point to the element created.
                        // TODO: Pop that form element off the stack of open elements.
                    }
                    _ => {
                        // TODO: Parse error.
                        // TODO: Enable foster parenting, process the token using the rules for the "in body" insertion mode, and then disable foster parenting.
                    }
                }
                InsertionMode::InTableText => {
                    // TODO: If any of the tokens in the pending table character tokens list are character tokens that are not ASCII whitespace, then this is a parse error: reprocess the character tokens in the pending table character tokens list using the rules given in the "anything else" entry in the "in table" insertion mode.
                    // TODO: Otherwise, insert the characters given by the pending table character tokens list.
                    // TODO: Switch the insertion mode to the original insertion mode and reprocess the token.
                }
                InsertionMode::InCaption => match name {
                    TagKind::Html(HtmlTag::CAPTION) |
                    TagKind::Html(HtmlTag::COL) |
                    TagKind::Html(HtmlTag::COLGROUP) |
                    TagKind::Html(HtmlTag::TBODY) |
                    TagKind::Html(HtmlTag::TD) |
                    TagKind::Html(HtmlTag::TFOOT) |
                    TagKind::Html(HtmlTag::TH) |
                    TagKind::Html(HtmlTag::THEAD) |
                    TagKind::Html(HtmlTag::TR) => {
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
                }
                InsertionMode::InColumnGroup => match name {
                    TagKind::Html(HtmlTag::HTML) => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    TagKind::Html(HtmlTag::COL) => {
                        // TODO: Insert an HTML element for the token. Immediately pop the current node off the stack of open elements.
                        // TODO: Acknowledge the token's self-closing flag, if it is set.
                    }
                    TagKind::Html(HtmlTag::TEMPLATE) => {
                        // TODO: Process the token using the rules for the "in head" insertion mode.
                    }
                    _ => {
                        // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                        // TODO: Otherwise, pop the current node from the stack of open elements.
                        self.switch_to(InsertionMode::InTable);
                        // Reprocess the token.
                    }
                }
                InsertionMode::InTableBody => match name {
                    TagKind::Html(HtmlTag::TR) => {
                        // TODO: Clear the stack back to a table body context.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InRow);
                        return true;
                    }
                    TagKind::Html(HtmlTag::TH) |
                    TagKind::Html(HtmlTag::TD) => {
                        // TODO: Parse error.
                        // TODO: Clear the stack back to a table body context.
                        // TODO: Insert an HTML element for a "tr" start tag token with no attributes
                        self.switch_to(InsertionMode::InRow);
                        // Reprocess the current token.
                    }
                    TagKind::Html(HtmlTag::CAPTION) |
                    TagKind::Html(HtmlTag::COL) |
                    TagKind::Html(HtmlTag::COLGROUP) |
                    TagKind::Html(HtmlTag::TBODY) |
                    TagKind::Html(HtmlTag::TFOOT) |
                    TagKind::Html(HtmlTag::THEAD) => {
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
                }
                InsertionMode::InRow => match name {
                    TagKind::Html(HtmlTag::TH) |
                    TagKind::Html(HtmlTag::TD) => {
                        // TODO: Clear the stack back to a table row context.
                        // TODO: Insert an HTML element for the token
                        self.switch_to(InsertionMode::InCell);
                        // TODO: Insert a marker at the end of the list of active formatting elements.
                        return true;
                    }
                    TagKind::Html(HtmlTag::CAPTION) |
                    TagKind::Html(HtmlTag::COL) |
                    TagKind::Html(HtmlTag::COLGROUP) |
                    TagKind::Html(HtmlTag::TBODY) |
                    TagKind::Html(HtmlTag::TFOOT) |
                    TagKind::Html(HtmlTag::THEAD) |
                    TagKind::Html(HtmlTag::TR) => {
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
                }
                _ => {
                }
            }
        }
    }

    fn handle_character(&mut self, c: char) -> bool {
        loop {
            match self.insertion_mode {
                InsertionMode::Initial => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // ignore
                            return true;
                        }
                        _ => {
                            self.insertion_mode = InsertionMode::BeforeHtml;
                        }
                    }
                }
                InsertionMode::BeforeHtml => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // ignore
                            return true;
                        }
                        _ => {
                            // TODO: Create an html element whose node document is the Document object.
                            // TODO: Append it to the Document object.
                            // TODO: Put this element in the stack of open elements
                            self.insertion_mode = InsertionMode::BeforeHead;
                        }
                    }
                }
                InsertionMode::BeforeHead => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // ignore
                            return true;
                        }
                        _ => {
                            // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                            // TODO: Set the head element pointer to the newly created head element.
                            self.insertion_mode = InsertionMode::InHead;
                        }
                    }
                }
                InsertionMode::InHead => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the character.
                            return true;
                        }
                        _ => {
                            // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                            self.insertion_mode = InsertionMode::AfterHead;
                        }
                    }
                }
                InsertionMode::InHeadNoscript => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the character.
                            return true;
                        }
                        _ => {
                            // TODO: Parse error.
                            // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                            self.insertion_mode = InsertionMode::InHead;
                        }
                    }
                }
                InsertionMode::AfterHead => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the character.
                            return true;
                        }
                        _ => {
                            // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                            self.insertion_mode = InsertionMode::InBody;
                        }
                    }
                }
                InsertionMode::InBody => {
                    match c {
                        '\0' => {
                            // TODO: Parse error
                            // Ignore the token.
                            return true;
                        }
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Reconstruct the active formatting elements, if any.
                            // TODO: Insert the token's character.
                            return true;
                        }
                        _ => {
                            // TODO: Reconstruct the active formatting elements, if any.
                            // TODO: Insert the token's character.
                            // TODO: Set the frameset-ok flag to "not ok".
                            return true;
                        }
                    }
                }
                InsertionMode::Text => {
                    // TODO: Insert the token's character.
                }
                InsertionMode::InTable => {
                    // TODO: if the current node is table, tbody, template, tfoot, thead, or tr element {
                    //     TODO: Let the pending table character tokens be an empty list of tokens.
                    //     TODO: Let the original insertion mode be the current insertion mode.
                    //     TODO: Switch the insertion mode to "in table text" and reprocess the token.
                    // } else {
                    //     TODO: Parse error.
                    //     TODO: Enable foster parenting
                    //     TODO: process the token using the rules for the "in body" insertion mode
                    //     TODO: and then disable foster parenting.
                    // }
                }
                InsertionMode::InTableText => {
                    match c {
                        '\0' => {
                            // TODO: Parse error
                            // Ignore the token.
                            return true;
                        }
                        _ => {
                            // TODO: Append the character token to the pending table character tokens list.
                            return true;
                        }
                    }
                }
                InsertionMode::InCaption => {
                    // TODO: Process the token using the rules for the "in body" insertion mode.
                }
                InsertionMode::InColumnGroup => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the character.
                            return true;
                        }
                        _ => {
                            // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                            // TODO: Otherwise, pop the current node from the stack of open elements.
                            self.insertion_mode = InsertionMode::InTable;
                        }
                    }
                }
                InsertionMode::InTableBody => {
                    // TODO: Process the token using the rules for the "in table" insertion mode.
                }
                InsertionMode::InRow => {
                    // TODO: Process the token using the rules for the "in table" insertion mode.
                }
                InsertionMode::InCell => {
                    // TODO: Process the token using the rules for the "in body" insertion mode.
                }
                InsertionMode::InSelect => {
                    match c {
                        '\0' => {
                            // TODO: Parse error.
                            // Ignore the token.
                            return true;
                        }
                        _ => {
                            // TODO: Insert the token's character.
                            return true;
                        }
                    }
                }
                InsertionMode::InSelectInTable => {
                    // TODO: Process the token using the rules for the "in select" insertion mode.
                }
                InsertionMode::InTemplate => {
                    // TODO: Process the token using the rules for the "in body" insertion mode.
                }
                InsertionMode::AfterBody => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Process the token using the rules for the "in body" insertion mode.
                        }
                        _ => {
                            // TODO: Parse error.
                            self.insertion_mode = InsertionMode::InBody;
                        }
                    }
                }
                InsertionMode::InFrameset => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the character.
                            return true;
                        }
                        _ => {
                            // TODO: Parse error.
                            // Ignore the token.
                            return true;
                        }
                    }
                }
                InsertionMode::AfterFrameset => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the character.
                            return true;
                        }
                        _ => {
                            // TODO: Parse error.
                            // Ignore the token.
                            return true;
                        }
                    }
                }
                InsertionMode::AfterAfterBody => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Process the token using the rules for the "in body" insertion mode.
                        }
                        _ => {
                            // TODO: Parse error.
                            self.insertion_mode = InsertionMode::InBody;
                            return true;
                        }
                    }
                }
                InsertionMode::AfterAfterFrameset => {
                    match c {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Process the token using the rules for the "in body" insertion mode.
                        }
                        _ => {
                            // TODO: Parse error.
                            // Ignore the token.
                            return true;
                        }
                    }
                }
                InsertionMode::InForeignContent => {
                    match c {
                        '\0' => {
                            // TODO: Parse error.
                            // TODO: Insert a U+FFFD REPLACEMENT CHARACTER character.
                            return true;
                        }
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' => {
                            // TODO: Insert the token's character.
                            return true;
                        }
                        _ => {
                            // TODO: Insert the token's character.
                            // TODO: Set the frameset-ok flag to "not ok".
                            return true;
                        }
                    }
                }
            }
        }
    }

    fn handle_comment(&mut self, comment: &str) -> bool {
        match self.insertion_mode {
            InsertionMode::BeforeHead |
            InsertionMode::InHead |
            InsertionMode::InHeadNoscript |
            InsertionMode::AfterHead |
            InsertionMode::InBody |
            InsertionMode::InTable |
            InsertionMode::InCaption |
            InsertionMode::InColumnGroup |
            InsertionMode::InTableBody |
            InsertionMode::InRow |
            InsertionMode::InCell |
            InsertionMode::InSelect |
            InsertionMode::InSelectInTable |
            InsertionMode::InTemplate |
            InsertionMode::InFrameset |
            InsertionMode::AfterFrameset |
            InsertionMode::InForeignContent => {
                self.insert_comment(comment);
            }
            InsertionMode::Initial |
            InsertionMode::BeforeHtml |
            InsertionMode::AfterAfterBody |
            InsertionMode::AfterAfterFrameset => {
                self.append_comment(comment);
            }
            InsertionMode::AfterBody => {
                self.append_comment_to_root_element(comment);
            }
            InsertionMode::Text |
            InsertionMode::InTableText => {
                unreachable!();
            }
        }
        true
    }
}

enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
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
    InForeignContent,
}
