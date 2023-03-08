use super::*;

impl<W> TreeBuilder<W>
where
    W: DocumentWriter,
{
    pub fn is_in_foreign_content(&self, token: &Token<'_>) -> bool {
        if let Token::End = token {
            return false;
        }
        if let Namespace::Html = self.context.namespace {
            return false;
        }
        match token {
            Token::StartTag(ref tag) => {
                if self.context.html_integration_pont {
                    return false;
                }
                let local_name = LocalName::lookup(tag.name);
                if self.context.svg_integration_point {
                    if let tag!(svg: Svg) = local_name {
                        return false;
                    }
                }
                if self.context.mathml_text_integration_point {
                    match local_name {
                        tag!(mathml: Mglyph, Malignmark) => (),
                        _ => return false,
                    }
                }
            }
            Token::Text(_) => {
                if self.context.html_integration_pont {
                    return false;
                }
                if self.context.mathml_text_integration_point {
                    return false;
                }
            }
            _ => (),
        }
        true
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn handle_foreign(&mut self, token: Token<'_>) -> Control {
        tracing::debug!(?token);
        self.ignore_lf = false;
        match token {
            Token::Doctype(doctype) => {
                // TODO: Parse error.
                // Ignore the token.
                Control::Continue
            }
            Token::StartTag(tag) => match LocalName::lookup(tag.name) {
                tag!(
                    B, Big, Blockquote, Body, Br, Center, Code, Dd, Div, Dl, Dt, Em, Embed, H1, H2,
                    H3, H4, H5, H6, Head, Hr, I, Img, Li, Listing, Menu, Meta, Nobr, Ol, P, Pre,
                    Ruby, S, Small, Span, Strong, Strike, Sub, Sup, Table, Tt, U, Ul, Var
                ) => {
                    // TODO: Parse error.
                    // TODO: While the current node is not a MathML text integration point, an HTML integration point, or an element in the HTML namespace, pop elements from the stack of open elements.
                    loop {
                        if let Namespace::Html = self.context.namespace {
                            break;
                        }
                        if self.context.mathml_text_integration_point {
                            break;
                        }
                        if self.context.html_integration_pont {
                            break;
                        }
                        self.pop_element();
                    }
                    // TODO: Reprocess the token according to the rules given in the section corresponding to the current insertion mode in HTML content.
                    self.handle_start_tag(tag)
                }
                tag!(Font) if tag.has_any_attributes(&["color", "face", "size"]) => {
                    // TODO: Parse error.
                    // TODO: While the current node is not a MathML text integration point, an HTML integration point, or an element in the HTML namespace, pop elements from the stack of open elements.
                    loop {
                        if let Namespace::Html = self.context.namespace {
                            break;
                        }
                        if self.context.mathml_text_integration_point {
                            break;
                        }
                        if self.context.html_integration_pont {
                            break;
                        }
                        self.pop_element();
                    }
                    // TODO: Reprocess the token according to the rules given in the section corresponding to the current insertion mode in HTML content.
                    self.handle_start_tag(tag)
                }
                local_name => {
                    // TODO
                    match self.context.namespace {
                        Namespace::MathMl => self.push_mathml_element(&tag),
                        Namespace::Svg => self.push_svg_element(&tag, local_name),
                        _ => unreachable!(),
                    }
                    Control::Continue
                }
            },
            Token::EndTag(tag) => match LocalName::lookup(tag.name) {
                tag!(Br, P) => {
                    // TODO: Parse error.
                    // TODO: While the current node is not a MathML text integration point, an HTML integration point, or an element in the HTML namespace, pop elements from the stack of open elements.
                    loop {
                        if let Namespace::Html = self.context.namespace {
                            break;
                        }
                        if self.context.mathml_text_integration_point {
                            break;
                        }
                        if self.context.html_integration_pont {
                            break;
                        }
                        self.pop_element();
                    }
                    // TODO: Reprocess the token according to the rules given in the section corresponding to the current insertion mode in HTML content.
                    self.handle_start_tag(tag)
                }
                tag!(Script) if self.context.svg_script => {
                    self.pop_element();
                    // TODO
                    Control::Continue
                }
                _ => {
                    self.pop_element();
                    // TODO
                    Control::Continue
                }
            },
            Token::Text(text) => {
                for c in text.data.chars() {
                    match c {
                        '\0' => {
                            // TODO: Parse error.
                            self.append_char('\u{FFFD}');
                        }
                        char_class!(whitespace) => {
                            self.append_char(c);
                        }
                        _ => {
                            self.append_char(c);
                            self.frameset_ok = false;
                        }
                    }
                }
                Control::Continue
            }
            Token::Comment(comment) => {
                self.append_comment(&comment);
                Control::Continue
            }
            Token::Error(error) => self.handle_error(error),
            Token::End => unreachable!(),
        }
    }
}
