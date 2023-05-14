'use strict';

import * as log from 'https://deno.land/std@0.186.0/log/mod.ts';
import { assert, unreachable } from "https://deno.land/std@0.186.0/testing/asserts.ts";
import { Pattern, oneOf } from './pattern.js';
import { UnicodeSpan, UnicodeSet, UnicodeSetsBuilder } from './unicode.js';

export function compile(tokens) {
  log.info('Unifying regular expressions...');
  const unified = unify(tokens);
  unified.dump(log.debug);

  log.info('Building unicode sets...');
  const unicodeSets = buildUnicodeSets(unified);
  log.info(`#UnicodeSets: ${unicodeSets.length}`);
  unicodeSets.forEach((unicodeSet, i) => {
    log.debug(`UnicodeSet#${i}: ${unicodeSet.toString()}`);
  });

  log.info('Building ASCII table from unicode sets...');
  const asciiTable = buildAsciiTable(unicodeSets);

  log.info('Building non-ASCII list from unicode sets...');
  const nonAsciiList = buildNonAsciiList(unicodeSets);

  log.info('Building pattern tree from unified regular expression...');
  const [nodes, root, accepts] = buildTree(unified);
  log.info(`#Nodes: ${nodes.length}`);
  //log.debug(nodes);
  //log.debug(root);
  //log.debug(accepts);

  log.info('Building DFA...');
  const states = buildStates(unicodeSets, nodes, root, accepts);
  log.info(`#States: ${states.length}`);

  log.info('Minimizing states in DFA...');
  const finalStates = minimize(states, accepts.values());
  log.info(`#States: ${finalStates.length}`);

  return {
    tokens: tokens.map(({ name, regexp }) => {
      return { name, regexp };
    }),
    unicodeSets: unicodeSets.map((unicodeSet) => {
      return unicodeSet.spans.map((span) => {
        if (span.length === 1) {
          return span.firstCodePoint;
        }
        return [span.firstCodePoint, span.lastCodePoint];
      });
    }),
    states: finalStates.map((state) => {
      return {
        transitions: state.transitions,
        accept: state.accept,
        dead: state.dead,
      };
    }),
    asciiTable,
    nonAsciiList,
  };
}

// private

function unify(tokens) {
  return oneOf(...tokens.map(({ name, regexp, expr }, index) => {
    return expr.accept({ name, regexp, index });
  }));
}

function buildUnicodeSets(unified) {
  let builder = new UnicodeSetsBuilder();
  let unicodeSet = UnicodeSet.EMPTY;
  unified.traverseInInorder((node, depth) => {
    switch (node.type) {
    case Pattern.CHAR:
      unicodeSet = unicodeSet.merge(node.unicodeSet);
      break;
    case Pattern.CONCATINATION:
    case Pattern.REPETITION:
      builder.add(unicodeSet);
      unicodeSet = UnicodeSet.EMPTY;
      break;
    default:
      // Nothing to do.
      break;
    }
  });
  return builder.build();
}

export function buildAsciiTable(unicodeSets) {
  const not_found = unicodeSets.length;
  const table = [];
  for (let i = 0; i < 128; ++i) {
    const index = unicodeSets.findIndex((unicodeSet) => {
      return unicodeSet.includesSpan(new UnicodeSpan(i));
    });
    if (index === -1) {
      table[i] = not_found;
    } else {
      table[i] = index;
    }
  }
  assert(table.length === 128);
  return table;
}

export function buildNonAsciiList(unicodeSets) {
  const ascii = new UnicodeSpan(0, 127);
  const list = [];
  for (let i = 0; i < unicodeSets.length; ++i) {
    const unicodeSet = unicodeSets[i];
    const nonAscii = unicodeSet.excludeSpan(ascii);
    if (nonAscii.isEmpty) {
      continue;
    }
    for (const span of nonAscii.spans) {
      if (span.length === 1) {
        list.push({
          codePoint: span.firstCodePoint,
          unicodeSet: i,
        })
      } else {
        list.push({
          firstCodePoint: span.firstCodePoint,
          lastCodePoint: span.lastCodePoint,
          unicodeSet: i,
        });
      }
    }
  }
  return list;
}

function buildTree(unified) {
  const nodes = [];
  const accepts = new Map();  // nodeIndex -> token
  const stack = [];
  unified.traverseInPostorder((pat) => {
    const index = nodes.length;
    switch (pat.type) {
    case Pattern.EMPTY:
      assert(pat.isLeaf);
      nodes.push({
        index,
        type: Pattern.EMPTY,
        nullable: true,
        firstNodes: [],
        lastNodes: [],
      });
      break;
    case Pattern.ACCEPT:
      assert(pat.isLeaf);
      nodes.push({
        index,
        type: Pattern.ACCEPT,
        token: pat.token,
        nullable: false,
        firstNodes: [index],
        lastNodes: [index],
        followNodes: new Set(),
      });
      accepts.set(index, pat.token);
      break;
    case Pattern.CHAR:
      assert(pat.isLeaf);
      nodes.push({
        index,
        type: Pattern.CHAR,
        unicodeSet: pat.unicodeSet,
        nullable: false,
        firstNodes: [index],
        lastNodes: [index],
        followNodes: new Set(),
      });
      break;
    case Pattern.CONCATINATION:
      {
        assert(pat.left !== null);
        assert(pat.right !== null);
        const right = stack.pop();
        const left = stack.pop();
        const node = { index };
        node.type = Pattern.CONCATINATION;
        node.left = left;
        node.right = right;
        node.nullable = nodes[left].nullable && nodes[right].nullable;
        if (nodes[left].nullable) {
          node.firstNodes = nodes[left].firstNodes.concat(nodes[right].firstNodes);
        } else {
          node.firstNodes = nodes[left].firstNodes;
        }
        if (nodes[right].nullable) {
          node.lastNodes = nodes[left].lastNodes.concat(nodes[right].lastNodes);
        } else {
          node.lastNodes = nodes[right].lastNodes;
        }
        for (const last of nodes[left].lastNodes) {
          for (const follow of nodes[right].firstNodes) {
            nodes[last].followNodes.add(follow);
          }
        }
        nodes.push(node);
      }
      break;
    case Pattern.ALTERNATION:
      {
        assert(pat.left !== null);
        assert(pat.right !== null);
        const right = stack.pop();
        const left = stack.pop();
        const node = { index };
        node.type = Pattern.ALTERNATION;
        node.left = left;
        node.right = right;
        node.nullable = nodes[left].nullable || nodes[right].nullable;
        node.firstNodes = nodes[left].firstNodes.concat(nodes[right].firstNodes);
        node.lastNodes = nodes[left].lastNodes.concat(nodes[right].lastNodes);
        nodes.push(node);
      }
      break;
    case Pattern.REPETITION:
      {
        assert(pat.left !== null);
        assert(pat.right === null);
        const left = stack.pop();
        const node = { index };
        node.type = Pattern.REPETITION;
        node.left = left;
        node.nullable = true;
        node.firstNodes = nodes[left].firstNodes;
        node.lastNodes = nodes[left].lastNodes;
        for (const last of node.lastNodes) {
          for (const follow of node.firstNodes) {
            nodes[last].followNodes.add(follow);
          }
        }
        nodes.push(node);
      }
      break;
    }
    stack.push(index);
  });
  assert(stack.length === 1);
  const root = stack.pop();
  assert(nodes.length === root + 1);  // The last node is the root of the tree.
  return [nodes, root, accepts];
}

function buildStates(unicodeSets, nodes, root, accepts) {
  const states = [{
    nodes: nodes[root].firstNodes,
    transitions: [],
    accept: null,
  }];
  const remaining = [0];
  const processed = new Set();
  while (remaining.length > 0) {
    const stateIndex = remaining.shift();
    log.debug(`Building transition table for state#${stateIndex}...`);
    processed.add(stateIndex);
    const state = states[stateIndex];
    state.accept = determineToken(state.nodes, accepts);
    for (let usIndex = 0; usIndex < unicodeSets.length; ++usIndex) {
      const unicodeSet = unicodeSets[usIndex];
      const nextState = {
        nodes: new Set(),
        transitions: [],
      };
      for (const nodeIndex of state.nodes) {
        const node = nodes[nodeIndex];
        if (node.type === Pattern.ACCEPT) {
          continue;
        }
        assert(node.type === Pattern.CHAR);
        if (node.unicodeSet.includes(unicodeSet)) {
          for (const followIndex of node.followNodes) {
            nextState.nodes.add(followIndex);
          }
        }
      }
      if (nextState.nodes.length === 0) {
        log.debug(`  UnicodeSet#{usIndex} -> No State`);
        state.transitions[usIndex] = null;
        continue;
      }
      nextState.nodes = Array.from(nextState.nodes).sort((a, b) => a - b);
      let nextStateIndex = states.findIndex((state) => {
        if (state.nodes.length !== nextState.nodes.length) {
          return false;
        }
        return state.nodes.every((a, i) => a === nextState.nodes[i]);
      });
      if (nextStateIndex === -1) {
        nextStateIndex = states.length;
        states.push(nextState);
      }
      log.debug(`  UnicodeSet#${usIndex} -> State#${nextStateIndex}`);
      state.transitions[usIndex] = nextStateIndex;
      if (processed.has(nextStateIndex)) {
        continue;
      }
      if (remaining.includes(nextStateIndex)) {
        continue;
      }
      remaining.push(nextStateIndex);
    }
  }
  return states;
}

function determineToken(nodes, accepts) {
  let accept = null;
  for (const nodeIndex of nodes) {
    if (accepts.has(nodeIndex)) {
      const token = accepts.get(nodeIndex);
      if (accept === null) {
        accept = token;
      } else if (accept.index > token.index) {
        accept = token;
      }
    }
  }
  return accept;
}

function minimize(states, tokens) {
  // Each group contains the index of each state.
  let groups = [
    states.map((_, i) => i).filter((si) => states[si].accept === null),
  ];
  for (const token of tokens) {
    // Collect states accepting the same token.
    groups.push(states.map((_, i) => i).filter((si) => {
      return states[si].accept === token;
    }));
  }

  for (let i = 0; true; ++i) {
    log.debug(`round#${i}: ${JSON.stringify(groups)}`);
    let newGroups = [];
    for (const group of groups) {
      // Collect states having the same transition table in `groups`.
      const map = new Map();
      for (const si of group) {
        const trans = rebuildTransitions(states[si], groups);
        const key = trans.join(',');
        if (map.has(key)) {
          map.get(key).push(si);
        } else {
          map.set(key, [si]);
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

  // Reconstruct states and transition tables.
  const newStates = [];
  for (const group of groups) {
    assert(group.length > 0);
    const si = newStates.length;
    const nodes = group.reduce((nodes, si) => {
      return nodes.concat(states[si].nodes);
    }, []);
    const transitions = rebuildTransitions(states[group[0]], groups);
    const accept = states[group[0]].accept;
    assert(group.every((si) => states[si].accept === accept));
    const dead = accept === null && transitions.every((nsi) => nsi === si);
    newStates.push({ nodes, transitions, accept, dead });
  }

  return newStates;
}

function rebuildTransitions(state, groups) {
  return state.transitions.map((si) => {
    return groups.findIndex((group) => group.includes(si));
  });
}
