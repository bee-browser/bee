// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by using lalr/auto_semicolon.rs.hbs.

use phf::phf_set;
use phf::Set;

pub const DISALLOWED: Set<u16> = phf_set! {
    // State(42)
    //   [EmptyStatement -> SEMI_COLON .]*
    42u16,
    // State(397)
    //   [ForStatement -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    397u16,
    // State(682)
    //   [ForStatement -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    682u16,
    // State(700)
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    700u16,
    // State(715)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement]*
    715u16,
    // State(1030)
    //   [ForStatement -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1030u16,
    // State(1037)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    1037u16,
    // State(1074)
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1074u16,
    // State(1091)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1091u16,
    // State(1423)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1423u16,
    // State(1453)
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1453u16,
    // State(1484)
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1484u16,
    // State(1533)
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1533u16,
    // State(1754)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1754u16,
    // State(1807)
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    1807u16,
    // State(1811)
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1811u16,
    // State(1816)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement_Return]*
    1816u16,
    // State(1845)
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    1845u16,
    // State(1853)
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1853u16,
    // State(1863)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    1863u16,
    // State(1993)
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    1993u16,
    // State(2221)
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2221u16,
    // State(2224)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    2224u16,
    // State(2225)
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2225u16,
    // State(2233)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2233u16,
    // State(2256)
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2256u16,
    // State(2263)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    2263u16,
    // State(2265)
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2265u16,
    // State(2277)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2277u16,
    // State(2477)
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2477u16,
    // State(2481)
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    2481u16,
    // State(2486)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2486u16,
    // State(2661)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2661u16,
    // State(2665)
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2665u16,
    // State(2704)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2704u16,
    // State(2708)
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2708u16,
    // State(2879)
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2879u16,
    // State(2882)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    2882u16,
    // State(2883)
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2883u16,
    // State(2891)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2891u16,
    // State(2952)
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    2952u16,
    // State(3018)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    3018u16,
    // State(3058)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    3058u16,
    // State(3136)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3136u16,
    // State(3198)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    3198u16,
    // State(3202)
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    3202u16,
    // State(3246)
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3246u16,
    // State(3267)
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3267u16,
    // State(3282)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3282u16,
    // State(3381)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3381u16,
    // State(3402)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3402u16,
    // State(3417)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3417u16,
    // State(3450)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    3450u16,
    // State(3487)
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3487u16,
    // State(3494)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3494u16,
    // State(3507)
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3507u16,
    // State(3524)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3524u16,
    // State(3585)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3585u16,
    // State(3592)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3592u16,
    // State(3605)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3605u16,
    // State(3622)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3622u16,
    // State(3681)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3681u16,
    // State(3685)
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3685u16,
    // State(3735)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3735u16,
    // State(3739)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3739u16,
    // State(3785)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3785u16,
    // State(3828)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3828u16,
};

pub const DO_WHITES: Set<u16> = phf_set! {
    // State(1411)
    //   [DoWhileStatement -> DO Statement WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    1411u16,
    // State(2654)
    //   [DoWhileStatement_Return -> DO Statement_Return WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    2654u16,
    // State(2690)
    //   [DoWhileStatement_Await_Return -> DO Statement_Await_Return WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    2690u16,
    // State(3188)
    //   [DoWhileStatement_Await -> DO Statement_Await WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    3188u16,
    // State(3669)
    //   [DoWhileStatement_Yield_Return -> DO Statement_Yield_Return WHILE LPAREN Expression_In_Yield RPAREN . SEMI_COLON]*
    3669u16,
    // State(3720)
    //   [DoWhileStatement_Yield_Await_Return -> DO Statement_Yield_Await_Return WHILE LPAREN Expression_In_Yield_Await RPAREN . SEMI_COLON]*
    3720u16,
};
