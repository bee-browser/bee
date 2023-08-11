'use strict';

import { assert } from 'https://deno.land/std@0.197.0/testing/asserts.ts';
import { readAllText } from '../../../../tools/lib/cli.js';

const spec = JSON.parse(await readAllText(Deno.stdin));

const states = [];
for (const state of spec.dfa.states) {
  const transitions = spec.unicodeSets.map((us) => {
    const trans = state.transitions.find((trans) => {
      // trans.unicode_set.contains(us)
      return us.every((span1) => {
        return trans.unicode_set.some((span2) => {
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
  states.push({
    transitions,
    accept: state.accept,
    lookahead: state.lookahead,
    dead: transitions.every((i) => i == state.id),
  });
}

const asciiTable = [];
for (let ascii = 0; ascii < 0x80; ++ascii) {
  const i = spec.unicodeSets.findIndex((us) => {
    return us.some((span) => ascii >= span.base && ascii < span.base + span.length);
  });
  if (i === -1) {
    asciiTable[ascii] = spec.unicodeSets.length;
  } else {
    asciiTable[ascii] = i;
  }
}

const nonAsciiList = [];
for (let i = 0; i < spec.unicodeSets.length; ++i) {
  for (const span of spec.unicodeSets[i]) {
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

spec.states = states;
spec.asciiTable = asciiTable;
spec.nonAsciiList = nonAsciiList;

console.log(JSON.stringify(spec));
