'use strict';

import { assert } from 'https://deno.land/std@0.208.0/testing/asserts.ts';
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
  let ignoreLineTerminatorSequence = true;

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
    case 'LineTerminatorSequence':
      ignoreLineTerminatorSequence = false;
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
      {
        const nextId = action[1].data.next_id;
        if (action[0].label === 'SEMI_COLON') {
          const nextState = spec.states[nextId];
          nextState.isAutoSemicolonDisallowed = isAutoSemicolonDisallowed(nextState);
        }
        action[1] = `Action::Shift(State(${nextId}))`;
      }
      break;
    case 'Reduce':
      {
        const nonTerminal = action[1].data.non_terminal;
        const numPops = action[1].data.num_pops;
        const rule = action[1].data.rule;
        action[1] = `Action::Reduce(NonTerminal::${nonTerminal}, ${numPops}, "${rule}")`;
      }
      break;
    case 'Replace':
      {
        const nextId = action[1].data.next_id;
        action[1] = `Action::Replace(State(${nextId}))`;
      }
      break;
    }
  }

  state.actions.push([
    { index: tokenIndexMap['WhiteSpaceSequence'], label: 'WhiteSpaceSequence' },
    'Action::Ignore',
  ]);
  state.actions.push([
    { index: tokenIndexMap['Comment'], label: 'Comment' },
    'Action::Ignore',
  ]);
  if (ignoreLineTerminatorSequence) {
    state.actions.push([
      { index: tokenIndexMap['LineTerminatorSequence'], label: 'LineTerminatorSequence' },
      'Action::Ignore',
    ]);
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
