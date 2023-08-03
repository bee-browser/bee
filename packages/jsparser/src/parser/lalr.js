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
  for (const action of state.actions) {
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
  for (const goto_ of state.gotos) {
    goto_[0] = nonTerminalIndexMap[goto_[0]];
    goto_[1] = `State(${goto_[1]})`;
  }
  state.kernel_items = state.kernel_items.join(', ');
}

console.log(JSON.stringify(spec));
