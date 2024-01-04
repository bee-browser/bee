'use strict';

import deepDiff from 'npm:deep-diff@1.0.2';

import { Acorn, ESTree } from './helpers.js';

// TODO: remove
const IGNORES = {
  // Half surrogates.
  // These are not supported in Rust's Strings.
  // Probably, we have to use Vec<u16> or implement DomString.
  'core-js': [
    'body.0.expression.argument.callee.body.body.2.expression.expressions.0.right.elements.387.body.body.1.expression.arguments.0.properties.2.value.arguments.0.body.body.0.argument.left.value',
    'body.0.expression.argument.callee.body.body.2.expression.expressions.0.right.elements.93.body.body.0.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.callee.body.body.2.expression.expressions.0.right.elements.93.body.body.0.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  tensorflow: [
    'body.0.expression.argument.arguments.1.body.body.368.expression.arguments.0.properties.2.value.arguments.0.body.body.0.argument.left.value',
    'body.0.expression.argument.arguments.1.body.body.57.declarations.34.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.57.declarations.34.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  KaTeX: [
    'body.0.expression.argument.arguments.1.body.body.0.argument.callee.body.body.116.declarations.6.init.left.left.left.left.right.value',
    'body.0.expression.argument.arguments.1.body.body.0.argument.callee.body.body.116.declarations.6.init.right.value',
  ],
  amis: [
    'body.441.expression.arguments.1.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.441.expression.arguments.1.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  mathjs: [
    'body.0.expression.argument.arguments.1.body.callee.body.body.0.declarations.0.init.properties.226.value.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.callee.body.body.0.declarations.0.init.properties.226.value.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  exceljs: [
    'body.0.expression.argument.arguments.0.body.body.0.argument.callee.arguments.0.properties.361.value.elements.0.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.0.body.body.0.argument.callee.arguments.0.properties.361.value.elements.0.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'json-editor': [
    'body.0.expression.argument.arguments.1.body.callee.body.body.0.declarations.0.init.properties.203.value.body.body.0.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.callee.body.body.0.declarations.0.init.properties.203.value.body.body.0.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'chatui-core': [
    'body.0.expression.argument.arguments.1.body.body.138.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.138.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'simple-keyboard': [
    'body.0.expression.argument.arguments.1.body.body.0.argument.callee.body.body.1.declarations.0.init.properties.166.value.body.body.0.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.0.argument.callee.body.body.1.declarations.0.init.properties.166.value.body.body.0.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'vis-network': [
    'body.0.expression.argument.arguments.1.body.body.81.declarations.27.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.81.declarations.27.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'ali-oss': [
    'body.0.expression.callee.body.body.0.expression.argument.arguments.0.body.body.0.argument.callee.arguments.0.properties.284.value.elements.0.body.body.2.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.callee.body.body.0.expression.argument.arguments.0.body.body.0.argument.callee.arguments.0.properties.284.value.elements.0.body.body.2.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  jsonld: [
    'body.0.expression.argument.arguments.1.body.body.0.argument.arguments.0.elements.117.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.0.argument.arguments.0.elements.117.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  jodit: [
    'body.0.expression.argument.arguments.1.body.body.0.argument.callee.body.body.0.declarations.1.init.properties.269.value.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.0.argument.callee.body.body.0.declarations.1.init.properties.269.value.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'vis-timeline': [
    'body.0.expression.argument.arguments.1.body.body.67.declarations.28.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.1.body.body.67.declarations.28.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'emoji-js': [
    'body.1.expression.argument.callee.object.body.body.2.expression.expressions.12.right.body.body.1.consequent.body.4.expression.expressions.1.right.arguments.0.right.value',
  ],
  parse: [
    'body.0.expression.argument.arguments.0.body.body.0.argument.callee.arguments.0.properties.460.value.elements.0.body.body.2.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.argument.arguments.0.body.body.0.argument.callee.arguments.0.properties.460.value.elements.0.body.body.2.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'google-libphonenumber': [
    'body.0.expression.arguments.0.body.body.1.argument.callee.arguments.0.properties.0.value.elements.0.body.body.0.expression.callee.object.body.body.0.expression.callee.object.body.body.565.expression.right.value',
    'body.0.expression.arguments.0.body.body.1.argument.callee.arguments.0.properties.0.value.elements.0.body.body.0.expression.callee.object.body.body.0.expression.callee.object.body.body.564.expression.right.value',
  ],
  'googlemaps-js-api-loader': [
    'body.0.expression.expressions.3.right.callee.body.body.31.declarations.36.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.expressions.3.right.callee.body.body.31.declarations.36.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'citation-js': [
    'body.0.expression.right.arguments.0.properties.515.value.elements.0.body.body.2.expression.arguments.0.properties.2.value.arguments.0.body.body.0.argument.left.value',
    'body.0.expression.right.arguments.0.properties.416.value.elements.0.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.right.arguments.0.properties.416.value.elements.0.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  crayons: [
    'body.1.expression.callee.object.body.body.1.consequent.body.0.expression.argument.callee.body.body.1.expression.argument.arguments.0.elements.116.body.body.0.declarations.8.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.1.expression.callee.object.body.body.1.consequent.body.0.expression.argument.callee.body.body.1.expression.argument.arguments.0.elements.116.body.body.0.declarations.8.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  'vis-data': [
    'body.0.expression.arguments.1.body.body.588.declarations.0.init.arguments.0.body.body.0.argument.left.left.arguments.0.value',
    'body.0.expression.arguments.1.body.body.588.declarations.0.init.arguments.0.body.body.0.argument.right.left.arguments.0.value',
  ],
  fusioncharts: [
    'body.0.expression.expressions.3.arguments.1.body.body.0.argument.object.arguments.0.elements.137.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.left.right.arguments.0.value',
    'body.0.expression.expressions.3.arguments.1.body.body.0.argument.object.arguments.0.elements.137.body.body.1.declarations.22.init.arguments.0.body.body.0.argument.right.right.arguments.0.value',
  ],
  // Legacy octal escape sequence
  weld: [
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.7.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.6.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.5.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.4.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.3.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.2.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.1.value.value',
    'body.0.expression.callee.body.body.3.declarations.0.init.properties.0.value.value',
  ],
};

self.onmessage = async ({ data }) => {
  const { name, url, options } = data;

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
  const actual = await ESTree.parse(source, sourceType, options);
  if (actual === null && expected === null) {
    self.postMessage({ type: 'skip', reason: `acorn fails parsing` });
    return;
  }
  if (actual === null && expected !== null) {
    self.postMessage({ type: 'fail', reason: `estree fails parsing ${sourceType}` });
    return;
  }
  if (actual !== null && expected === null) {
    self.postMessage({ type: 'fail', reason: `estree should fail parsing ${sourceType}` });
    return;
  }

  self.postMessage({ type: 'progress', message: 'comparing...' });
  const diffs = deepDiff(actual, expected);
  if (diffs) {
    let ignore = false;
    if (name in IGNORES) {
      ignore = true;
      // There is no diff.kind of 'A' at this point.
      for (const diff of diffs) {
        const path = diff.path.join('.');
        if (IGNORES[name].includes(path)) {
          continue;
        }
        ignore = false;
      }
    }
    if (!ignore) {
      self.postMessage({ type: 'fail', reason: `${sourceType}'s estree mismatch`, diffs });
      return;
    }
  }

  self.postMessage({ type: 'pass' });
};
