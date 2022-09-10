use crate::error::Error;
use crate::error::ErrorCode;
use std::collections::VecDeque;
use std::ops::Range;
use crate::Location;
use crate::charref::CharRefResolver;
use crate::inputstream::CodePoint;
use crate::inputstream::InputStream;

/// An HTML5-compliant tokenizer.
///
/// The `Tokenizer` type implements the tokenization state machine described in
/// "13.2.5 Tokenization" in the WHATWG HTML specification.
pub struct Tokenizer {
    state: State,
    return_state: State,
    input_stream: InputStream,
    next_char: Option<Char>,
    next_code_point: Option<(CodePoint, Location)>,
    char_buffer: String,
    temp_buffer: String,
    tag: Tag,
    last_start_tag: Option<String>,
    doctype: Doctype,
    char_ref_code: u32,
    char_ref_resolver: CharRefResolver,
    has_text: bool,
    has_comment: bool,
    clear_char_buffer: bool,
    has_duplicate_attr: bool,
    tokens: VecDeque<Result<Token, Error>>,
}

impl Tokenizer {
    const INITIAL_BUFFER_SIZE: usize = 4096;

    pub fn new() -> Self {
        Tokenizer {
            state: State::Data,
            return_state: State::Data,
            input_stream: InputStream::new(),
            next_char: None,
            next_code_point: None,
            char_buffer: String::with_capacity(Self::INITIAL_BUFFER_SIZE),
            temp_buffer: String::with_capacity(Self::INITIAL_BUFFER_SIZE),
            tag: Default::default(),
            last_start_tag: None,
            doctype: Default::default(),
            char_ref_code: 0,
            char_ref_resolver: Default::default(),
            has_text: false,
            has_comment: false,
            clear_char_buffer: false,
            has_duplicate_attr: false,
            tokens: VecDeque::with_capacity(3),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        loop {
            if let Some(token) = self.tokens.pop_front() {
                return token;
            }

            if let State::End = self.state {
                return Ok(Token::End);
            }

            if self.clear_char_buffer {
                self.char_buffer.clear();
                self.clear_char_buffer = false;
            }

            self.tokenize();
        }
    }

    pub fn feed_data(&mut self, data: Vec<u16>) {
        self.input_stream.feed_data(data);
    }

    pub fn feed_end(&mut self) {
        self.input_stream.feed_end();
    }

    #[cfg(test)]
    pub(crate) fn set_initial_state(&mut self, state: State) {
        self.state = state;
    }

    #[cfg(test)]
    pub fn set_last_start_tag(&mut self, tag_name: String) {
        self.last_start_tag = Some(tag_name);
    }

    pub fn doctype_name(&self) -> Option<&str> {
        self.doctype.name.as_ref().map(|range| {
            self.char_buffer.get(range.clone())
                .expect("")
        })
    }

    pub fn doctype_public_id(&self) -> Option<&str> {
        self.doctype.public_id.as_ref().map(|range| {
            self.char_buffer.get(range.clone())
                .expect("")
        })
    }

    pub fn doctype_system_id(&self) -> Option<&str> {
        self.doctype.system_id.as_ref().map(|range| {
            self.char_buffer.get(range.clone())
                .expect("")
        })
    }

    pub fn force_quirks(&self) -> bool {
        self.doctype.force_quirks
    }

    pub fn tag_name(&self) -> &str {
        self.char_buffer.get(self.tag.name.clone())
            .expect("")
    }

    pub fn attrs(&self) -> Attrs {
        Attrs::new(self)
    }

    pub fn is_empty_tag(&self) -> bool {
        self.tag.self_closing
    }

    pub fn text(&self) -> &str {
        self.char_buffer.as_str()
    }

    pub fn comment(&self) -> &str {
        self.char_buffer.as_str()
    }

    fn tokenize(&mut self) {
        match self.state {
            State::Data =>
                self.tokenize_data(),
            State::Rcdata =>
                self.tokenize_rcdata(),
            State::Rawtext =>
                self.tokenize_rawtext(),
            State::ScriptData =>
                self.tokenize_script_data(),
            State::Plaintext =>
                self.tokenize_plaintext(),
            State::TagOpen =>
                self.tokenize_tag_open(),
            State::EndTagOpen =>
                self.tokenize_end_tag_open(),
            State::TagName =>
                self.tokenize_tag_name(),
            State::RcdataLessThanSign =>
                self.tokenize_rcdata_less_than_sign(),
            State::RcdataEndTagOpen =>
                self.tokenizer_rcdata_end_tag_open(),
            State::RcdataEndTagName =>
                self.tokenize_rcdata_end_tag_name(),
            State::RawtextLessThanSign =>
                self.tokenize_rawtext_less_than_sign(),
            State::RawtextEndTagOpen =>
                self.tokenizer_rawtext_end_tag_open(),
            State::RawtextEndTagName =>
                self.tokenize_rawtext_end_tag_name(),
            State::ScriptDataLessThanSign =>
                self.tokenize_script_data_less_than_sign(),
            State::ScriptDataEndTagOpen =>
                self.tokenize_script_data_end_tag_open(),
            State::ScriptDataEndTagName =>
                self.tokenize_script_data_end_tag_name(),
            State::ScriptDataEscapeStart =>
                self.tokenize_script_data_escape_start(),
            State::ScriptDataEscapeStartDash =>
                self.tokenize_script_data_escape_start_dash(),
            State::ScriptDataEscaped =>
                self.tokenize_script_data_escaped(),
            State::ScriptDataEscapedDash =>
                self.tokenize_script_data_escaped_dash(),
            State::ScriptDataEscapedDashDash =>
                self.tokenize_script_data_escaped_dash_dash(),
            State::ScriptDataEscapedLessThanSign =>
                self.tokenize_script_data_escaped_less_than_sign(),
            State::ScriptDataEscapedEndTagOpen =>
                self.tokenize_script_data_escaped_end_tag_open(),
            State::ScriptDataEscapedEndTagName =>
                self.tokenize_script_data_escaped_end_tag_name(),
            State::ScriptDataDoubleEscapeStart =>
                self.tokenize_script_data_double_escape_start(),
            State::ScriptDataDoubleEscaped =>
                self.tokenize_script_data_double_escaped(),
            State::ScriptDataDoubleEscapedDash =>
                self.tokenize_script_data_double_escaped_dash(),
            State::ScriptDataDoubleEscapedDashDash =>
                self.tokenize_script_data_double_escaped_dash_dash(),
            State::ScriptDataDoubleEscapedLessThanSign =>
                self.tokenize_script_data_double_escaped_less_than_sign(),
            State::ScriptDataDoubleEscapeEnd =>
                self.tokenize_script_data_double_escape_end(),
            State::BeforeAttributeName =>
                self.tokenize_before_attribute_name(),
            State::AttributeName =>
                self.tokenize_attribute_name(),
            State::AfterAttributeName =>
                self.tokenize_after_attribute_name(),
            State::BeforeAttributeValue =>
                self.tokenize_before_attribute_value(),
            State::AttributeValueDoubleQuoted =>
                self.tokenize_attribute_value_double_quoted(),
            State::AttributeValueSingleQuoted =>
                self.tokenize_attribute_value_single_quoted(),
            State::AttributeValueUnquoted =>
                self.tokenize_attribute_value_unquoted(),
            State::AfterAttributeValueQuoted =>
                self.tokenize_after_attribute_value_quoted(),
            State::SelfClosingTag =>
                self.tokenize_self_closing_tag(),
            State::BogusComment =>
                self.tokenize_bogus_comment(),
            State::MarkupDeclarationOpen =>
                self.tokenize_markup_declaration(),
            State::MaybeCommentStart =>
                self.tokenize_maybe_comment_start(),
            State::CommentStart =>
                self.tokenize_comment_start(),
            State::CommentStartDash =>
                self.tokenize_comment_start_dash(),
            State::Comment =>
                self.tokenize_comment(),
            State::CommentLessThanSign =>
                self.tokenize_comment_less_than_sign(),
            State::CommentLessThanSignBang =>
                self.tokenize_comment_less_than_sign_bang(),
            State::CommentLessThanSignBangDash =>
                self.tokenize_comment_less_than_sign_bang_dash(),
            State::CommentLessThanSignBangDashDash =>
                self.tokenize_comment_less_than_sign_bang_dash_dash(),
            State::CommentEndDash =>
                self.tokenize_comment_end_dash(),
            State::CommentEnd =>
                self.tokenize_comment_end(),
            State::CommentEndBang =>
                self.tokenize_comment_end_bang(),
            State::MaybeDoctype1 =>
                self.tokenize_maybe_doctype1(),
            State::MaybeDoctype2 =>
                self.tokenize_maybe_doctype2(),
            State::MaybeDoctype3 =>
                self.tokenize_maybe_doctype3(),
            State::MaybeDoctype4 =>
                self.tokenize_maybe_doctype4(),
            State::MaybeDoctype5 =>
                self.tokenize_maybe_doctype5(),
            State::MaybeDoctype6 =>
                self.tokenize_maybe_doctype6(),
            State::Doctype =>
                self.tokenize_doctype(),
            State::BeforeDoctypeName =>
                self.tokenize_before_doctype_name(),
            State::DoctypeName =>
                self.tokenize_doctype_name(),
            State::AfterDoctypeName =>
                self.tokenize_after_doctype_name(),
            State::MaybeDoctypePublicKeyword1 =>
                self.tokenize_maybe_doctype_public_keyword1(),
            State::MaybeDoctypePublicKeyword2 =>
                self.tokenize_maybe_doctype_public_keyword2(),
            State::MaybeDoctypePublicKeyword3 =>
                self.tokenize_maybe_doctype_public_keyword3(),
            State::MaybeDoctypePublicKeyword4 =>
                self.tokenize_maybe_doctype_public_keyword4(),
            State::MaybeDoctypePublicKeyword5 =>
                self.tokenize_maybe_doctype_public_keyword5(),
            State::AfterDoctypePublicKeyword =>
                self.tokenize_after_doctype_public_keyword(),
            State::BeforeDoctypePublicIdentifier =>
                self.tokenize_before_doctype_public_identifier(),
            State::DoctypePublicIdentifierDoubleQuoted =>
                self.tokenize_doctype_public_identifier_double_quoted(),
            State::DoctypePublicIdentifierSingleQuoted =>
                self.tokenize_doctype_public_identifier_single_quoted(),
            State::AfterDoctypePublicIdentifier =>
                self.tokenize_after_doctype_public_identifier(),
            State::BetweenDoctypePublicAndSystemIdentifiers =>
                self.tokenize_between_doctype_public_and_system_identifiers(),
            State::MaybeDoctypeSystemKeyword1 =>
                self.tokenize_maybe_doctype_system_keyword1(),
            State::MaybeDoctypeSystemKeyword2 =>
                self.tokenize_maybe_doctype_system_keyword2(),
            State::MaybeDoctypeSystemKeyword3 =>
                self.tokenize_maybe_doctype_system_keyword3(),
            State::MaybeDoctypeSystemKeyword4 =>
                self.tokenize_maybe_doctype_system_keyword4(),
            State::MaybeDoctypeSystemKeyword5 =>
                self.tokenize_maybe_doctype_system_keyword5(),
            State::AfterDoctypeSystemKeyword =>
                self.tokenize_after_doctype_system_keyword(),
            State::BeforeDoctypeSystemIdentifier =>
                self.tokenize_before_doctype_system_identifier(),
            State::DoctypeSystemIdentifierDoubleQuoted =>
                self.tokenize_doctype_system_identifier_double_quoted(),
            State::DoctypeSystemIdentifierSingleQuoted =>
                self.tokenize_doctype_system_identifier_single_quoted(),
            State::AfterDoctypeSystemIdentifier =>
                self.tokenize_after_doctype_system_identifier(),
            State::BogusDoctype =>
                self.tokenize_bogus_doctype(),
            State::MaybeCdataSection1 =>
                self.tokenize_maybe_cdata_section1(),
            State::MaybeCdataSection2 =>
                self.tokenize_maybe_cdata_section2(),
            State::MaybeCdataSection3 =>
                self.tokenize_maybe_cdata_section3(),
            State::MaybeCdataSection4 =>
                self.tokenize_maybe_cdata_section4(),
            State::MaybeCdataSection5 =>
                self.tokenize_maybe_cdata_section5(),
            State::MaybeCdataSection6 =>
                self.tokenize_maybe_cdata_section6(),
            State::CdataSection =>
                self.tokenize_cdata_section(),
            State::CdataSectionBracket =>
                self.tokenize_cdata_section_bracket(),
            State::CdataSectionEnd =>
                self.tokenize_cdata_section_end(),
            State::CharacterReference =>
                self.tokenize_character_reference(),
            State::NamedCharacterReference =>
                self.tokenize_named_character_reference(),
            State::AmbigousAmpersand =>
                self.tokenize_ambigous_ampersand(),
            State::NumericCharacterReference =>
                self.tokenize_numeric_character_reference(),
            State::HexadecimalCharacterReferenceStart =>
                self.tokenize_hexadecimal_character_reference_start(),
            State::DecimalCharacterReferenceStart =>
                self.tokenize_decimal_character_reference_start(),
            State::HexadecimalCharacterReference =>
                self.tokenize_hexadecimal_character_reference(),
            State::DecimalCharacterReference =>
                self.tokenize_decimal_character_reference(),
            State::NumericCharacterReferenceEnd =>
                self.tokenize_numeric_character_reference_end(),
            _ => unreachable!("{:?}", self.state),
        }
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#data-state
    fn tokenize_data(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('&'), _) => {
                    self.return_state = State::Data;
                    self.switch_to(State::CharacterReference);
                    return;
                }
                Char(Some('<'), _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::TagOpen);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text('\0');
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
                Char(None, _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
            }
        }
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#rcdata-state
    fn tokenize_rcdata(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('&'), _) => {
                    self.return_state = State::Rcdata;
                    self.switch_to(State::CharacterReference);
                    return;
                }
                Char(Some('<'), _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::RcdataLessThanSign);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                }
                Char(None, _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_rawtext(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('<'), _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::RawtextLessThanSign);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                }
                Char(None, _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_script_data(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('<'), _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::ScriptDataLessThanSign);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                }
                Char(None, _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_plaintext(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                }
                Char(None, _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_tag_open(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('!'), _) => {
                self.switch_to(State::MarkupDeclarationOpen);
            }
            Char(Some('/'), _) => {
                self.switch_to(State::EndTagOpen);
            }
            Char(Some('?'), location) => {
                self.emit_error(
                    ErrorCode::UnexpectedQuestionMarkInsteadOfTagName,
                    location);
                // TODO: Create a comment token whose data is the empty string
                self.reconsume_in(ch, State::BogusComment);
            }
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.create_start_tag();
                self.reconsume_in(ch, State::TagName);
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofBeforeTagName, location);
                self.append_char_to_text('<');
                self.emit_text();
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidFirstCharacterOfTagName, location);
                self.append_char_to_text('<');
                self.reconsume_in(ch, State::Data);
            }
        }
    }

    fn tokenize_end_tag_open(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.create_end_tag();
                self.reconsume_in(ch, State::TagName)
            }
            Char(Some('>'), location) => {
                self.emit_error(ErrorCode::MissingEndTagName, location);
                self.switch_to(State::Data)
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofBeforeTagName, location);
                self.append_str_to_text("</");
                self.emit_text();
                self.switch_to(State::End)
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidFirstCharacterOfTagName, location);
                // TODO: Create a comment token whose data is the empty string
                self.reconsume_in(ch, State::BogusComment)
            }
        }
    }

    fn tokenize_tag_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    self.switch_to(State::BeforeAttributeName);
                    return;
                }
                Char(Some('/'), _) => {
                    self.switch_to(State::SelfClosingTag);
                    return;
                }
                Char(Some('>'), location) => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_tag_name(c.to_ascii_lowercase());
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_tag_name(char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInTag, location);
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_tag_name(c);
                }
            }
        }
    }

    fn tokenize_rcdata_less_than_sign(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('/'), _) => {
                self.reset_temp();
                self.switch_to(State::RcdataEndTagOpen);
            }
            _ => {
                self.append_char_to_text('<');
                self.reconsume_in(ch, State::Rcdata);
            }
        }
    }

    fn tokenizer_rcdata_end_tag_open(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.create_end_tag();
                self.reconsume_in(ch, State::RcdataEndTagName);
            }
            _ => {
                self.append_str_to_text("</");
                self.reconsume_in(ch, State::Rcdata);
            }
        }
    }

    fn tokenize_rcdata_end_tag_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::BeforeAttributeName);
                    return;
                }
                Char(Some('/'), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::SelfClosingTag);
                    return;
                }
                Char(Some('>'), location) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_tag_name(c.to_ascii_lowercase());
                    self.append_char_to_temp(c);
                }
                Char(Some(c), _) if c.is_ascii_lowercase() => {
                    self.append_char_to_tag_name(c);
                    self.append_char_to_temp(c);
                }
                _ => {
                    self.discard_tag();
                    self.append_str_to_text("</");
                    self.append_temp_to_text();
                    self.reconsume_in(ch, State::Rcdata);
                    return;
                }
            }
        }
    }

    fn tokenize_rawtext_less_than_sign(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('/'), _) => {
                self.reset_temp();
                self.switch_to(State::RawtextEndTagOpen);
            }
            _ => {
                self.append_char_to_text('<');
                self.reconsume_in(ch, State::Rawtext);
            }
        }
    }

    fn tokenizer_rawtext_end_tag_open(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.create_end_tag();
                self.reconsume_in(ch, State::RawtextEndTagName);
            }
            _ => {
                self.append_str_to_text("</");
                self.reconsume_in(ch, State::Rawtext);
            }
        }
    }

    fn tokenize_rawtext_end_tag_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::BeforeAttributeName);
                    return;
                }
                Char(Some('/'), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::SelfClosingTag);
                    return;
                }
                Char(Some('>'), location) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_tag_name(c.to_ascii_lowercase());
                    self.append_char_to_temp(c);
                }
                Char(Some(c), _) if c.is_ascii_lowercase() => {
                    self.append_char_to_tag_name(c);
                    self.append_char_to_temp(c);
                }
                _ => {
                    self.discard_tag();
                    self.append_str_to_text("</");
                    self.append_temp_to_text();
                    self.reconsume_in(ch, State::Rawtext);
                    return;
                }
            }
        }
    }

    fn tokenize_script_data_less_than_sign(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('/'), _) => {
                self.reset_temp();
                self.switch_to(State::ScriptDataEndTagOpen);
            }
            Char(Some('!'), _) => {
                self.switch_to(State::ScriptDataEscapeStart);
                self.append_str_to_text("<!");
            }
            _ => {
                self.append_char_to_text('<');
                self.reconsume_in(ch, State::ScriptData);
            }
        }
    }

    fn tokenize_script_data_end_tag_open(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.create_end_tag();
                self.reconsume_in(ch, State::ScriptDataEndTagName);
            }
            _ => {
                self.append_str_to_text("</");
                self.reconsume_in(ch, State::ScriptData);
            }
        }
    }

    fn tokenize_script_data_end_tag_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::BeforeAttributeName);
                    return;
                }
                Char(Some('/'), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::SelfClosingTag);
                    return;
                }
                Char(Some('>'), location) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_tag_name(c.to_ascii_lowercase());
                    self.append_char_to_temp(c);
                }
                Char(Some(c), _) if c.is_ascii_lowercase() => {
                    self.append_char_to_tag_name(c);
                    self.append_char_to_temp(c);
                }
                _ => {
                    self.discard_tag();
                    self.append_str_to_text("</");
                    self.append_temp_to_text();
                    return self.reconsume_in(ch, State::ScriptData);
                }
            }
        }
    }

    fn tokenize_script_data_escape_start(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::ScriptDataEscapeStartDash);
                self.append_char_to_text('-');
            }
            _ => {
                self.reconsume_in(ch, State::ScriptData);
            }
        }
    }

    fn tokenize_script_data_escape_start_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::ScriptDataEscapedDashDash);
                self.append_char_to_text('-');
            }
            _ => {
                self.reconsume_in(ch, State::ScriptData);
            }
        }
    }

    fn tokenize_script_data_escaped(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('-'), _) => {
                    self.switch_to(State::ScriptDataEscapedDash);
                    self.append_char_to_text('-');
                    return;
                }
                Char(Some('<'), _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::ScriptDataEscapedLessThanSign);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(
                        ErrorCode::EofInScriptHtmlCommentLikeText, location);
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_script_data_escaped_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::ScriptDataEscapedDashDash);
                self.append_char_to_text('-');
            }
            Char(Some('<'), _) => {
                self.emit_token_if_exists();
                self.switch_to(State::ScriptDataEscapedLessThanSign);
            }
            Char(Some('\0'), location) => {
                self.emit_error(ErrorCode::UnexpectedNullCharacter, location);
                self.switch_to(State::ScriptDataEscaped);
                self.append_char_to_text(char::REPLACEMENT_CHARACTER);
            }
            Char(None, location) => {
                self.emit_error(
                    ErrorCode::EofInScriptHtmlCommentLikeText, location);
                self.emit_token_if_exists();
                self.switch_to(State::End);
            }
            Char(Some(c), _) => {
                self.switch_to(State::ScriptDataEscaped);
                self.append_char_to_text(c);
            }
        }
    }

    fn tokenize_script_data_escaped_dash_dash(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('-'), _) => {
                    self.append_char_to_text('-');
                }
                Char(Some('<'), _) => {
                    self.emit_token_if_exists();
                    self.switch_to(State::ScriptDataEscapedLessThanSign);
                    return;
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::ScriptData);
                    self.append_char_to_text('>');
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.switch_to(State::ScriptDataEscaped);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                    return;
                }
                Char(None, location) => {
                    self.emit_error(
                        ErrorCode::EofInScriptHtmlCommentLikeText, location);
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.switch_to(State::ScriptDataEscaped);
                    self.append_char_to_text(c);
                    return;
                }
            }
        }
    }

    fn tokenize_script_data_escaped_less_than_sign(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('/'), _) => {
                self.reset_temp();
                self.switch_to(State::ScriptDataEscapedEndTagOpen);
            }
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.reset_temp();
                self.append_char_to_text('<');
                self.reconsume_in(ch, State::ScriptDataDoubleEscapeStart);
            }
            _ => {
                self.append_char_to_text('<');
                self.reconsume_in(ch, State::ScriptDataEscaped);
            }
        }
    }

    fn tokenize_script_data_escaped_end_tag_open(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_alphabetic() => {
                self.create_end_tag();
                self.reconsume_in(ch, State::ScriptDataEscapedEndTagName);
            }
            _ => {
                self.append_str_to_text("</");
                self.reconsume_in(ch, State::ScriptDataEscaped);
            }
        }
    }

    fn tokenize_script_data_escaped_end_tag_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::BeforeAttributeName);
                    return;
                }
                Char(Some('/'), _) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::SelfClosingTag);
                    return;
                }
                Char(Some('>'), location) if self.is_appropriate_end_tag() => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_tag_name(c.to_ascii_lowercase());
                    self.append_char_to_temp(c);
                }
                Char(Some(c), _) if c.is_ascii_lowercase() => {
                    self.append_char_to_tag_name(c);
                    self.append_char_to_temp(c);
                }
                _ => {
                    self.discard_tag();
                    self.append_str_to_text("</");
                    self.append_temp_to_text();
                    self.reconsume_in(ch, State::ScriptDataEscaped);
                    return;
                }
            }
        }
    }

    fn tokenize_script_data_double_escape_start(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) |
                Char(Some('/'), _) |
                Char(Some('>'), _) => {
                    if self.temp_buffer == "script" {
                        self.switch_to(State::ScriptDataDoubleEscaped);
                    } else {
                        self.switch_to(State::ScriptDataEscaped);
                    }
                    if let Char(Some(c), _) = ch {
                        self.append_char_to_text(c);
                    }
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_temp(c.to_ascii_lowercase());
                    self.append_char_to_text(c);
                }
                Char(Some(c), _) if c.is_ascii_lowercase() => {
                    self.append_char_to_temp(c);
                    self.append_char_to_text(c);
                }
                _ => {
                    self.reconsume_in(ch, State::ScriptDataEscaped);
                    return;
                }
            }
        }
    }

    fn tokenize_script_data_double_escaped(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('-'), _) => {
                    self.switch_to(State::ScriptDataDoubleEscapedDash);
                    self.append_char_to_text('-');
                    return;
                }
                Char(Some('<'), _) => {
                    self.switch_to(State::ScriptDataDoubleEscapedLessThanSign);
                    self.append_char_to_text('<');
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(
                        ErrorCode::EofInScriptHtmlCommentLikeText, location);
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_script_data_double_escaped_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::ScriptDataDoubleEscapedDashDash);
                self.append_char_to_text('-');
            }
            Char(Some('<'), _) => {
                self.switch_to(State::ScriptDataDoubleEscapedLessThanSign);
                self.append_char_to_text('<');
            }
            Char(Some('\0'), location) => {
                self.emit_error(ErrorCode::UnexpectedNullCharacter, location);
                self.switch_to(State::ScriptDataDoubleEscaped);
                self.append_char_to_text(char::REPLACEMENT_CHARACTER);
            }
            Char(None, location) => {
                self.emit_error(
                    ErrorCode::EofInScriptHtmlCommentLikeText, location);
                    self.emit_token_if_exists();
                self.switch_to(State::End);
            }
            Char(Some(c), _) => {
                self.switch_to(State::ScriptDataDoubleEscaped);
                self.append_char_to_text(c);
            }
        }
    }

    fn tokenize_script_data_double_escaped_dash_dash(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('-'), _) => {
                    self.append_char_to_text('-');
                }
                Char(Some('<'), _) => {
                    self.switch_to(State::ScriptDataDoubleEscapedLessThanSign);
                    self.append_char_to_text('<');
                    return;
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::ScriptData);
                    self.append_char_to_text('>');
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.switch_to(State::ScriptDataDoubleEscaped);
                    self.append_char_to_text(char::REPLACEMENT_CHARACTER);
                    return;
                }
                Char(None, location) => {
                    self.emit_error(
                        ErrorCode::EofInScriptHtmlCommentLikeText, location);
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.switch_to(State::ScriptDataDoubleEscaped);
                    self.append_char_to_text(c);
                    return;
                }
            }
        }
    }

    fn tokenize_script_data_double_escaped_less_than_sign(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('/'), _) => {
                self.reset_temp();
                self.switch_to(State::ScriptDataDoubleEscapeEnd);
                self.append_char_to_text('/');
            }
            _ => {
                self.reconsume_in(ch, State::ScriptDataDoubleEscaped);
            }
        }
    }

    fn tokenize_script_data_double_escape_end(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) |
                Char(Some('/'), _) |
                Char(Some('>'), _) => {
                    if self.temp_buffer == "script" {
                        self.switch_to(State::ScriptDataEscaped);
                    } else {
                        self.switch_to(State::ScriptDataDoubleEscaped);
                    }
                    if let Char(Some(c), _) = ch {
                        self.append_char_to_text(c);
                    }
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_temp(c.to_ascii_lowercase());
                    self.append_char_to_text(c);
                }
                Char(Some(c), _) if c.is_ascii_lowercase() => {
                    self.append_char_to_temp(c);
                    self.append_char_to_text(c);
                }
                _ => {
                    self.reconsume_in(ch, State::ScriptDataDoubleEscaped);
                    return;
                }
            }
        }
    }

    fn tokenize_before_attribute_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('/'), _) |
                Char(Some('>'), _) => {
                    self.reconsume_in(ch, State::AfterAttributeName);
                    return;
                }
                Char(None, _) => {
                    self.switch_to(State::AfterAttributeName);
                    return;
                }
                Char(Some('='), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedEqualsSignBeforeAttributeName,
                        location);
                    self.start_new_attr();
                    self.append_char_to_attr_name('=');
                    self.switch_to(State::AttributeName);
                    return;
                }
                _ => {
                    self.start_new_attr();
                    self.reconsume_in(ch, State::AttributeName);
                    return;
                }
            }
        }
    }

    fn check_duplicate_attr(&mut self, location: Location) {
        let last = self.tag.attrs.last().expect("");
        let last_name = &self.char_buffer[last.name.clone()];
        for attr in &self.tag.attrs[0..self.tag.attrs.len() - 1] {
            let name = &self.char_buffer[attr.name.clone()];
            if name == last_name {
                self.emit_error(ErrorCode::DuplicateAttribute, location);
                self.tag.attrs.last_mut().expect("").duplicate = true;
                return;
            }
        }
    }

    fn tokenize_attribute_name(&mut self) {
        const UNEXPECTED_CHARS: &[char] = &['\"', '\'', '<'];

        // TODO: duplicate-attribute

        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), location) |
                Char(Some('\n'), location) |
                Char(Some('\x0C'), location) |
                Char(Some(' '), location) |
                Char(Some('/'), location) |
                Char(Some('>'), location) => {
                    self.check_duplicate_attr(location);
                    self.reconsume_in(ch, State::AfterAttributeName);
                    return;
                }
                Char(None, location) => {
                    self.check_duplicate_attr(location);
                    self.switch_to(State::AfterAttributeName);
                    return;
                }
                Char(Some('='), location) => {
                    self.check_duplicate_attr(location);
                    self.switch_to(State::BeforeAttributeValue);
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_attr_name(c.to_ascii_lowercase());
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_attr_name(char::REPLACEMENT_CHARACTER);
                }
                Char(Some(c), location) if UNEXPECTED_CHARS.contains(&c) => {
                    self.emit_error(
                        ErrorCode::UnexpectedCharacterInAttributeName,
                        location);
                    self.append_char_to_attr_name(c);
                }
                Char(Some(c), _) => {
                    self.append_char_to_attr_name(c);
                }
            }
        }
    }

    fn tokenize_after_attribute_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('/'), _) => {
                    self.switch_to(State::SelfClosingTag);
                    return;
                }
                Char(Some('='), _) => {
                    self.switch_to(State::BeforeAttributeValue);
                    return;
                }
                Char(Some('>'), location) => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInTag, location);
                    self.switch_to(State::End);
                    return;
                }
                _ => {
                    self.start_new_attr();
                    self.reconsume_in(ch, State::AttributeName);
                    return;
                }
            }
        }
    }

    fn tokenize_before_attribute_value(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('\"'), _) => {
                    self.switch_to(State::AttributeValueDoubleQuoted);
                    return;
                }
                Char(Some('\''), _) => {
                    self.switch_to(State::AttributeValueSingleQuoted);
                    return;
                }
                Char(Some('>'), location) => {
                    self.emit_error(ErrorCode::MissingAttributeValue, location);
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                _ => {
                    self.reconsume_in(ch, State::AttributeValueUnquoted);
                    return;
                }
            }
        }
    }

    fn tokenize_attribute_value_double_quoted(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\"'), _) => {
                    self.switch_to(State::AfterAttributeValueQuoted);
                    return;
                }
                Char(Some('&'), _) => {
                    self.return_state = State::AttributeValueDoubleQuoted;
                    self.switch_to(State::CharacterReference);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_attr_value(char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInTag, location);
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_attr_value(c);
                }
            }
        }
    }

    fn tokenize_attribute_value_single_quoted(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\''), _) => {
                    self.switch_to(State::AfterAttributeValueQuoted);
                    return;
                }
                Char(Some('&'), _) => {
                    self.return_state = State::AttributeValueSingleQuoted;
                    self.switch_to(State::CharacterReference);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_attr_value(char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInTag, location);
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_attr_value(c);
                }
            }
        }
    }

    fn tokenize_attribute_value_unquoted(&mut self) {
        const UNEXPECTED_CHARS: [char; 5] = ['\"', '\'', '<', '=', '`'];

        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    self.switch_to(State::BeforeAttributeName);
                    return;
                }
                Char(Some('&'), _) => {
                    self.return_state = State::AttributeValueUnquoted;
                    self.switch_to(State::CharacterReference);
                    return;
                }
                Char(Some('>'), location) => {
                    self.switch_to(State::Data);
                    self.emit_tag(location);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_attr_value(char::REPLACEMENT_CHARACTER);
                }
                Char(Some(c), location) if UNEXPECTED_CHARS.contains(&c) => {
                    self.emit_error(
                        ErrorCode::UnexpectedCharacterInUnquotedAttributeValue,
                        location);
                    self.append_char_to_attr_value(c);
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInTag, location);
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_attr_value(c);
                }
            }
        }
    }

    fn tokenize_after_attribute_value_quoted(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('\t'), _) |
            Char(Some('\n'), _) |
            Char(Some('\x0C'), _) |
            Char(Some(' '), _) => {
                self.switch_to(State::BeforeAttributeName);
            }
            Char(Some('/'), _) => {
                self.switch_to(State::SelfClosingTag);
            }
            Char(Some('>'), location) => {
                self.switch_to(State::Data);
                self.emit_tag(location);
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInTag, location);
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceBetweenAttributes, location);
                self.reconsume_in(ch, State::BeforeAttributeName);
            }
        }
    }

    fn tokenize_self_closing_tag(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('>'), location) => {
                self.tag.self_closing = true;
                self.switch_to(State::Data);
                self.emit_tag(location);
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInTag, location);
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(ErrorCode::UnexpectedSolidusInTag, location);
                self.reconsume_in(ch, State::BeforeAttributeName);
            }
        }
    }

    fn tokenize_bogus_comment(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_comment();
                    return;
                }
                Char(None, _) => {
                    self.switch_to(State::End);
                    self.emit_comment();
                    return;
                }
                Char(Some('\0'), location)  => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_comment(char::REPLACEMENT_CHARACTER);
                }
                Char(Some(c), _) => {
                    self.append_char_to_comment(c);
                }
            }
        }
    }

    fn tokenize_markup_declaration(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.reset_temp();
                self.append_char_to_temp('-');
                self.switch_to(State::MaybeCommentStart);
            }
            Char(Some(c), _) if c == 'd' || c == 'D' => {
                self.reset_temp();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctype1);
            }
            Char(Some('['), _) => {
                self.reset_temp();
                self.append_char_to_temp('[');
                self.switch_to(State::MaybeCdataSection1);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment, location);
                // TODO: Create a comment token whose data is the empty string
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_comment_start(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::CommentStart);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment, location.offset(-1));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_comment_start(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::CommentStartDash);
            }
            Char(Some('>'), location) => {
                self.emit_error(
                    ErrorCode::AbruptClosingOfEmptyComment, location);
                self.switch_to(State::Data);
                self.emit_comment();
            }
            _ => {
                self.reconsume_in(ch, State::Comment);
            }
        }
    }

    fn tokenize_comment_start_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::CommentEnd);
            }
            Char(Some('>'), location) => {
                self.emit_error(
                    ErrorCode::AbruptClosingOfEmptyComment, location);
                self.switch_to(State::Data);
                self.emit_comment();
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInComment, location);
                self.switch_to(State::End);
                self.emit_comment();
            }
            _ => {
                self.append_char_to_comment('-');
                self.reconsume_in(ch, State::Comment);
            }
        }
    }

    fn tokenize_comment(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('<'), _) => {
                    self.append_char_to_comment('<');
                    self.switch_to(State::CommentLessThanSign);
                    return;
                }
                Char(Some('-'), _) => {
                    self.switch_to(State::CommentEndDash);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_comment(char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInComment, location);
                    self.switch_to(State::End);
                    self.emit_comment();
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_comment(c);
                }
            }
        }
    }

    fn tokenize_comment_less_than_sign(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('!'), _) => {
                    self.append_char_to_comment('!');
                    self.switch_to(State::CommentLessThanSignBang);
                    return;
                }
                Char(Some('<'), _) => {
                    self.append_char_to_comment('<');
                }
                _ => {
                    self.reconsume_in(ch, State::Comment);
                    return;
                }
            }
        }
    }

    fn tokenize_comment_less_than_sign_bang(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::CommentLessThanSignBangDash);
            }
            _ => {
                self.reconsume_in(ch, State::Comment);
            }
        }
    }

    fn tokenize_comment_less_than_sign_bang_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::CommentLessThanSignBangDashDash);
            }
            _ => {
                self.reconsume_in(ch, State::CommentEndDash);
            }
        }
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#comment-less-than-sign-bang-dash-dash-state
    fn tokenize_comment_less_than_sign_bang_dash_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('>'), _) |
            Char(None, _) => {
                self.reconsume_in(ch, State::CommentEnd);
            }
            Char(_, location) => {
                self.emit_error(ErrorCode::NestedComment, location);
                self.reconsume_in(ch, State::CommentEnd);
            }
        }
    }

    fn tokenize_comment_end_dash(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.switch_to(State::CommentEnd);
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInComment, location);
                self.switch_to(State::End);
                self.emit_comment();
            }
            _ => {
                self.append_char_to_comment('-');
                self.reconsume_in(ch, State::Comment);
            }
        }
    }

    fn tokenize_comment_end(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_comment();
                    return;
                }
                Char(Some('!'), _) => {
                    self.switch_to(State::CommentEndBang);
                    return;
                }
                Char(Some('-'), _) => {
                    self.append_char_to_comment('-');
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInComment, location);
                    self.switch_to(State::End);
                    self.emit_comment();
                    return;
                }
                _ => {
                    self.append_str_to_comment("--");
                    self.reconsume_in(ch, State::Comment);
                    return;
                }
            }
        }
    }

    fn tokenize_comment_end_bang(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('-'), _) => {
                self.append_str_to_comment("--!");
                self.switch_to(State::CommentEndDash);
            }
            Char(Some('>'), location) => {
                self.emit_error(ErrorCode::IncorrectlyClosedComment, location);
                self.switch_to(State::Data);
                self.emit_comment();
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInComment, location);
                self.switch_to(State::End);
                self.emit_comment();
            }
            _ => {
                self.append_str_to_comment("--!");
                self.reconsume_in(ch, State::Comment);
            }
        }
    }

    fn tokenize_maybe_doctype1(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'o' || c == 'O' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctype2);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment,
                    location.offset(-1));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.switch_to(State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_doctype2(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'c' || c == 'C' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctype3);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment,
                    location.offset(-2));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.switch_to(State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_doctype3(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 't' || c == 'T' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctype4);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment,
                    location.offset(-3));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.switch_to(State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_doctype4(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'y' || c == 'Y' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctype5);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment,
                    location.offset(-4));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.switch_to(State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_doctype5(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'p' || c == 'P' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctype6);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment,
                    location.offset(-5));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.switch_to(State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_doctype6(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'e' || c == 'E' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::Doctype);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::IncorrectlyOpenedComment,
                    location.offset(-6));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.switch_to(State::BogusComment);
            }
        }
    }

    fn tokenize_doctype(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('\t'), _) |
            Char(Some('\n'), _) |
            Char(Some('\x0C'), _) |
            Char(Some(' '), _) => {
                self.switch_to(State::BeforeDoctypeName)
            }
            Char(Some('>'), _) => {
                self.reconsume_in(ch, State::BeforeDoctypeName);
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInDoctype, location);
                self.create_doctype();
                self.doctype.force_quirks = true;
                self.emit_docype();
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceBeforeDoctypeName, location);
                self.reconsume_in(ch, State::BeforeDoctypeName);
            }
        }
    }

    fn tokenize_before_doctype_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.create_doctype();
                    self.start_doctype_name();
                    self.append_char_to_doctype_name(c.to_ascii_lowercase());
                    self.switch_to(State::DoctypeName);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.create_doctype();
                    self.start_doctype_name();
                    self.append_char_to_doctype_name(
                        char::REPLACEMENT_CHARACTER);
                    self.switch_to(State::DoctypeName);
                    return;
                }
                Char(Some('>'), location) => {
                    self.emit_error(ErrorCode::MissingDoctypeName, location);
                    self.create_doctype();
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.create_doctype();
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.create_doctype();
                    self.start_doctype_name();
                    self.append_char_to_doctype_name(c);
                    self.switch_to(State::DoctypeName);
                    return;
                }
            }
        }
    }

    fn tokenize_doctype_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    self.switch_to(State::AfterDoctypeName);
                    return;
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(Some(c), _) if c.is_ascii_uppercase() => {
                    self.append_char_to_doctype_name(c.to_ascii_lowercase());
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_doctype_name(
                        char::REPLACEMENT_CHARACTER);
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_doctype_name(c);
                }
            }
        }
    }

    fn tokenize_after_doctype_name(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) if c == 'p' || c == 'P' => {
                    self.reset_temp();
                    self.append_char_to_temp(c);
                    self.switch_to(State::MaybeDoctypePublicKeyword1);
                    return;
                }
                Char(Some(c), _) if c == 's' || c == 'S' => {
                    self.reset_temp();
                    self.append_char_to_temp(c);
                    self.switch_to(State::MaybeDoctypeSystemKeyword1);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                        location);
                    self.doctype.force_quirks = true;
                    self.reconsume_in(ch, State::BogusDoctype);
                    return;
                }
            }
        }
    }

    fn tokenize_maybe_doctype_public_keyword1(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'u' || c == 'U' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypePublicKeyword2);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-1));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_public_keyword2(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'b' || c == 'B' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypePublicKeyword3);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-2));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_public_keyword3(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'l' || c == 'L' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypePublicKeyword4);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-3));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_public_keyword4(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'i' || c == 'I' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypePublicKeyword5);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-4));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_public_keyword5(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'c' || c == 'C' => {
                self.consume_code_point();
                self.reset_temp();
                self.switch_to(State::AfterDoctypePublicKeyword);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-5));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_after_doctype_public_keyword(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('\t'), _) |
            Char(Some('\n'), _) |
            Char(Some('\x0C'), _) |
            Char(Some(' '), _) => {
                self.switch_to(State::BeforeDoctypePublicIdentifier);
            }
            Char(Some('"'), location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceAfterDoctypePublicKeyword,
                    location);
                self.doctype.force_quirks = true;
                self.start_doctype_public_id();
                self.switch_to(State::DoctypePublicIdentifierDoubleQuoted);
            }
            Char(Some('\''), location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceAfterDoctypePublicKeyword,
                    location);
                self.start_doctype_public_id();
                self.switch_to(State::DoctypePublicIdentifierSingleQuoted);
            }
            Char(Some('>'), location) => {
                self.emit_error(
                    ErrorCode::MissingDoctypePublicIdentifier, location);
                self.doctype.force_quirks = true;
                self.switch_to(State::Data);
                self.emit_docype();
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInDoctype, location);
                self.doctype.force_quirks = true;
                self.emit_docype();
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::MissingQuoteBeforeDoctypePublicIdentifier,
                    location);
                self.doctype.force_quirks = true;
                self.reconsume_in(ch, State::BogusDoctype);
            }
        }
    }

    fn tokenize_before_doctype_public_identifier(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('"'), _) => {
                    self.start_doctype_public_id();
                    self.switch_to(State::DoctypePublicIdentifierDoubleQuoted);
                    return;
                }
                Char(Some('\''), _) => {
                    self.start_doctype_public_id();
                    self.switch_to(State::DoctypePublicIdentifierSingleQuoted);
                    return;
                }
                Char(Some('>'), location) => {
                    self.emit_error(
                        ErrorCode::MissingDoctypePublicIdentifier, location);
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::MissingQuoteBeforeDoctypePublicIdentifier,
                        location);
                    self.doctype.force_quirks = true;
                    self.reconsume_in(ch, State::BogusDoctype);
                    return;
                }
            }
        }
    }

    fn tokenize_doctype_public_identifier_double_quoted(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('"'), _) => {
                    self.switch_to(State::AfterDoctypePublicIdentifier);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_doctype_public_id(
                        char::REPLACEMENT_CHARACTER);
                }
                Char(Some('>'), location) => {
                    self.emit_error(
                        ErrorCode::AbruptDoctypePublicIdentifier, location);
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_doctype_public_id(c);
                }
            }
        }
    }

    fn tokenize_doctype_public_identifier_single_quoted(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\''), _) => {
                    self.switch_to(State::AfterDoctypePublicIdentifier);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_doctype_public_id(
                        char::REPLACEMENT_CHARACTER);
                }
                Char(Some('>'), location) => {
                    self.emit_error(
                        ErrorCode::AbruptDoctypePublicIdentifier, location);
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_doctype_public_id(c);
                }
            }
        }
    }

    fn tokenize_after_doctype_public_identifier(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('\t'), _) |
            Char(Some('\n'), _) |
            Char(Some('\x0C'), _) |
            Char(Some(' '), _) => {
                self.switch_to(State::BetweenDoctypePublicAndSystemIdentifiers);
            }
            Char(Some('>'), _) => {
                self.switch_to(State::Data);
                self.emit_docype();
            }
            Char(Some('"'), location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers,
                    location);
                self.start_doctype_system_id();
                self.switch_to(State::DoctypeSystemIdentifierDoubleQuoted);
            }
            Char(Some('\''), location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers,
                    location);
                self.start_doctype_system_id();
                self.switch_to(State::DoctypeSystemIdentifierSingleQuoted);
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInDoctype, location);
                self.doctype.force_quirks = true;
                self.emit_docype();
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::MissingQuoteBeforeDoctypeSystemIdentifier,
                    location);
                self.doctype.force_quirks = true;
                self.reconsume_in(ch, State::BogusDoctype);
            }
        }
    }

    fn tokenize_between_doctype_public_and_system_identifiers(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(Some('"'), _) => {
                    self.start_doctype_system_id();
                    self.switch_to(State::DoctypeSystemIdentifierDoubleQuoted);
                    return;
                }
                Char(Some('\''), _) => {
                    self.start_doctype_system_id();
                    self.switch_to(State::DoctypeSystemIdentifierSingleQuoted);
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::MissingQuoteBeforeDoctypeSystemIdentifier,
                        location);
                    self.doctype.force_quirks = true;
                    self.reconsume_in(ch, State::BogusDoctype);
                    return;
                }
            }
        }
    }

    fn tokenize_maybe_doctype_system_keyword1(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'y' || c == 'Y' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypeSystemKeyword2);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-1));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_system_keyword2(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 's' || c == 'S' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypeSystemKeyword3);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-2));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_system_keyword3(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 't' || c == 'T' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypeSystemKeyword4);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-3));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_system_keyword4(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'e' || c == 'E' => {
                self.consume_code_point();
                self.append_char_to_temp(c);
                self.switch_to(State::MaybeDoctypeSystemKeyword5);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-4));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_maybe_doctype_system_keyword5(&mut self) {
        self.fetch_code_point();
        let ch = self.peek_char();
        match ch {
            Char(Some(c), _) if c == 'm' || c == 'M' => {
                self.consume_code_point();
                self.reset_temp();
                self.switch_to(State::AfterDoctypeSystemKeyword);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::InvalidCharacterSequenceAfterDoctypeName,
                    location.offset(-5));
                self.doctype.force_quirks = true;
                self.switch_to(State::BogusDoctype);
            }
        }
    }

    fn tokenize_after_doctype_system_keyword(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('\t'), _) |
            Char(Some('\n'), _) |
            Char(Some('\x0C'), _) |
            Char(Some(' '), _) => {
                self.switch_to(State::BeforeDoctypeSystemIdentifier);
            }
            Char(Some('"'), location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceAfterDoctypeSystemKeyword,
                    location);
                self.start_doctype_system_id();
                self.switch_to(State::DoctypeSystemIdentifierDoubleQuoted);
            }
            Char(Some('\''), location) => {
                self.emit_error(
                    ErrorCode::MissingWhitespaceAfterDoctypeSystemKeyword,
                    location);
                self.start_doctype_system_id();
                self.switch_to(State::DoctypeSystemIdentifierSingleQuoted);
            }
            Char(Some('>'), location) => {
                self.emit_error(
                    ErrorCode::MissingDoctypeSystemIdentifier, location);
                self.doctype.force_quirks = true;
                self.switch_to(State::Data);
                self.emit_docype();
            }
            Char(None, location) => {
                self.emit_error(ErrorCode::EofInDoctype, location);
                self.doctype.force_quirks = true;
                self.emit_docype();
                self.switch_to(State::End);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::MissingQuoteBeforeDoctypeSystemIdentifier,
                    location);
                self.doctype.force_quirks = true;
                self.reconsume_in(ch, State::BogusDoctype);
            }
        }
    }

    fn tokenize_before_doctype_system_identifier(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('"'), _) => {
                    self.start_doctype_system_id();
                    self.switch_to(State::DoctypeSystemIdentifierDoubleQuoted);
                    return;
                }
                Char(Some('\''), _) => {
                    self.start_doctype_system_id();
                    self.switch_to(State::DoctypeSystemIdentifierSingleQuoted);
                    return;
                }
                Char(Some('>'), location) => {
                    self.emit_error(
                        ErrorCode::MissingDoctypeSystemIdentifier, location);
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::MissingQuoteBeforeDoctypeSystemIdentifier,
                        location);
                    self.doctype.force_quirks = true;
                    self.reconsume_in(ch, State::BogusDoctype);
                    return;
                }
            }
        }
    }

    fn tokenize_doctype_system_identifier_double_quoted(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('"'), _) => {
                    self.switch_to(State::AfterDoctypeSystemIdentifier);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_doctype_system_id(
                        char::REPLACEMENT_CHARACTER);
                }
                Char(Some('>'), location) => {
                    self.emit_error(
                        ErrorCode::AbruptDoctypeSystemIdentifier, location);
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_doctype_system_id(c);
                }
            }
        }
    }

    fn tokenize_doctype_system_identifier_single_quoted(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\''), _) => {
                    self.switch_to(State::AfterDoctypeSystemIdentifier);
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    self.append_char_to_doctype_system_id(
                        char::REPLACEMENT_CHARACTER);
                }
                Char(Some('>'), location) => {
                    self.emit_error(
                        ErrorCode::AbruptDoctypeSystemIdentifier, location);
                    self.doctype.force_quirks = true;
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_doctype_system_id(c);
                }
            }
        }
    }

    fn tokenize_after_doctype_system_identifier(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('\t'), _) |
                Char(Some('\n'), _) |
                Char(Some('\x0C'), _) |
                Char(Some(' '), _) => {
                    // Ignore the character.
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInDoctype, location);
                    self.doctype.force_quirks = true;
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedCharacterAfterDoctypeSystemIdentifier,
                        location);
                    self.reconsume_in(ch, State::BogusDoctype);
                    return;
                }
            }
        }
    }

    fn tokenize_bogus_doctype(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    self.emit_docype();
                    return;
                }
                Char(Some('\0'), location) => {
                    self.emit_error(
                        ErrorCode::UnexpectedNullCharacter, location);
                    // Ignore the character.
                }
                Char(None, _) => {
                    self.emit_docype();
                    self.switch_to(State::End);
                    return;
                }
                _ => {
                    // Ignore the character.
                }
            }
        }
    }

    fn tokenize_maybe_cdata_section1(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('C'), _) => {
                self.append_char_to_temp('C');
                self.switch_to(State::MaybeCdataSection2);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::CdataInHtmlContent, location.offset(-1));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_cdata_section2(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('D'), _) => {
                self.append_char_to_temp('D');
                self.switch_to(State::MaybeCdataSection3);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::CdataInHtmlContent, location.offset(-2));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_cdata_section3(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('A'), _) => {
                self.append_char_to_temp('A');
                self.switch_to(State::MaybeCdataSection4);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::CdataInHtmlContent, location.offset(-3));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_cdata_section4(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('T'), _) => {
                self.append_char_to_temp('T');
                self.switch_to(State::MaybeCdataSection5);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::CdataInHtmlContent, location.offset(-4));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_cdata_section5(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('A'), _) => {
                self.append_char_to_temp('A');
                self.switch_to(State::MaybeCdataSection6);
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::CdataInHtmlContent, location.offset(-6));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_maybe_cdata_section6(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some('['), location) => {
                if self.last_start_tag.is_none() {
                    self.emit_error(ErrorCode::CdataInHtmlContent, location);
                    // TODO: Create a comment token whose data is the empty string
                    self.append_temp_to_comment();
                    self.reconsume_in(ch, State::BogusComment);
                } else {
                    self.switch_to(State::CdataSection);
                }
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::CdataInHtmlContent, location.offset(-7));
                // TODO: Create a comment token whose data is the empty string
                self.append_temp_to_comment();
                self.reconsume_in(ch, State::BogusComment);
            }
        }
    }

    fn tokenize_cdata_section(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some(']'), _) => {
                    self.switch_to(State::CdataSectionBracket);
                    return;
                }
                Char(None, location) => {
                    self.emit_error(ErrorCode::EofInCdata, location);
                    self.emit_token_if_exists();
                    self.switch_to(State::End);
                    return;
                }
                Char(Some(c), _) => {
                    self.append_char_to_text(c);
                }
            }
        }
    }

    fn tokenize_cdata_section_bracket(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(']'), _) => {
                self.switch_to(State::CdataSectionEnd);
            }
            _ => {
                self.append_char_to_text(']');
                self.reconsume_in(ch, State::CdataSection);
            }
        }
    }

    fn tokenize_cdata_section_end(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some(']'), _) => {
                    self.append_char_to_text(']');
                }
                Char(Some('>'), _) => {
                    self.switch_to(State::Data);
                    return;
                }
                _ => {
                    self.append_str_to_text("]]");
                    self.reconsume_in(ch, State::CdataSection);
                    return;
                }
            }
        }
    }

    fn tokenize_character_reference(&mut self) {
        self.reset_temp();
        self.append_char_to_temp('&');

        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_alphanumeric() => {
                self.reconsume_in(ch, State::NamedCharacterReference)
            }
            Char(Some('#'), _) => {
                self.append_char_to_temp('#');
                self.switch_to(State::NumericCharacterReference)
            }
            _ => {
                if self.does_append_to_attr_value() {
                    self.append_temp_to_attr_value();
                } else {
                    self.append_temp_to_text();
                }
                self.reconsume_in(ch, self.return_state)
            }
        }
    }

    fn does_append_to_attr_value(&self) -> bool {
        match self.return_state {
            State::AttributeValueDoubleQuoted |
            State::AttributeValueSingleQuoted |
            State::AttributeValueUnquoted => true,
            _ => false,
        }
    }

    fn tokenize_named_character_reference(&mut self) {
        self.char_ref_resolver.reset();

        // Assumed that next_char is available.
        debug_assert!(self.next_char.is_some());
        let base_location = self.next_char.as_ref().expect("").1;

        loop {
            let ch = self.next_char();
            let has_remaining = !self.char_ref_resolver.remaining().is_empty();
            let (accepted, special_case) = match ch {
                Char(Some(c), _) => (
                    self.char_ref_resolver.accept(c),
                    has_remaining ||
                        c == ';' || c == '=' || c.is_ascii_alphanumeric()),
                Char(None, _) => (false, has_remaining),
            };
            if self.char_ref_resolver.end() {
                debug_assert!(self.char_ref_resolver.remaining().is_empty());
                let (_, chars) = self.char_ref_resolver.resolve().expect("");
                if self.does_append_to_attr_value() {
                    self.append_str_to_attr_value(chars);
                } else {
                    self.append_str_to_text(chars);
                }
                self.switch_to(self.return_state);
                return;
            }
            if !accepted {
                if self.does_append_to_attr_value() && special_case {
                    self.append_temp_to_attr_value();
                    self.append_str_to_attr_value(
                        self.char_ref_resolver.buffer());
                    self.reconsume_in(ch, self.return_state);
                    return;
                }
                if let Some((char_ref, chars)) = self.char_ref_resolver.resolve() {
                    self.emit_error(
                        ErrorCode::MissingSemicolonAfterCharacterReference,
                        base_location.offset(
                            char_ref.len().try_into().unwrap()));
                    if self.does_append_to_attr_value() {
                        self.append_str_to_attr_value(chars);
                        self.append_str_to_attr_value(
                            self.char_ref_resolver.remaining());
                    } else {
                        self.append_str_to_text(chars);
                        self.append_str_to_text(
                            self.char_ref_resolver.remaining());
                    }
                    self.reconsume_in(ch, self.return_state);
                    return;
                }
                if self.does_append_to_attr_value() {
                    self.append_temp_to_attr_value();
                    self.append_str_to_attr_value(
                        self.char_ref_resolver.remaining());
                } else {
                    self.append_temp_to_text();
                    self.append_str_to_text(
                        self.char_ref_resolver.remaining());
                }
                self.reconsume_in(ch, State::AmbigousAmpersand);
                return;
            }
        }
    }

    fn tokenize_ambigous_ampersand(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some(c), _) if c.is_ascii_alphanumeric() => {
                    if self.does_append_to_attr_value() {
                        self.append_char_to_attr_value(c);
                    } else {
                        self.append_char_to_text(c);
                    }
                }
                Char(Some(';'), location) => {
                    self.emit_error(
                        ErrorCode::UnknownNamedCharacterReference, location);
                    self.reconsume_in(ch, self.return_state);
                    return;
                }
                _ => {
                    self.reconsume_in(ch, self.return_state);
                    return;
                }
            }
        }
    }

    fn tokenize_numeric_character_reference(&mut self) {
        self.char_ref_code = 0;

        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c == 'x' || c == 'X' => {
                self.append_char_to_temp(c);
                self.switch_to(State::HexadecimalCharacterReferenceStart)
            }
            _ => {
                self.reconsume_in(ch, State::DecimalCharacterReferenceStart)
            }
        }
    }

    fn tokenize_hexadecimal_character_reference_start(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_hexdigit() => {
                self.reconsume_in(ch, State::HexadecimalCharacterReference)
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::AbsenceOfDigitsInNumericCharacterReference,
                    location);
                self.append_temp_to_text();
                self.reconsume_in(ch, self.return_state)
            }
        }
    }

    fn tokenize_decimal_character_reference_start(&mut self) {
        let ch = self.next_char();
        match ch {
            Char(Some(c), _) if c.is_ascii_digit() => {
                self.reconsume_in(ch, State::DecimalCharacterReference)
            }
            Char(_, location) => {
                self.emit_error(
                    ErrorCode::AbsenceOfDigitsInNumericCharacterReference,
                    location);
                self.append_temp_to_text();
                self.reconsume_in(ch, self.return_state)
            }
        }
    }

    fn tokenize_hexadecimal_character_reference(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some(c), _) if c.is_ascii_hexdigit() => {
                    let digit = c.to_digit(16).unwrap();
                    self.char_ref_code = self.char_ref_code.saturating_mul(16)
                        .saturating_add(digit);
                }
                Char(Some(';'),  _) => {
                    self.switch_to(State::NumericCharacterReferenceEnd);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::MissingSemicolonAfterCharacterReference,
                        location);
                    self.reconsume_in(ch, State::NumericCharacterReferenceEnd);
                    return;
                }
            }
        }
    }

    fn tokenize_decimal_character_reference(&mut self) {
        loop {
            let ch = self.next_char();
            match ch {
                Char(Some(c), _) if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap();
                    self.char_ref_code = self.char_ref_code.saturating_mul(10)
                        .saturating_add(digit);
                }
                Char(Some(';'), _) => {
                    self.switch_to(State::NumericCharacterReferenceEnd);
                    return;
                }
                Char(_, location) => {
                    self.emit_error(
                        ErrorCode::MissingSemicolonAfterCharacterReference,
                        location);
                    self.reconsume_in(ch, State::NumericCharacterReferenceEnd);
                    return;
                }
            }
        }
    }

    fn tokenize_numeric_character_reference_end(&mut self) {
        const CHARMAP_C1: [char; 32] = [
            '\u{20AC}', '\u{0081}', '\u{201A}', '\u{0192}',
            '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
            '\u{02C6}', '\u{2030}', '\u{0160}', '\u{2039}',
            '\u{0152}', '\u{008D}', '\u{017D}', '\u{008F}',
            '\u{0090}', '\u{2018}', '\u{2019}', '\u{201C}',
            '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
            '\u{02DC}', '\u{2122}', '\u{0161}', '\u{203A}',
            '\u{0153}', '\u{009D}', '\u{017E}', '\u{0178}',
        ];

        let ch = self.next_char();

        let location = ch.1;

        let c = match self.char_ref_code {
            0 => {
                self.emit_error(
                    ErrorCode::NullCharacterReference, location);
                char::REPLACEMENT_CHARACTER
            }
            0x110000.. => {
                self.emit_error(
                    ErrorCode::CharacterReferenceOutsideUnicodeRange,
                    location);
                char::REPLACEMENT_CHARACTER
            }
            0xD800..=0xDFFF => {
                self.emit_error(
                    ErrorCode::SurrogateCharacterReference, location);
                char::REPLACEMENT_CHARACTER
            }
            0xFDD0..=0xFDEF => {
                self.emit_error(
                    ErrorCode::NoncharacterCharacterReference, location);
                char::from_u32(self.char_ref_code).expect("")
            }
            code if (code & 0xFFFF) == 0xFFFE => {
                self.emit_error(
                    ErrorCode::NoncharacterCharacterReference, location);
                char::from_u32(self.char_ref_code).expect("")
            }
            code if (code & 0xFFFF) == 0xFFFF => {
                self.emit_error(
                    ErrorCode::NoncharacterCharacterReference, location);
                char::from_u32(self.char_ref_code).expect("")
            }
            // CARRIAGE RETURN
            0x0D |
            // C0 except for ASCII whitespace
            0x01..=0x08 |
            0x0B |
            0x0E..=0x1F |
            // DELETE
            0x7F => {
                self.emit_error(
                    ErrorCode::ControlCharacterReference, location);
                char::from_u32(self.char_ref_code).expect("")
            }
            0x80..=0x9F => {
                self.emit_error(
                    ErrorCode::ControlCharacterReference, location);
                CHARMAP_C1[self.char_ref_code as usize - 0x80]
            }
            _ => {
                char::from_u32(self.char_ref_code).expect("")
            }
        };
        if self.does_append_to_attr_value() {
            self.append_char_to_attr_value(c);
        } else {
            self.append_char_to_text(c);
        }
        self.reconsume_in(ch, self.return_state);
    }

    fn next_char(&mut self) -> Char {
        if let Some(ch) = self.next_char.take() {
            ch
        } else {
            let cp = if self.next_code_point.is_some() {
                self.next_code_point.take()
            } else {
                self.input_stream.next_code_point()
            };
            match cp {
                Some((CodePoint::Scalar(cp), location)) => {
                    if Self::is_nonchar(cp) {
                        self.emit_error(
                            ErrorCode::NoncharacterInInputStream, location);
                    } else if Self::is_control_other_than_ascii_whitespace(cp) {
                        self.emit_error(
                            ErrorCode::ControlCharacterInInputStream,
                            location);
                    }
                    Char(Some(char::from_u32(cp).expect("")), location)
                }
                Some((CodePoint::Surrogate(_), location)) => {
                    self.emit_error(
                        ErrorCode::SurrogateInInputStream, location);
                    Char(Some(char::REPLACEMENT_CHARACTER), location)
                }
                Some((CodePoint::Eof, location)) => {
                    Char(None, location)
                }
                None => {
                  todo!("");
                }
            }
        }
    }

    fn fetch_code_point(&mut self) {
        self.next_code_point = self.input_stream.next_code_point();
    }

    fn consume_code_point(&mut self) {
        self.next_code_point = None;
    }

    fn peek_char(&self) -> Char {
        match self.next_code_point {
            Some((CodePoint::Scalar(cp), location)) => {
                Char(Some(char::from_u32(cp).expect("")), location)
            }
            Some((CodePoint::Surrogate(_), location)) => {
                Char(Some(char::REPLACEMENT_CHARACTER), location)
            }
            Some((CodePoint::Eof, location)) => {
                Char(None, location)
            }
            None => {
                todo!("");
            }
        }
    }

    fn reconsume_in(&mut self, ch: Char, state: State) {
        self.next_char = Some(ch);
        self.switch_to(state);
    }

    fn switch_to(&mut self, state: State) {
        self.state = state;
    }

    fn create_start_tag(&mut self) {
        self.tag.clear(self.char_buffer.len(), true);
    }

    fn create_end_tag(&mut self) {
        self.tag.clear(self.char_buffer.len(), false);
    }

    fn discard_tag(&mut self) {
        self.char_buffer.truncate(self.tag.name.start);
    }

    // TODO
    // ----
    // Currently, we update the tag.name every time a character is appended to
    // the buffer.  It's inefficient but simple and reliable.
    fn append_char_to_tag_name(&mut self, c: char) {
        self.char_buffer.push(c);
        self.tag.name.end = self.char_buffer.len();
    }

    fn start_new_attr(&mut self) {
        let pos = self.char_buffer.len();
        self.tag.attrs.push(Attr {
            name: pos..pos,
            value: pos..pos,
            duplicate: false,
        });
        self.has_duplicate_attr = false;
    }

    // TODO
    // ----
    // Currently, we update the name and value of the last attribute in the list
    // every time a character is appended to the buffer.  It's inefficient but
    // simple and reliable.
    fn append_char_to_attr_name(&mut self, c: char) {
        self.char_buffer.push(c);
        let attr = self.tag.attrs.last_mut().unwrap();
        let pos = self.char_buffer.len();
        attr.name.end = pos;
        attr.value = pos..pos;
    }

    // TODO
    // ----
    // Currently, we update the value of the last attribute in the list every
    // time a character is appended to the buffer.  It's inefficient but simple
    // and reliable.
    fn append_char_to_attr_value(&mut self, c: char) {
        self.char_buffer.push(c);
        self.tag.attrs.last_mut().unwrap().value.end = self.char_buffer.len();
    }

    // TODO
    // ----
    // Currently, we update the value of the last attribute in the list every
    // time a character is appended to the buffer.  It's inefficient but simple
    // and reliable.
    fn append_str_to_attr_value(&mut self, s: &str) {
        self.char_buffer.push_str(s);
        self.tag.attrs.last_mut().unwrap().value.end = self.char_buffer.len();
    }

    fn emit_tag(&mut self, location: Location) {
        if self.tag.start_tag {
            self.tokens.push_back(Ok(Token::StartTag));
            self.last_start_tag = Some(self.tag_name().to_string());
        } else {
            if !self.tag.attrs.is_empty() {
                self.emit_error(ErrorCode::EndTagWithAttributes, location);
            }
            if self.tag.self_closing {
                self.emit_error(ErrorCode::EndTagWithTrailingSolidus, location);
            }
            self.tokens.push_back(Ok(Token::EndTag));
        }
        self.clear_char_buffer = true;
    }

    fn create_doctype(&mut self) {
        self.doctype.clear();
    }

    fn start_doctype_name(&mut self) {
        let pos = self.char_buffer.len();
        self.doctype.name = Some(pos..pos);
    }

    // TODO
    // ----
    // Currently, we update the tag.name every time a character is appended to
    // the buffer.  It's inefficient but simple and reliable.
    fn append_char_to_doctype_name(&mut self, c: char) {
        self.char_buffer.push(c);
        let pos = self.char_buffer.len();
        self.doctype.name.as_mut().expect("").end = pos;
    }

    fn start_doctype_public_id(&mut self) {
        let pos = self.char_buffer.len();
        self.doctype.public_id = Some(pos..pos);
    }

    fn append_char_to_doctype_public_id(&mut self, c: char) {
        self.char_buffer.push(c);
        let pos = self.char_buffer.len();
        self.doctype.public_id.as_mut().expect("").end = pos;
    }

    fn start_doctype_system_id(&mut self) {
        let pos = self.char_buffer.len();
        self.doctype.system_id = Some(pos..pos);
    }

    fn append_char_to_doctype_system_id(&mut self, c: char) {
        self.char_buffer.push(c);
        let pos = self.char_buffer.len();
        self.doctype.system_id.as_mut().expect("").end = pos;
    }

    fn emit_docype(&mut self) {
        self.tokens.push_back(Ok(Token::Doctype));
        self.clear_char_buffer = true;
    }

    fn append_char_to_comment(&mut self, c: char) {
        self.char_buffer.push(c);
        self.has_comment = true;
    }

    fn append_str_to_comment(&mut self, s: &str) {
        self.char_buffer.push_str(s);
        self.has_comment = true;
    }

    fn emit_comment(&mut self) {
        self.tokens.push_back(Ok(Token::Comment));
        self.has_comment = false;
        self.clear_char_buffer = true;
    }

    fn append_char_to_text(&mut self, c: char) {
        self.char_buffer.push(c);
        self.has_text = true;
    }

    fn append_str_to_text(&mut self, s: &str) {
        self.char_buffer.push_str(s);
        self.has_text = true;
    }

    fn emit_text(&mut self) {
        self.tokens.push_back(Ok(Token::Text));
        self.has_text = false;
        self.clear_char_buffer = true;
    }

    fn emit_token_if_exists(&mut self) {
        if self.has_text {
            self.emit_text();
        } else if self.has_comment {
            self.emit_comment();
        }
    }

    fn reset_temp(&mut self) {
        self.temp_buffer.truncate(0);
    }

    fn append_char_to_temp(&mut self, c: char) {
        self.temp_buffer.push(c);
    }

    fn append_temp_to_text(&mut self) {
        self.char_buffer.push_str(&self.temp_buffer);
        self.has_text = true;
    }

    fn append_temp_to_comment(&mut self) {
        self.char_buffer.push_str(&self.temp_buffer);
        self.has_comment = true;
    }

    fn append_temp_to_attr_value(&mut self) {
        self.char_buffer.push_str(&self.temp_buffer);
        self.tag.attrs.last_mut().unwrap().value.end = self.char_buffer.len();
    }

    fn emit_error(&mut self, code: ErrorCode, location: Location) {
        self.tokens.push_back(Err(Error::new(code, location)));
    }

    fn is_appropriate_end_tag(&self) -> bool {
        if let Some(ref tag_name) = self.last_start_tag {
            if self.tag_name() == tag_name {
                return true;
            }
        }
        false
    }

    #[inline]
    fn is_nonchar(cp: u32) -> bool {
        if (0x00FDD0..=0x00FDEF).contains(&cp) {
            return true;
        }
        if ((cp + 1) & 0x00FFFF) == 0 {  // 0x__FFFF
            return true;
        }
        if ((cp + 2) & 0x00FFFF) == 0 {  // 0x__FFFE
            return true;
        }
        return false;
    }

    #[inline]
    fn is_control_other_than_ascii_whitespace(cp: u32) -> bool {
        match cp {
            // ASCII whitespace
            0x09 | 0x0A | 0x0C | 0x0D | 0x20 => false,
            // C0 except for NULL
            0x01..=0x1F => true,
            // DEL + C1
            0x7F..=0x9F => true,
            // Others
            _ => false
        }
    }
}

struct Char(Option<char>, Location);

pub enum Token {
    Doctype,
    StartTag,
    EndTag,
    Text,
    Comment,
    End,
}

#[derive(Default)]
struct Tag {
    name: Range<usize>,
    attrs: Vec<Attr>,
    start_tag: bool,
    self_closing: bool,
}

impl Tag {
    fn clear(&mut self, pos: usize, start_tag: bool) {
        self.name = pos..pos;
        self.attrs.clear();
        self.start_tag = start_tag;
        self.self_closing = false;
    }
}

#[derive(Default)]
struct Attr {
    name: Range<usize>,
    value: Range<usize>,
    duplicate: bool,
}

#[derive(Default)]
struct Doctype {
    name: Option<Range<usize>>,
    public_id: Option<Range<usize>>,
    system_id: Option<Range<usize>>,
    force_quirks: bool,
}

impl Doctype {
    fn clear(&mut self) {
        self.name = None;
        self.public_id = None;
        self.system_id = None;
        self.force_quirks = false;
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum State {
    Data,
    Rcdata,
    Rawtext,
    ScriptData,
    Plaintext,
    TagOpen,
    EndTagOpen,
    TagName,
    RcdataLessThanSign,
    RcdataEndTagOpen,
    RcdataEndTagName,
    RawtextLessThanSign,
    RawtextEndTagOpen,
    RawtextEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingTag,
    BogusComment,
    MarkupDeclarationOpen,
    MaybeCommentStart,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    MaybeDoctype1,
    MaybeDoctype2,
    MaybeDoctype3,
    MaybeDoctype4,
    MaybeDoctype5,
    MaybeDoctype6,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    MaybeDoctypePublicKeyword1,
    MaybeDoctypePublicKeyword2,
    MaybeDoctypePublicKeyword3,
    MaybeDoctypePublicKeyword4,
    MaybeDoctypePublicKeyword5,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    MaybeDoctypeSystemKeyword1,
    MaybeDoctypeSystemKeyword2,
    MaybeDoctypeSystemKeyword3,
    MaybeDoctypeSystemKeyword4,
    MaybeDoctypeSystemKeyword5,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    MaybeCdataSection1,
    MaybeCdataSection2,
    MaybeCdataSection3,
    MaybeCdataSection4,
    MaybeCdataSection5,
    MaybeCdataSection6,
    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbigousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
    End,
}

pub struct Attrs<'a> {
    tokenizer: &'a Tokenizer,
    index: usize,
}

impl<'a> Attrs<'a> {
    fn new(tokenizer: &'a Tokenizer) -> Self {
        Attrs {
            tokenizer,
            index: 0,
        }
    }
}

impl<'a> Iterator for Attrs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.index;
        while i < self.tokenizer.tag.attrs.len() {
            if !self.tokenizer.tag.attrs[i].duplicate {
                break;
            }
            i += 1;
        }

        let attr = self.tokenizer.tag.attrs
            .get(i)
            .map(|attr| {
                let name = &self.tokenizer.char_buffer[attr.name.clone()];
                let value = &self.tokenizer.char_buffer[attr.value.clone()];
                (name, value)
            });
        self.index = i + 1;
        attr
    }
}
