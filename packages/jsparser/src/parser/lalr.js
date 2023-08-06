'use strict';

import { assert } from 'https://deno.land/std@0.193.0/testing/asserts.ts';
import { readAllText } from '../../../../tools/lib/cli.js';

const spec = JSON.parse(await readAllText(Deno.stdin));
const tokens = JSON.parse(await Deno.readTextFile("../lexer/tokens.json"));

const tokenIndexMap = {};
for (let i = 0; i < tokens.length; ++i) {
  tokenIndexMap[tokens[i]] = i;
}
tokenIndexMap['$'] = tokens.length;

const nonTerminalIndexMap = {};
for (let i = 0; i < spec.non_terminals.length; ++i) {
  nonTerminalIndexMap[spec.non_terminals[i]] = i;
}

function isAutoSemicolonDisallowed(state) {
  for (const item of state.kernel_items) {
    // 12.9.1 Rules of Automatic Semicolon Insertion
    //
    //   However, there is an additional overriding condition on the preceding rules: a semicolon
    //   is never inserted automatically if the semicolon would then be parsed as an empty
    //   statement or if that semicolon would become one of the two semicolons in the header of a
    //   for statement (see 14.7.4).
    if (item === '[EmptyStatement -> SEMI_COLON .]*') {
      return true;
    }
    if (item.startsWith('[ForStatement')) {
      if (item.includes(' SEMI_COLON . ')) {
        return true;
      }
    }
  }
  return false;
}

function isAutoSemicolonDoWhile(state) {
  for (const item of state.kernel_items) {
    // ';' at the end of a do-white statement.
    if (item.startsWith('[DoWhileStatement')) {
      if (item.endsWith(' RPAREN . SEMI_COLON]*')) {
        return true;
      }
    }
  }
  return false;
}

for (const state of spec.states) {
  let permitRegularExpressionLiteral = false;
  let permitTemplateMiddle = false;
  let permitTemplateTail = false;

  state.isAutoSemicolonDoWhile = isAutoSemicolonDoWhile(state);

  for (const action of state.actions) {
    switch (action[0]) {
    case 'RegularExpressionLiteral':
      permitRegularExpressionLiteral = true;
      break;
    case 'TemplateMiddle':
      permitTemplateMiddle = true;
      break;
    case 'TemplateTail':
      permitTemplateTail = true;
      break;
    }

    action[0] = {
      index: tokenIndexMap[action[0]],
      label: action[0],
    };
    switch (action[1].type) {
    case 'Accept':
      action[1] = 'Action::Accept';
      break;
    case 'Shift':
      if (action[0].label === 'SEMI_COLON') {
        const nextState = spec.states[action[1].data];
        nextState.isAutoSemicolonDisallowed = isAutoSemicolonDisallowed(nextState);
      }
      action[1] = `Action::Shift(State(${action[1].data}))`;
      break;
    case 'Reduce':
      action[1] = `Action::Reduce(NonTerminal::${action[1].data[0]}, ${action[1].data[1]}, "${action[1].data[2]}")`;
      break;
    }
  }

  // The lexical goal symbol.
  //
  // InputElementRegExpOrTemplateTail
  //   In syntactic grammar contexts where a RegularExpressionLiteral, a TemplateMiddle, or a
  //   TemplateTail is permitted.
  //
  // InputElementRegExp
  //   In syntactic grammar contexts where a RegularExpressionLiteral is permitted but
  //   neither a TemplateMiddle, nor a TemplateTail is permitted.
  //
  // InputElementTemplateTail
  //   In syntactic grammar contexts where a TemplateMiddle or a TemplateTail is permitted
  //   but a RegularExpressionLiteral is not permitted.
  //
  // InputElementDiv
  //   In all other contexts.
  //
  // See the section #sec-ecmascript-language-lexical-grammar in the ECMA-262 specification.
  if (permitRegularExpressionLiteral && permitTemplateMiddle && permitTemplateTail) {
    state.lexical_goal = 'InputElementRegExpOrTemplateTail';
  } else if (permitRegularExpressionLiteral && !permitTemplateMiddle && !permitTemplateTail) {
    state.lexical_goal = 'InputElementRegExp';
  } else if (!permitRegularExpressionLiteral && permitTemplateMiddle && permitTemplateTail) {
    state.lexical_goal = 'InputElementTemplateTail';
  } else {
    assert(!permitRegularExpressionLiteral);
    assert(!permitTemplateMiddle);
    assert(!permitTemplateTail);
    state.lexical_goal = 'InputElementDiv';
  }

  for (const goto_ of state.gotos) {
    goto_[0] = {
      index: nonTerminalIndexMap[goto_[0]],
      label: goto_[0],
    };
    goto_[1] = {
      index: goto_[1],
      label: spec.states[goto_[1]].kernel_items.join(', '),
    };
  }

  state.label = state.kernel_items.join(', ');
}

for (const state of spec.states) {
  if (state.isAutoSemicolonDisallowed) {
    assert(!state.isAutoSemicolonDoWhile);
  }
  if (state.isAutoSemicolonDoWhile) {
    assert(!state.isAutoSemicolonDisallowed);
  }
}

console.log(JSON.stringify(spec));
