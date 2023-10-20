'use strict';

import { assert } from 'https://deno.land/std@0.204.0/testing/asserts.ts';
import { readAllText } from '../../../../tools/lib/cli.js';

const spec = JSON.parse(await readAllText(Deno.stdin));

const states = [];
for (const state of spec.dfa.states) {
  const transitions = spec.unicodeSets.map((us) => {
    const trans = state.transitions.find((trans) => {
      // trans.unicode_set.contains(us)
      return us.spans.every((span1) => {
        return trans.unicode_set.spans.some((span2) => {
          return span2.base <= span1.base &&
            span1.base + span1.length <= span2.base + span2.length;
        });
      });
    });
    if (trans === undefined) {
      return spec.dfa.states.length;
    }
    return trans.next_id;
  });
  // EOF
  const trans = state.transitions.find((trans) => trans.unicode_set.eof);
  if (trans === undefined) {
    transitions.push(spec.dfa.states.length);
  } else {
    transitions.push(trans.next_id);
  }
  states.push({
    transitions,
    accept: state.accept,
    lookahead: state.lookahead,
    dead: transitions.every((i) => i == state.id),
    labels: state.labels,
    transitionLabels: state.transitions.map((trans) => trans.label),
  });
}

const asciiTable = [];
for (let ascii = 0; ascii < 0x80; ++ascii) {
  const i = spec.unicodeSets.findIndex((us) => {
    return us.spans.some((span) => ascii >= span.base && ascii < span.base + span.length);
  });
  if (i === -1) {
    asciiTable[ascii] = spec.unicodeSets.length;  // EOF
  } else {
    asciiTable[ascii] = i;
  }
}

const nonAsciiList = [];
for (let i = 0; i < spec.unicodeSets.length; ++i) {
  for (const span of spec.unicodeSets[i].spans) {
    let nonAscii;
    if (span.length === 0 || span.base > 0x7F) {
      nonAscii = span;
    } else {
      nonAscii = {
        base: 0x80,
        length: span.length - (0x80 - span.base),
      };
    }
    if (nonAscii.length <= 0) {
      continue;
    }
    if (nonAscii.length === 1) {
      nonAsciiList.push({
        span: false,
        firstCodePoint: nonAscii.base,
        lastCodePoint: nonAscii.base,
        unicodeSet: i,
      });
    } else {
      nonAsciiList.push({
        span: true,
        firstCodePoint: nonAscii.base,
        lastCodePoint: nonAscii.base + nonAscii.length - 1,
        unicodeSet: i,
      });
    }
  }
}

// A special hack for supporting ID_Start and ID_Continue.
for (let state of states) {
  state.checkIdStart =
    state.labels.some((label) => label.startsWith('UnicodeIDStart -> . '));
  state.checkIdContinue =
    state.labels.some((label) => label.startsWith('UnicodeIDContinue -> . '));
  assert(!(state.checkIdStart && state.checkIdContinue));
}
states[0].checkIdStart = true;
assert(!states[0].checkIdContinue);

spec.states = states;
spec.numStates = states.length;
spec.numTransitions = spec.unicodeSets.length + 1;
spec.asciiTable = asciiTable;
spec.nonAsciiList = nonAsciiList;

console.log(JSON.stringify(spec));
