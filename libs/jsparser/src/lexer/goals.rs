// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsparser/src/lexer/goals.rs.hbs

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Goal {
    InputElementDiv,
    InputElementRegExp,
    InputElementRegExpOrTemplateTail,
    InputElementTemplateTail,
    InputElementHashbangOrRegExp,
}
