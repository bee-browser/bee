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
        tracing::debug!(?text);
        for c in text.data.chars() {
            loop {
                match self.insertion_mode {
                    InsertionMode::Initial => {
                        match c {
                            whitespace!() => {
                                // Ignore
                                break;
                            }
                            _ => {
                                self.switch_to(InsertionMode::BeforeHtml);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::BeforeHtml => {
                        match c {
                            whitespace!() => {
                                // Ignore
                                break;
                            }
                            _ => {
                                // TODO: Create an html element whose node document is the Document object.
                                // TODO: Append it to the Document object.
                                // TODO: Put this element in the stack of open elements
                                self.push_element(&Tag::with_html_tag(HtmlTag::HTML));
                                self.switch_to(InsertionMode::BeforeHead);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::BeforeHead => {
                        match c {
                            whitespace!() => {
                                // Ignore
                                break;
                            }
                            _ => {
                                // TODO: Insert an HTML element for a "head" start tag token with no attributes.
                                // TODO: Set the head element pointer to the newly created head element.
                                self.push_element(&Tag::with_html_tag(HtmlTag::HEAD));
                                self.switch_to(InsertionMode::InHead);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::InHead => {
                        match c {
                            whitespace!() => {
                                self.append_char(c);
                                break;
                            }
                            _ => {
                                // TODO: Pop the current node (which will be the head element) off the stack of open elements.
                                self.pop();
                                self.switch_to(InsertionMode::AfterHead);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::InHeadNoscript => {
                        match c {
                            whitespace!() => {
                                self.append_char(c);
                                break;
                            }
                            _ => {
                                // TODO: Parse error.
                                // TODO: Pop the current node (which will be a noscript element) from the stack of open elements; the new current node will be a head element.
                                self.switch_to(InsertionMode::InHead);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::AfterHead => {
                        match c {
                            whitespace!() => {
                                self.append_char(c);
                                break;
                            }
                            _ => {
                                // TODO: Insert an HTML element for a "body" start tag token with no attributes.
                                self.push_element(&Tag::with_html_tag(HtmlTag::BODY));
                                self.switch_to(InsertionMode::InBody);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::InBody => {
                        match c {
                            '\0' => {
                                // TODO: Parse error
                                // Ignore the token.
                            }
                            whitespace!() => {
                                // TODO: Reconstruct the active formatting elements, if any.
                                // TODO: Insert the token's character.
                                self.append_char(c);
                            }
                            _ => {
                                // TODO: Reconstruct the active formatting elements, if any.
                                // TODO: Insert the token's character.
                                // TODO: Set the frameset-ok flag to "not ok".
                                self.append_char(c);
                            }
                        }
                        break;
                    }
                    InsertionMode::Text => {
                        self.append_char(c);
                        break;
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
                            }
                            _ => {
                                // TODO: Append the character token to the pending table character tokens list.
                                return Control::Continue;
                            }
                        }
                        break;
                    }
                    InsertionMode::InCaption => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    InsertionMode::InColumnGroup => {
                        match c {
                            whitespace!() => {
                                // TODO: Insert the character.
                                self.append_char(c);
                                break;
                            }
                            _ => {
                                // TODO: If the current node is not a colgroup element, then this is a parse error; ignore the token.
                                // TODO: Otherwise, pop the current node from the stack of open elements.
                                self.switch_to(InsertionMode::InTable);
                                // reconsume
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
                            }
                            _ => {
                                // TODO: Insert the token's character.
                                self.append_char(c);
                            }
                        }
                        break;
                    }
                    InsertionMode::InSelectInTable => {
                        // TODO: Process the token using the rules for the "in select" insertion mode.
                    }
                    InsertionMode::InTemplate => {
                        // TODO: Process the token using the rules for the "in body" insertion mode.
                    }
                    InsertionMode::AfterBody => {
                        match c {
                            whitespace!() => {
                                // TODO: Process the token using the rules for the "in body" insertion mode.
                            }
                            _ => {
                                // TODO: Parse error.
                                self.switch_to(InsertionMode::InBody);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::InFrameset => {
                        match c {
                            whitespace!() => {
                                // TODO: Insert the character.
                                self.append_char(c);
                            }
                            _ => {
                                // TODO: Parse error.
                                // Ignore the token.
                            }
                        }
                        break;
                    }
                    InsertionMode::AfterFrameset => {
                        match c {
                            whitespace!() => {
                                // TODO: Insert the character.
                                self.append_char(c);
                            }
                            _ => {
                                // TODO: Parse error.
                                // Ignore the token.
                            }
                        }
                        break;
                    }
                    InsertionMode::AfterAfterBody => {
                        match c {
                            whitespace!() => {
                                // TODO: Process the token using the rules for the "in body" insertion mode.
                            }
                            _ => {
                                // TODO: Parse error.
                                self.switch_to(InsertionMode::InBody);
                                // reconsume
                            }
                        }
                    }
                    InsertionMode::AfterAfterFrameset => {
                        match c {
                            whitespace!() => {
                                // TODO: Process the token using the rules for the "in body" insertion mode.
                            }
                            _ => {
                                // TODO: Parse error.
                                // Ignore the token.
                            }
                        }
                        break;
                    }
                    InsertionMode::InForeignContent => {
                        match c {
                            '\0' => {
                                // TODO: Parse error.
                                // TODO: Insert a U+FFFD REPLACEMENT CHARACTER character.
                                self.append_char('\u{FFFD}');
                            }
                            whitespace!() => {
                                // TODO: Insert the token's character.
                                self.append_char(c);
                            }
                            _ => {
                                // TODO: Insert the token's character.
                                // TODO: Set the frameset-ok flag to "not ok".
                                self.append_char(c);
                            }
                        }
                        break;
                    }
                }
            }
        }
        Control::Continue
    }
}
