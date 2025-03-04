use super::*;

impl<T> TreeBuilder<T>
where
    T: DomTreeBuilder,
{
    pub fn is_in_foreign_content(&self, token: &Token<'_>) -> bool {
        if let Token::End = token {
            return false;
        }
        let context = self.adjusted_context();
        if context.is_html() {
            return false;
        }
        match token {
            Token::StartTag(tag) => {
                if context.is_html_integration_point() {
                    return false;
                }
                let local_name = LocalName::lookup(tag.name);
                if context.is_svg_integration_point() {
                    if let tag!(svg: Svg) = local_name {
                        return false;
                    }
                }
                if context.is_mathml_text_integration_point() {
                    match local_name {
                        tag!(mathml: Mglyph, Malignmark) => (),
                        _ => return false,
                    }
                }
            }
            Token::Null(_) | Token::Whitespace(_) | Token::Text(_) => {
                if context.is_html_integration_point() {
                    return false;
                }
                if context.is_mathml_text_integration_point() {
                    return false;
                }
            }
            _ => (),
        }
        true
    }

    pub fn handle_foreign(&mut self, token: Token<'_>) -> Control {
        logger::debug!(?token);
        self.ignore_lf = false;
        match token {
            Token::Doctype(_) => {
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
                    loop {
                        let context = self.context();
                        if context.is_mathml_text_integration_point() {
                            break;
                        }
                        if context.is_html_integration_point() {
                            break;
                        }
                        if context.is_html() {
                            break;
                        }
                        self.pop_element();
                    }
                    self.handle_start_tag(tag)
                }
                tag!(Font) if tag.has_any_attributes(&["color", "face", "size"]) => {
                    // TODO: Parse error.
                    loop {
                        let context = self.context();
                        if context.is_mathml_text_integration_point() {
                            break;
                        }
                        if context.is_html_integration_point() {
                            break;
                        }
                        if context.is_html() {
                            break;
                        }
                        self.pop_element();
                    }
                    self.handle_start_tag(tag)
                }
                local_name => {
                    match self.adjusted_context().open_element.namespace {
                        Namespace::MathMl => self.push_mathml_element(&tag, local_name),
                        Namespace::Svg => self.push_svg_element(&tag, local_name),
                        _ => unreachable!(),
                    }
                    if tag.self_closing {
                        match local_name {
                            tag!(Script) => {
                                self.pop_element();
                                // TODO
                            }
                            _ => {
                                self.pop_element();
                                // TODO: acknowledge the token's self-closing flag
                            }
                        }
                    }
                    Control::Continue
                }
            },
            Token::EndTag(tag) => match LocalName::lookup(tag.name) {
                tag!(Br, P) => {
                    // TODO: Parse error.
                    // TODO: While the current node is not a MathML text integration point, an HTML integration point, or an element in the HTML namespace, pop elements from the stack of open elements.
                    loop {
                        if self.context().is_html() {
                            break;
                        }
                        if self.context().is_mathml_text_integration_point() {
                            break;
                        }
                        if self.context().is_html_integration_point() {
                            break;
                        }
                        self.pop_element();
                    }
                    self.handle_end_tag(tag)
                }
                tag!(Script) if self.context().is_svg_script() => {
                    self.pop_element();
                    // TODO
                    Control::Continue
                }
                _ => {
                    self.append_text_if_exists();
                    let mut stack_pos = self.context_stack.len() - 1;
                    if !self.context_stack[stack_pos]
                        .open_element
                        .has_same_name(tag.name)
                    {
                        // TODO: Parser error.
                    }
                    while stack_pos > 0 {
                        if self.context_stack[stack_pos]
                            .open_element
                            .has_same_name(tag.name)
                        {
                            self.context_stack.truncate(stack_pos);
                            break;
                        }
                        stack_pos -= 1;
                        if self.context_stack[stack_pos].is_html() {
                            return self.handle_end_tag(tag);
                        }
                    }
                    Control::Continue
                }
            },
            Token::Null(text) => {
                // TODO: Parse error.
                self.append_replacement_characters(text.data.len());
                Control::Continue
            }
            Token::Whitespace(text) => {
                self.append_text(text.data);
                Control::Continue
            }
            Token::Text(text) => {
                self.append_text(text.data);
                self.frameset_ok = false;
                Control::Continue
            }
            Token::Comment(comment) => {
                self.insert_comment(&comment);
                Control::Continue
            }
            Token::Error(error) => self.handle_error(error),
            Token::End => unreachable!(),
        }
    }
}
