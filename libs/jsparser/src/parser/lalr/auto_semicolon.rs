// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsparser/src/parser/lalr/auto_semicolon.rs.hbs

use phf::phf_set;
use phf::Set;

pub static DISALLOWED: Set<u16> = phf_set! {
    // State(43)
    //   [EmptyStatement -> SEMICOLON .]*
    43u16,
    // State(1078)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    1078u16,
    // State(1289)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    1289u16,
    // State(1440)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement]*
    1440u16,
    // State(1458)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [_LOOP_INIT_EXPRESSION_ -> (empty) .]*
    1458u16,
    // State(1587)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    1587u16,
    // State(1605)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [_LOOP_INIT_EXPRESSION_ -> (empty) .]*
    1605u16,
    // State(1729)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [_LOOP_TEST_ -> (empty) .]*
    1729u16,
    // State(1735)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [_LOOP_INIT_VAR_DECLARATION_ -> (empty) .]*
    1735u16,
    // State(1765)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement]*
    1765u16,
    // State(1855)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [_LOOP_TEST_ -> (empty) .]*
    1855u16,
    // State(1861)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [_LOOP_INIT_VAR_DECLARATION_ -> (empty) .]*
    1861u16,
    // State(1883)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    1883u16,
    // State(2057)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement]*
    2057u16,
    // State(2068)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2068u16,
    // State(2085)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    2085u16,
    // State(2102)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    2102u16,
    // State(2154)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    2154u16,
    // State(2165)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2165u16,
    // State(2470)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement]*
    2470u16,
    // State(2476)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2476u16,
    // State(2496)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    2496u16,
    // State(2500)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [_LOOP_INIT_EXPRESSION_ -> (empty) .]*
    2500u16,
    // State(2516)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    2516u16,
    // State(2520)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [_LOOP_INIT_EXPRESSION_ -> (empty) .]*
    2520u16,
    // State(2557)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    2557u16,
    // State(2561)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2561u16,
    // State(2927)
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement]*
    //   [ForStatement -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2927u16,
    // State(2946)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2946u16,
    // State(2949)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [_LOOP_INIT_VAR_DECLARATION_ -> (empty) .]*
    2949u16,
    // State(2955)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    2955u16,
    // State(2968)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    2968u16,
    // State(2971)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [_LOOP_INIT_VAR_DECLARATION_ -> (empty) .]*
    2971u16,
    // State(2977)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    2977u16,
    // State(3011)
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3011u16,
    // State(3293)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    3293u16,
    // State(3301)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3301u16,
    // State(3321)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    3321u16,
    // State(3329)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Await _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3329u16,
    // State(3488)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield SEMICOLON _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield SEMICOLON _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    3488u16,
    // State(3542)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    3542u16,
    // State(3546)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3546u16,
    // State(3571)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    3571u16,
    // State(3575)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3575u16,
    // State(3638)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield_Await SEMICOLON _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    3638u16,
    // State(3697)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    3697u16,
    // State(3719)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Yield SEMICOLON _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Yield SEMICOLON _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [_LOOP_INIT_EXPRESSION_ -> (empty) .]*
    3719u16,
    // State(3761)
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In SEMICOLON . _LOOP_TEST_ Expression_In RPAREN _LOOP_NEXT_ Statement_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3761u16,
    // State(3785)
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Await SEMICOLON . _LOOP_TEST_ Expression_In_Await RPAREN _LOOP_NEXT_ Statement_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3785u16,
    // State(3832)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    3832u16,
    // State(3854)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ SEMICOLON Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Yield_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . _LOOP_INIT_EXPRESSION_ Expression_In_Yield_Await SEMICOLON _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [_LOOP_INIT_EXPRESSION_ -> (empty) .]*
    3854u16,
    // State(3902)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield SEMICOLON . _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    3902u16,
    // State(3908)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield SEMICOLON _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield SEMICOLON _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [_LOOP_INIT_VAR_DECLARATION_ -> (empty) .]*
    3908u16,
    // State(3932)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    3932u16,
    // State(4004)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4004u16,
    // State(4010)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ SEMICOLON Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield_Await SEMICOLON _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield_Await SEMICOLON _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [_LOOP_INIT_VAR_DECLARATION_ -> (empty) .]*
    4010u16,
    // State(4034)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield_Await _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield_Await _LOOP_INIT_LEXICAL_DECLARATION_ SEMICOLON . Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    4034u16,
    // State(4075)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    4075u16,
    // State(4086)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Yield SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Yield SEMICOLON . _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4086u16,
    // State(4149)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON _LOOP_INIT_EXPRESSION_ SEMICOLON . Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    4149u16,
    // State(4160)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield_Await _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN LexicalDeclaration_Yield_Await _LOOP_INIT_LEXICAL_DECLARATION_ Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4160u16,
    // State(4188)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    4188u16,
    // State(4192)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Yield SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Yield SEMICOLON . _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4192u16,
    // State(4236)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ SEMICOLON . Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    4236u16,
    // State(4240)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON _LOOP_INIT_EXPRESSION_ Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4240u16,
    // State(4270)
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield SEMICOLON . _LOOP_TEST_ Expression_In_Yield RPAREN _LOOP_NEXT_ Statement_Yield_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4270u16,
    // State(4304)
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR _LOOP_START_ LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON _LOOP_INIT_VAR_DECLARATION_ Expression_In_Yield_Await SEMICOLON . _LOOP_TEST_ Expression_In_Yield_Await RPAREN _LOOP_NEXT_ Statement_Yield_Await_Return]*
    //   [_LOOP_TEST_ -> (empty) .]*
    4304u16,
};

pub static DO_WHILES: Set<u16> = phf_set! {
    // State(2464)
    //   [DoWhileStatement -> DO _LOOP_START_ Statement _LOOP_BODY_ WHILE LPAREN Expression_In RPAREN . SEMICOLON]*
    2464u16,
    // State(2540)
    //   [DoWhileStatement_Await -> DO _LOOP_START_ Statement_Await _LOOP_BODY_ WHILE LPAREN Expression_In_Await RPAREN . SEMICOLON]*
    2540u16,
    // State(3536)
    //   [DoWhileStatement_Return -> DO _LOOP_START_ Statement_Return _LOOP_BODY_ WHILE LPAREN Expression_In RPAREN . SEMICOLON]*
    3536u16,
    // State(3562)
    //   [DoWhileStatement_Await_Return -> DO _LOOP_START_ Statement_Await_Return _LOOP_BODY_ WHILE LPAREN Expression_In_Await RPAREN . SEMICOLON]*
    3562u16,
    // State(4182)
    //   [DoWhileStatement_Yield_Return -> DO _LOOP_START_ Statement_Yield_Return _LOOP_BODY_ WHILE LPAREN Expression_In_Yield RPAREN . SEMICOLON]*
    4182u16,
    // State(4227)
    //   [DoWhileStatement_Yield_Await_Return -> DO _LOOP_START_ Statement_Yield_Await_Return _LOOP_BODY_ WHILE LPAREN Expression_In_Yield_Await RPAREN . SEMICOLON]*
    4227u16,
};
