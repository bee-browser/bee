'use strict';

import microdiff from 'https://deno.land/x/microdiff@v1.3.2/index.ts';
import JSON5 from 'npm:json5@2.2.3';

import { Acorn, ESTree } from './test262_helper.js';

// Spawn estree in the server mode in order to reduce overhead of process creations.
let server = new ESTree(); // TODO: options
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
    self.postMessage({ type: 'skip', reason: 'acorn cannot parse' });
    return;
  }

  self.postMessage({ type: 'progress', message: 'estree...' });
  const actual = await server.parse(source, sourceType);
  if (actual === null) {
    self.postMessage({ type: 'fail', reason: `estree cannot parse (${sourceType})` });
    return;
  }

  self.postMessage({ type: 'progress', message: 'comparing...' });
  const diffs = microdiff(actual, expected);
  if (diffs.length > 0) {
    self.postMessage({ type: 'fail', reason: `estree mismatch (${sourceType})` });
    return;
  }

  self.postMessage({ type: 'pass' });
};
