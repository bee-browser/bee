'use strict';

import { equal } from "https://deno.land/std@0.209.0/testing/asserts.ts";

import { Acorn, ESTree } from './test262_helper.js';

// Spawn estree in the server mode in order to reduce overhead of process creations.
let server = new ESTree();  // TODO: options
server.start();

self.onmessage = async ({ data }) => {
  const url = data;

  self.postMessage({ type: 'progress', message: 'loading...' });
  const res = await fetch(url);
  if (res.status !== 200) {
    self.postMessage({ type: 'skip', reason: 'broken link' });
    return;
  }

  const source = await res.text();

  let sourceType, expected;
  for (sourceType of ['script', 'module']) {
    self.postMessage({ type: 'progress', message: `parsing as ${sourceType}...` });
    expected = Acorn.parse(source, sourceType);
    if (expected !== null) {
      break;
    }
  }
  if (expected === null) {
    sourceType = 'script';
  }

  self.postMessage({ type: 'progress', message: 'estree...' });
  const actual = await server.parse(source, sourceType);
  if (actual === null && expected !== null) {
    self.postMessage({ type: 'fail', reason: `estree cannot parse ${sourceType}` });
    return;
  }
  if (actual !== null && expected === null) {
    self.postMessage({ type: 'fail', reason: `estree should fail parsing ${sourceType}` });
    return;
  }

  self.postMessage({ type: 'progress', message: 'comparing...' });
  // TODO: compute diffs
  if (!equal(actual, expected)) {
    self.postMessage({ type: 'fail', reason: `estree of ${sourceType} mismatch` });
    return;
  }

  self.postMessage({ type: 'pass' });
};
