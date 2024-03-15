// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/htmlparser/tests/html5lib_tests.rs.hbs

mod helper;

use helper::parse;
use helper::Scripting;
use helper::Test;

logging::init!();

#[test]
fn test_0000() {
    parse(Test {
        data: "<isindex>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<isindex>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<isindex name=\"A\" action=\"B\" prompt=\"C\" foo=\"D\">",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<isindex>"),
            (3, "action=\"B\""),
            (3, "foo=\"D\""),
            (3, "name=\"A\""),
            (3, "prompt=\"C\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<form><isindex>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<form>"),
            (3, "<isindex>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html><isindex>x</isindex>x",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<isindex>"),
            (3, "\"x\""),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}