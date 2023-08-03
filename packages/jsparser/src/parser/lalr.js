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

for (const state of spec.states) {
  let permitRegularExpressionLiteral = false;
  let permitTemplateMiddle = false;
  let permitTemplateTail = false;

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

    action[0] = tokenIndexMap[action[0]];
    switch (action[1].type) {
    case 'Accept':
      action[1] = 'Action::Accept';
      break;
    case 'Shift':
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
    goto_[0] = nonTerminalIndexMap[goto_[0]];
    goto_[1] = `State(${goto_[1]})`;
  }

  state.kernel_items = state.kernel_items.join(', ');
}

console.log(JSON.stringify(spec));
