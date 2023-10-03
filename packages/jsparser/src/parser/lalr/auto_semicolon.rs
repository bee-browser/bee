// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// codegen.js --input-stdin --no-escape lalr/auto_semicolon.rs.hbs

use phf::phf_set;
use phf::Set;

pub const DISALLOWED: Set<u16> = phf_set! {
    // State(43)
    //   [EmptyStatement -> SEMI_COLON .]*
    43u16,
    // State(679)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    679u16,
    // State(906)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    906u16,
    // State(1105)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1105u16,
    // State(1123)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    1123u16,
    // State(1139)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement]*
    1139u16,
    // State(1330)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1330u16,
    // State(1348)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    1348u16,
    // State(1364)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1364u16,
    // State(1492)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1492u16,
    // State(1499)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    1499u16,
    // State(1519)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1519u16,
    // State(1536)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1536u16,
    // State(1651)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1651u16,
    // State(1658)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    1658u16,
    // State(1670)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1670u16,
    // State(1687)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1687u16,
    // State(1810)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1810u16,
    // State(1818)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1818u16,
    // State(1849)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1849u16,
    // State(1871)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1871u16,
    // State(1923)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1923u16,
    // State(1927)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1927u16,
    // State(2095)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    2095u16,
    // State(2123)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2123u16,
    // State(2127)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    2127u16,
    // State(2133)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2133u16,
    // State(2147)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2147u16,
    // State(2151)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    2151u16,
    // State(2157)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2157u16,
    // State(2194)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2194u16,
    // State(2519)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2519u16,
    // State(2522)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    2522u16,
    // State(2523)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2523u16,
    // State(2531)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2531u16,
    // State(2544)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2544u16,
    // State(2547)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    2547u16,
    // State(2548)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2548u16,
    // State(2556)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2556u16,
    // State(2948)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2948u16,
    // State(2952)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2952u16,
    // State(2979)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2979u16,
    // State(2983)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2983u16,
    // State(3225)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3225u16,
    // State(3291)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    3291u16,
    // State(3323)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    3323u16,
    // State(3412)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3412u16,
    // State(3491)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3491u16,
    // State(3513)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3513u16,
    // State(3529)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3529u16,
    // State(3623)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3623u16,
    // State(3645)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3645u16,
    // State(3661)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3661u16,
    // State(3699)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3699u16,
    // State(3706)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3706u16,
    // State(3720)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3720u16,
    // State(3737)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3737u16,
    // State(3792)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3792u16,
    // State(3799)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3799u16,
    // State(3813)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3813u16,
    // State(3830)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3830u16,
    // State(3870)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3870u16,
    // State(3874)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3874u16,
    // State(3922)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3922u16,
    // State(3926)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3926u16,
    // State(3965)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3965u16,
    // State(4008)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    4008u16,
};

pub const DO_WHITES: Set<u16> = phf_set! {
    // State(1798)
    //   [DoWhileStatement -> DO Statement WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    1798u16,
    // State(1900)
    //   [DoWhileStatement_Await -> DO Statement_Await WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    1900u16,
    // State(2941)
    //   [DoWhileStatement_Return -> DO Statement_Return WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    2941u16,
    // State(2969)
    //   [DoWhileStatement_Await_Return -> DO Statement_Await_Return WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    2969u16,
    // State(3858)
    //   [DoWhileStatement_Yield_Return -> DO Statement_Yield_Return WHILE LPAREN Expression_In_Yield RPAREN . SEMI_COLON]*
    3858u16,
    // State(3907)
    //   [DoWhileStatement_Yield_Await_Return -> DO Statement_Yield_Await_Return WHILE LPAREN Expression_In_Yield_Await RPAREN . SEMI_COLON]*
    3907u16,
};
