use crate::Location;
use std::fmt;
use thiserror;

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("{location}: {code}")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Error {
    code: ErrorCode,
    location: Location,
}

impl Error {
    pub(crate) fn new(code: ErrorCode, location: Location) -> Self {
        Error { code, location }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum ErrorCode {
    AbruptClosingOfEmptyComment,
    AbruptDoctypePublicIdentifier,
    AbruptDoctypeSystemIdentifier,
    AbsenceOfDigitsInNumericCharacterReference,
    CdataInHtmlContent,
    CharacterReferenceOutsideUnicodeRange,
    ControlCharacterInInputStream,
    ControlCharacterReference,
    DuplicateAttribute,
    EndTagWithAttributes,
    EndTagWithTrailingSolidus,
    EofBeforeTagName,
    EofInCdata,
    EofInComment,
    EofInDoctype,
    EofInScriptHtmlCommentLikeText,
    EofInTag,
    IncorrectlyClosedComment,
    IncorrectlyOpenedComment,
    InvalidCharacterSequenceAfterDoctypeName,
    InvalidFirstCharacterOfTagName,
    MissingAttributeValue,
    MissingDoctypeName,
    MissingDoctypePublicIdentifier,
    MissingDoctypeSystemIdentifier,
    MissingEndTagName,
    MissingQuoteBeforeDoctypePublicIdentifier,
    MissingQuoteBeforeDoctypeSystemIdentifier,
    MissingSemicolonAfterCharacterReference,
    MissingWhitespaceAfterDoctypePublicKeyword,
    MissingWhitespaceAfterDoctypeSystemKeyword,
    MissingWhitespaceBeforeDoctypeName,
    MissingWhitespaceBetweenAttributes,
    MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers,
    NestedComment,
    NoncharacterCharacterReference,
    NoncharacterInInputStream,
    NonVoidHtmlElementStartTagWithTrailingSolidus,
    NullCharacterReference,
    SurrogateCharacterReference,
    SurrogateInInputStream,
    UnexpectedCharacterAfterDoctypeSystemIdentifier,
    UnexpectedCharacterInAttributeName,
    UnexpectedCharacterInUnquotedAttributeValue,
    UnexpectedEqualsSignBeforeAttributeName,
    UnexpectedNullCharacter,
    UnexpectedQuestionMarkInsteadOfTagName,
    UnexpectedSolidusInTag,
    UnknownNamedCharacterReference,
    InsufficientInput,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::AbruptClosingOfEmptyComment => write!(f, "abrupt-closing-of-empty-comment"),
            Self::AbruptDoctypePublicIdentifier => write!(f, "abrupt-doctype-public-identifier"),
            Self::AbruptDoctypeSystemIdentifier => write!(f, "abrupt-doctype-system-identifier"),
            Self::AbsenceOfDigitsInNumericCharacterReference => {
                write!(f, "absence-of-digits-in-numeric-character-reference")
            }
            Self::CdataInHtmlContent => write!(f, "cdata-in-html-content"),
            Self::CharacterReferenceOutsideUnicodeRange => {
                write!(f, "character-reference-outside-unicode-range")
            }
            Self::ControlCharacterInInputStream => write!(f, "control-character-in-input-stream"),
            Self::ControlCharacterReference => write!(f, "control-character-reference"),
            Self::DuplicateAttribute => write!(f, "duplicate-attribute"),
            Self::EndTagWithAttributes => write!(f, "end-tag-with-attributes"),
            Self::EndTagWithTrailingSolidus => write!(f, "end-tag-with-trailing-solidus"),
            Self::EofBeforeTagName => write!(f, "eof-before-tag-name"),
            Self::EofInCdata => write!(f, "eof-in-cdata"),
            Self::EofInComment => write!(f, "eof-in-comment"),
            Self::EofInDoctype => write!(f, "eof-in-doctype"),
            Self::EofInScriptHtmlCommentLikeText => {
                write!(f, "eof-in-script-html-comment-like-text")
            }
            Self::EofInTag => write!(f, "eof-in-tag"),
            Self::IncorrectlyClosedComment => write!(f, "incorrectly-closed-comment"),
            Self::IncorrectlyOpenedComment => write!(f, "incorrectly-opened-comment"),
            Self::InvalidCharacterSequenceAfterDoctypeName => {
                write!(f, "invalid-character-sequence-after-doctype-name")
            }
            Self::InvalidFirstCharacterOfTagName => {
                write!(f, "invalid-first-character-of-tag-name")
            }
            Self::MissingAttributeValue => write!(f, "missing-attribute-value"),
            Self::MissingDoctypeName => write!(f, "missing-doctype-name"),
            Self::MissingDoctypePublicIdentifier => write!(f, "missing-doctype-public-identifier"),
            Self::MissingDoctypeSystemIdentifier => write!(f, "missing-doctype-system-identifier"),
            Self::MissingEndTagName => write!(f, "missing-end-tag-name"),
            Self::MissingQuoteBeforeDoctypePublicIdentifier => {
                write!(f, "missing-quote-before-doctype-public-identifier")
            }
            Self::MissingQuoteBeforeDoctypeSystemIdentifier => {
                write!(f, "missing-quote-before-doctype-system-identifier")
            }
            Self::MissingSemicolonAfterCharacterReference => {
                write!(f, "missing-semicolon-after-character-reference")
            }
            Self::MissingWhitespaceAfterDoctypePublicKeyword => {
                write!(f, "missing-whitespace-after-doctype-public-keyword")
            }
            Self::MissingWhitespaceAfterDoctypeSystemKeyword => {
                write!(f, "missing-whitespace-after-doctype-system-keyword")
            }
            Self::MissingWhitespaceBeforeDoctypeName => {
                write!(f, "missing-whitespace-before-doctype-name")
            }
            Self::MissingWhitespaceBetweenAttributes => {
                write!(f, "missing-whitespace-between-attributes")
            }
            Self::MissingWhitespaceBetweenDoctypePublicAndSystemIdentifiers => write!(
                f,
                "missing-whitespace-between-doctype-public-and-system-identifiers"
            ),
            Self::NestedComment => write!(f, "nested-comment"),
            Self::NoncharacterCharacterReference => write!(f, "noncharacter-character-reference"),
            Self::NoncharacterInInputStream => write!(f, "noncharacter-in-input-stream"),
            Self::NonVoidHtmlElementStartTagWithTrailingSolidus => {
                write!(f, "non-void-html-element-start-tag-with-trailing-solidus")
            }
            Self::NullCharacterReference => write!(f, "null-character-reference"),
            Self::SurrogateCharacterReference => write!(f, "surrogate-character-reference"),
            Self::SurrogateInInputStream => write!(f, "surrogate-in-input-stream"),
            Self::UnexpectedCharacterAfterDoctypeSystemIdentifier => {
                write!(f, "unexpected-character-after-doctype-system-identifier")
            }
            Self::UnexpectedCharacterInAttributeName => {
                write!(f, "unexpected-character-in-attribute-name")
            }
            Self::UnexpectedCharacterInUnquotedAttributeValue => {
                write!(f, "unexpected-character-in-unquoted-attribute-value")
            }
            Self::UnexpectedEqualsSignBeforeAttributeName => {
                write!(f, "unexpected-equals-sign-before-attribute-name")
            }
            Self::UnexpectedNullCharacter => write!(f, "unexpected-null-character"),
            Self::UnexpectedQuestionMarkInsteadOfTagName => {
                write!(f, "unexpected-question-mark-instead-of-tag-name")
            }
            Self::UnexpectedSolidusInTag => write!(f, "unexpected-solidus-in-tag"),
            Self::UnknownNamedCharacterReference => write!(f, "unknown-named-character-reference"),
            Self::InsufficientInput => write!(f, "insufficient-input"),
        }
    }
}
