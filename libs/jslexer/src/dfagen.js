'use strict';

import {
  assert,
  assertEquals,
  assertExists,
  assertInstanceOf,
  fail,
  unimplemented,
} from 'https://deno.land/std@0.187.0/testing/asserts.ts';
import * as log from 'https://deno.land/std@0.187.0/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.187.0/yaml/mod.ts';
import { snakeCase } from 'https://deno.land/x/case@2.1.1/mod.ts'
import { UnicodeSpan, UnicodeSet, UnicodeSetsBuilder } from '../../../tools/lib/dfa/unicode.js';
import { buildAsciiTable, buildNonAsciiList } from '../../../tools/lib/dfa/compiler.js';
import { parseCommand, readAllText } from '../../../tools/lib/cli.js';
import { setup } from '../../../tools/lib/log.js';

const PROGNAME = 'dfagen.js';

setup(PROGNAME, 'DEBUG');

const DOC = `
Generate a DFA recognizing a language defined by an ECMA lexical grammar.

Usage:
  ${PROGNAME} [options] <tokens>...
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.

Arguments:
  <tokens>
    Tokens that the generated DFA recognizes.
`.trim();

async function run(options, tokens) {
  if (options.debug) {
    setup(PROGNAME, 'DEBUG');
  } else {
    setup(PROGNAME, 'INFO');
  }
  const grammar = yaml.parse(await readAllText(Deno.stdin));
  const dfa = compile(grammar, tokens);
  const unicodeSets = dfa.buildUnicodeSets();
  log.debug(`#Tokens=${tokens.length} #UnicodeSets=${unicodeSets.length} #DFA=${dfa.size}`);
  console.log(JSON.stringify({
    tokens: tokens.map((token) => {
      return { name: token };
    }),
    unicodeSets: unicodeSets.map((unicodeSet) => {
      return unicodeSet.spans.map((span) => {
        if (span.length === 1) {
          return span.firstCodePoint;
        }
        return [span.firstCodePoint, span.lastCodePoint];
      });
    }),
    states: dfa.states.map((state) => {
      const transitions = [];
      for (const unicodeSet of unicodeSets) {
        const trans = state.transitions.find((trans) => {
          return trans.unicodeSet.includes(unicodeSet);
        });
        if (trans) {
          transitions.push(trans.next);
        } else {
          transitions.push(dfa.size);
        }
      }
      return {
        transitions,
        accept: state.accept,
        lookahead: state.lookahead,
        dead: transitions.every((id) => id === state.id),
      };
    }),
    asciiTable: buildAsciiTable(unicodeSets),
    nonAsciiList: buildNonAsciiList(unicodeSets),
  }));
}

function compile(grammar, tokens) {
  log.debug('Building NFA from the lexical grammar in CFG...');
  const nfa = buildNfa(grammar, tokens);
  log.debug(`Total number of states in NFA: ${nfa.size}`);

  log.debug('Building DFA...');
  const dfa = nfa.buildDfa();
  log.debug(`Total number of states in DFA: ${dfa.size}`);

  log.debug('Minifying DFA...');
  const min = dfa.minify(tokens);
  log.debug(`Total number of states in DFA: ${min.size}`);

  return min;
}

function buildNfa(grammar, tokens) {
  const builder = new NfaBuilder(grammar);
  for (const token of tokens) {
    log.debug(`Compiling ${token} into NFA...`);
    builder.addToken(token);
  }
  return builder.build();
}

class NfaBuilder {
  constructor(grammar) {
    this.grammar_ = grammar;
    this.nfa_ = new Nfa();
    const start = this.nfa_.createState('@start');
    // We assume that the ID of the start state is 0.
    assertEquals(start, 0);
  }

  addToken(token) {
    assertExists(this.grammar_[token]);
    const start = this.nfa_.createState(`${token}@start`);
    const accept = this.nfa_.createState(`${token}@accept`);
    this.nfa_.accept(accept, token);
    log.debug(`${this.nfa_.$(accept).toString()}`);
    // Contexts constitutes a stack used for detecting a recursion in production
    // rules.
    const context = {
      prev: null,
      label: token,
      item: {
        name: token,
        endpoints: [start, accept],
      },
      indent: '',
    };
    const [subStart, subAccept] = this.buildNfa_(this.grammar_[token], context);
    this.nfa_.addTransition(start, subStart);
    this.nfa_.addTransition(subAccept, accept);
    this.nfa_.addTransition(0, start);
  }

  build() {
    const nfa = this.nfa_;
    this.nfa_ = null;
    return nfa;
  }

  buildNfa_(item, context) {
    switch (item.type) {
    case 'non-terminal':
      return this.buildNonTerminalNfa_(item.data, context);
    case 'one-of':
      return this.buildUnifiedNfa_(item.data, context);
    case 'sequence':
      return this.buildSequenceNfa_(item.data, context);
    case 'word':
      return this.buildWordNfa_(item.data, context);
    case 'unicode-set':
      return this.buildUnicodeSetNfa_(item.data, context);
    case 'empty':
      return this.buildEmptyNfa_(context);
    default:
      unimplemented(`TODO: ${item.type}`);
    }
  }

  findRecursion_(name, context) {
    if (context.item?.name === name) {
      return context.item.endpoints;
    }
    if (context.prev !== null) {
      return this.findRecursion_(name, context.prev);
    }
    return undefined;
  }

  buildNonTerminalNfa_(name, context) {
    const endpoints = this.findRecursion_(name, context);
    if (endpoints !== undefined) {
      log.debug(`${context.indent}${name}: Recursion detected`);
      context.recursion = true;
      return endpoints;
    }
    log.debug(`${context.indent}${name}`);
    const start = this.nfa_.createState(`${name}@start`);
    const accept = this.nfa_.createState(`${name}@accept`);
    const [subStart, subAccept] = this.buildNfa_(this.grammar_[name], {
      prev: context,
      label: name,
      item: { name, endpoints: [start, accept] },
      indent: context.indent + '  ',
    });
    this.nfa_.addTransition(start, subStart);
    this.nfa_.addTransition(subAccept, accept);
    return [start, accept];
  }

  buildUnifiedNfa_(items, context) {
    log.debug(`${context.indent}one-of`);
    const start = this.nfa_.createState(`${context.label}/one-of@start`);
    const accept = this.nfa_.createState(`${context.label}/one-of@accept`);
    for (let i = 0; i < items.length; ++i) {
      const item = items[i];
      const [subStart, subAccept] = this.buildNfa_(item, {
        prev: context,
        label: `${context.label}/one-of[${i}]`,
        indent: context.indent + '  ',
      });
      this.nfa_.addTransition(start, subStart);
      this.nfa_.addTransition(subAccept, accept);
    }
    return [start, accept];
  }

  buildSequenceNfa_(items, context) {
    log.debug(`${context.indent}sequence`);
    const lookaheadIndex = items.findIndex((item) => item.type === 'lookahead');
    const normalSeqEnd = lookaheadIndex >= 0 ? lookaheadIndex : items.length;
    const start = this.nfa_.createState(`${context.label}/seq@start`);
    let state = start;
    let ahead = 0;
    for (let i = 0; i < normalSeqEnd; ++i) {
      const item = items[i];
      const localContext = {
        prev: context,
        label: `${context.label}/seq[${i}]`,
        indent: context.indent + '  ',
      };
      const [subStart, subAccept] = this.buildNfa_(item, localContext);
      // We build a DFA recognizing tokens defined in regular expressions.
      // A production like `A -> aAb` is not allowed.
      // A rule including such productions cannot be represented in a regular
      // expression and a stack is needed for recognizing it.
      if (localContext.recursion) {
        assert(i == 0 || i == items.length - 1);
      }
      this.nfa_.addTransition(state, subStart);
      state = subAccept;
    }
    // Process lookahead items.
    if (lookaheadIndex >= 0) {
      const [subStart, subAccept] = this.buildLookaheadsNfa_(
        items.slice(lookaheadIndex), {
          prev: context,
          label: `${context.label}/seq[${lookaheadIndex}..]`,
          indent: context.indent + '  ',
        }
      );
      this.nfa_.addTransition(state, subStart);
      state = subAccept;
    }
    return [start, state];
  }

  buildWordNfa_(word, context) {
    log.debug(`${context.indent}${JSON.stringify(word)}`);
    const start = this.nfa_.createState(`${context.label}/word(${word})@start`);
    let state = start;
    for (const ch of word) {
      const next = this.nfa_.createState(`${context.label}/word(${word})[${ch}]`);
      const unicodeSet = new UnicodeSet(new UnicodeSpan(ch));
      this.nfa_.addTransition(state, next, unicodeSet);
      state = next;
    }
    return [start, state];
  }

  buildUnicodeSetNfa_(data, context) {
    const start = this.nfa_.createState(`${context.label}/cc@start`);
    const accept = this.nfa_.createState(`${context.label}/cc@accept`);
    const unicodeSet = this.buildUnicodeSet_(data);
    log.debug(`${context.indent}unicode-set: ${unicodeSet.toString()}`);
    this.nfa_.addTransition(start, accept, unicodeSet);
    return [start, accept];
  }

  buildEmptyNfa_(context) {
    const start = this.nfa_.createState(`${context.label}/empty@start`);
    const accept = this.nfa_.createState(`${context.label}/empty@accept`);
    log.debug(`${context.indent}empty`);
    this.nfa_.addTransition(start, accept);
    return [start, accept];
  }

  buildLookaheadsNfa_(lookaheads, context) {
    const start = this.nfa_.createState(`${context.label}/lookahead@start`);
    const accept = this.nfa_.createState(`${context.label}/lookahead@accept`);
    this.nfa_.lookahead(accept);
    let unicodeSet = UnicodeSet.ANY;
    for (const lookahead of lookaheads) {
      let other = this.buildUnicodeSet_(lookahead.data);
      if (lookahead.negate) {
        other = UnicodeSet.ANY.exclude(other);
      }
      unicodeSet = unicodeSet.intersect(other);
    }
    log.debug(`${context.indent}lookahead: ${unicodeSet.toString()}`);
    this.nfa_.addTransition(start, accept, unicodeSet);
    return [start, accept];
  }

  buildUnicodeSet_(data) {
    let unicodeSet = UnicodeSet.EMPTY;
    for (const ch of data) {
      switch (ch.type) {
      case 'any':
        unicodeSet = unicodeSet.merge(UnicodeSet.ANY);
        break;
      case 'non-terminal':
        unicodeSet = unicodeSet.merge(this.buildNonTerminalUnicodeSet_(ch.data));
        break;
      case 'built-in':
        unicodeSet = unicodeSet.merge(UnicodeSet[ch.data]);
        break;
      case 'char':
        unicodeSet = unicodeSet.mergeSpan(new UnicodeSpan(ch.data));
        break;
      case 'span':
        unicodeSet = unicodeSet.mergeSpan(new UnicodeSpan(ch.data[0], ch.data[1]));
        break;
      case 'exclude':
        if (ch.data.length === 1) {
          unicodeSet = unicodeSet.excludeSpan(new UnicodeSpan(ch.data));
        } else {
          unicodeSet = unicodeSet.exclude(this.buildNonTerminalUnicodeSet_(ch.data));
        }
        break;
      default:
        unimplemented(`NfaBuilder.buildUnicodeSet_: ${ch.type}`);
      }
    }
    return unicodeSet;
  }

  buildNonTerminalUnicodeSet_(name) {
    assertExists(this.grammar_[name]);
    const item = this.grammar_[name];
    switch (item.type) {
    case 'any':
      return UnicodeSet.ANY;
    case 'non-terminal':
      return this.buildNonTerminalUnicodeSet_(item.data);
    case 'one-of':
      return this.buildUnifiedUnicodeSet_(item.data);
    case 'unicode-set':
      return this.buildUnicodeSet_(item.data);
    default:
      unimplemented(`NfaBuilder.buildNonTerminalUnicodeSet_: ${item.type}`);
    }
  }

  buildUnifiedUnicodeSet_(items) {
    let unicodeSet = UnicodeSet.EMPTY;
    for (const item of items) {
      switch (item.type) {
      case 'non-terminal':
        unicodeSet = unicodeSet.merge(this.buildNonTerminalUnicodeSet_(item.data));
        break;
      case 'unicode-set':
        unicodeSet = unicodeSet.merge(this.buildUnicodeSet_(item.data));
        break;
      default:
        unimplemented(`NfaBuilder.buildUnifiedUnicodeSet_: ${item.type}`);
      }
    }
    return unicodeSet;
  }
}

class Automaton {
  constructor() {
    this.states = [];
  }

  get size() {
    return this.states.length;
  }

  $(id) {
    return this.states[id];
  }

  createState(label) {
    const id = this.states.length;
    const state = new State(id, label);
    this.states.push(state);
    return id;
  }

  accept(id, token) {
    this.$(id).accept = token;
  }

  lookahead(id) {
    this.$(id).lookahead = true;
  }

  addTransition(id, next, unicodeSet = UnicodeSet.EMPTY) {
    this.$(id).transitions.push({ unicodeSet, next });
  }

  buildUnicodeSets() {
    const builder = new UnicodeSetsBuilder();
    for (const state of this.states) {
      for (const trans of state.transitions) {
        builder.add(trans.unicodeSet);
      }
    }
    return builder.build();
  }

  minifyTransitionTables() {
    // Merge unicode sets in the transition table of each state, which move
    // to the same state.
    for (const state of this.states) {
      log.debug(`${state.toString()}`);
      const map = new Map();  // state -> [unicodeSet]
      for (const trans of state.transitions) {
        if (map.has(trans.next)) {
          map.get(trans.next).push(trans.unicodeSet);
        } else {
          map.set(trans.next, [trans.unicodeSet]);
        }
      }
      state.transitions = [];
      for (const next of Array.from(map.keys()).sort((a, b) => a - b)) {
        let unicodeSet = UnicodeSet.EMPTY;
        for (const other of map.get(next)) {
          unicodeSet = unicodeSet.merge(other);
        }
        log.debug(`  ${unicodeSet.toString()} -> ${this.$(next).toString()}`);
        state.transitions.push({ unicodeSet, next });
      }
    }
    return this;
  }

  // protected methods

  determineToken_(ids) {
    // We assume that tokens specified in the command line have been sorted in
    // order of priority.  A higher priority token has a smaller identifier and
    // the `ids` have been sorted in ascending order.  So, we return the
    // identifier of the first accept state.
    return ids
      .map((id) => this.$(id).accept)
      .filter((accept) => accept)[0] ?? null;
  }

  determineLookahead_(ids) {
    return ids.some((id) => this.$(id).lookahead);
  }

  validateGroup_(ids) {
    if (ids.some((id) => this.$(id).lookahead)) {
      // If a group has a state generated by a lookahead item, every state in
      // the group must be a state generated by a lookahead item.
      if (!ids.every((id) => this.$(id).lookahead)) {
        log.error('Ambiguous lexical grammar:');
        for (const id of ids) {
          if (!this.$(id).lookahead) {
            log.error(`${this.$(id).toString()}`);
          }
        }
        fail();
      }
    }
  }
}

class Nfa extends Automaton {
  constructor() {
    super();
  }

  buildDfa() {
    const unicodeSets = this.buildUnicodeSets();
    unicodeSets.forEach((unicodeSet, i) => {
      log.debug(`UnicodeSet#${i}: ${unicodeSet.toString()}`);
    });

    const dfa = new Dfa();
    dfa.createState('@start');

    const nfaStartStateIds = this.closure_([0]);

    const stateMap = new Map();
    stateMap.set(nfaStartStateIds.join(','), 0);

    const remaining = [nfaStartStateIds];
    while (remaining.length > 0) {
      const nfaStateIds = remaining.shift();
      const dfaStateId = stateMap.get(nfaStateIds.join(','));
      log.debug(`${dfa.$(dfaStateId).toString()}: NFA#[${nfaStateIds.join(',')}]`);
      for (const unicodeSet of unicodeSets) {
        const nextNfaStateIds = this.closure_(this.move_(nfaStateIds, unicodeSet));
        if (nextNfaStateIds.length === 0) {
          log.debug(`  ${unicodeSet.toString()} -> ()`);
          continue;
        }
        let dfaNextStateId = stateMap.get(nextNfaStateIds.join(','));
        if (dfaNextStateId === undefined) {
          dfaNextStateId = dfa.createState('');
          const token = this.determineToken_(nextNfaStateIds);
          if (token) {
            dfa.accept(dfaNextStateId, token);
          }
          // It's ensured that every state in the group before computing the
          // closure was generated by a lookahead item in the lexical grammar.
          const lookahead = this.determineLookahead_(nextNfaStateIds);
          if (lookahead) {
            dfa.lookahead(dfaNextStateId);
          }
          stateMap.set(nextNfaStateIds.join(','), dfaNextStateId);
          remaining.push(nextNfaStateIds);
        }
        log.debug(`  ${unicodeSet.toString()} -> ${dfa.$(dfaNextStateId).toString()}`);
        dfa.addTransition(dfaStateId, dfaNextStateId, unicodeSet);
      }
    }

    return dfa;
  }

  // private methods

  move_(ids, unicodeSet) {
    const result = [];
    for (const id of ids) {
      for (const trans of this.$(id).transitions) {
        if (trans.unicodeSet.includes(unicodeSet)) {
          result.push(trans.next);
        }
      }
    }
    this.validateGroup_(result);
    return result;
  }

  closure_(ids) {
    const closure = new Set(ids);
    const remaining = Array.from(ids);
    while (remaining.length > 0) {
      const id = remaining.pop();
      const nextIds = this.states[id]
        .transitions
        .filter((trans) => trans.unicodeSet.isEmpty)
        .filter((trans) => !closure.has(trans.next))
        .map((trans) => trans.next);
      for (const nextId of nextIds) {
        closure.add(nextId);
        remaining.push(nextId);
      }
    }
    return Array.from(closure.values()).sort((a, b) => a - b);
  }
}

class Dfa extends Automaton {
  constructor() {
    super();
  }

  minify(tokens) {
    const unicodeSets = this.buildUnicodeSets();

    let groups = [];
    // Separate lookahead states from others.
    this.collectGroup_(null, false, groups);
    this.collectGroup_(null, true, groups);
    // Create a separate group for each token.
    for (const token of tokens) {
      this.collectGroup_(token, false, groups);
      this.collectGroup_(token, true, groups);
    }
    assert(groups.length > 0);

    for (let i = 0; true; ++i) {
      log.debug(`round#${i}: ${JSON.stringify(groups)}`);
      let newGroups = [];
      for (const group of groups) {
        this.validateGroup_(group);
        // Collect states having the same transition table in `groups`.
        const map = new Map();
        for (const id of group) {
          const trans = this.buildTransitionTable_(id, unicodeSets, groups);
          const key = trans.join(',');
          if (map.has(key)) {
            map.get(key).push(id);
          } else {
            map.set(key, [id]);
          }
        }
        for (const newGroup of map.values()) {
          newGroups.push(newGroup);
        }
      }
      if (groups.length === newGroups.length) {
        groups = newGroups;
        break;
      }
      groups = newGroups;
    }

    const newStates = [];
    const dfa = new Dfa();
    // Reconstruct states.
    for (let i = 0; i < groups.length; ++i) {
      const group = groups[i];
      assert(group.length > 0);
      const id = dfa.createState('');
      const token = this.determineToken_(group);
      if (token) {
        dfa.accept(id, token);
      }
      const lookahead = this.determineLookahead_(group);
      if (lookahead) {
        dfa.lookahead(id);
      }
      assertEquals(i, id);
    }
    // Reconstruct the transitions of each state.
    for (let id = 0; id < groups.length; ++id) {
      const group = groups[id];
      assert(group.length > 0);
      log.debug(`${dfa.$(id).toString()}: DFA#[${group.join(',')}]`);
      // Every state in `group` has the same transitions.  So, we can use
      // `group[0]` for rebuilding the transitions of the new state.
      for (const trans of this.$(group[0]).transitions) {
        const next = groups.findIndex((group) => group.includes(trans.next));
        log.debug(`  ${trans.unicodeSet.toString()} -> ${dfa.$(next).toString()}`);
        dfa.addTransition(id, next, trans.unicodeSet);
      }
    }

    return dfa.minifyTransitionTables();
  }

  // A group contains state IDs.
  collectGroup_(token, lookahead, groups) {
    const group = this.states
      .filter((state) => {
        return state.accept === token && state.lookahead === lookahead;
      })
      .map((state) => state.id);
    if (group.length > 0) {
      groups.push(group);
    }
  }

  buildTransitionTable_(id, unicodeSets, groups) {
    const transTable = [];
    for (const unicodeSet of unicodeSets) {
      const trans = this.$(id).transitions.find((trans) => {
        return trans.unicodeSet.includes(unicodeSet);
      });
      if (trans) {
        const next = groups.findIndex((group) => group.includes(trans.next));
        transTable.push(next);
      } else {
        transTable.push(null);
      }
    }
    return transTable;
  }
}

class State {
  constructor(id, label) {
    this.id = id;
    this.label = label;
    this.transitions = [];
    this.accept = null;
    this.lookahead = false;
  }

  toString() {
    let s = `State#${this.id}`;
    if (this.accept) {
      s += `@${this.accept}`;
    }
    if (this.lookahead) {
      s += '>';
    }
    if (this.label) {
      s += `[${this.label}]`;
    }
    return s;
  }
}

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(options, args.tokens));
