'use strict';

import * as path from 'https://deno.land/std@0.210.0/path/mod.ts';
import { equal } from "https://deno.land/std@0.210.0/testing/asserts.ts";

import deepDiff from 'npm:deep-diff@1.0.2';
import ora from 'npm:ora@7.0.1';
import TestStream from 'npm:test262-stream@1.4.0';

import { parseCommand } from '../../../tools/lib/cli.js';
import { VENDOR_DIR } from '../../../tools/lib/consts.js';
import { Acorn, ESTree, showDiffs } from './helpers.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const DEFAULT_TEST262_DIR = path.join(VENDOR_DIR, 'tc39', 'test262');

const DOC = `
Usage:
  ${PROGNAME} [options] [<test262-dir>]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --details
    Show the details of failed tests.

  --mode=(release | debug | coverage) [default: release]
    Choice one of the following modes:
      release: Use the release build
      debug: Use the debug build
      coverage: Use cargo-llvm-cov

Arguments:
  <test262-dir> [default: ${DEFAULT_TEST262_DIR}]
    Path to tc39/test262.
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

options.mode ||= 'release';
args.test262Dir ||= DEFAULT_TEST262_DIR;

// TODO: Remove
const IGNORE_FILES = [
  // panicked in jsparser::literal_content_to_string().
  // implement conversion from a string literal into a string value in lexer.
  'test/language/expressions/tagged-template/invalid-escape-sequences.js',
  'test/language/expressions/template-literal/invalid-hexidecimal-character-escape-sequence-truncated-1.js',
  'test/language/expressions/template-literal/invalid-hexidecimal-character-escape-sequence-truncated-2.js',
  'test/language/expressions/template-literal/invalid-hexidecimal-character-escape-sequence-truncated-3.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-1.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-2.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-3.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-4.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-5.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-6.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-7.js',
  'test/language/expressions/template-literal/invalid-unicode-escape-sequence-8.js',
  'test/language/expressions/template-literal/unicode-escape-nls-err.js',
  // panicked at //libs/jsparser/src/parser/mod.rs:315:30
  'test/language/module-code/import-assertions/import-assertion-newlines.js',
  'test/language/module-code/import-attributes/import-attribute-newlines.js',
];

const UNSUPPORTED_FEATURES = [
  // ES2023
  'hashbang',
];

// The signal handler must be registered before starting the estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner?.stop();
  // We cannot call server?.stop() here because it's async method...
  Deno.exit(0);
});

const spinner = ora({ spinner: 'line' });

// Spawn estree in the server mode in order to reduce overhead of process creations.
const server = new ESTree(options);
server.start();

const stream = new TestStream(args.test262Dir, {
  // Directory from which to load "includes" files (defaults to the
  // appropriate subdirectory of the provided `test262Dir`
  // Optional. Defaults to './harness'
  //includesDir: '/path/to/includes/dir',

  // File system paths refining the set of tests that should be produced;
  // only tests whose source file matches one of these values (in the case of
  // file paths) or is contained by one of these paths (in the case of
  // directory paths) will be created; all paths are interpreted relative to
  // the root of the provided `test262Dir`
  // Optional. Defaults to ['test']
  //paths: ['test/built-ins/eval', 'test/language/statements/empty/S12.3_A1.js'],

  // Flag to disable the insertion of code necessary to execute the test
  // (e.g. assertion functions and "include" files); defaults to `false`
  omitRuntime: true,

  // By default, this stream will emit an error if the provided version of
  // Test262 is not supported; this behavior may be disabled by providing a
  // value of the expected version. Use of this option may cause the stream
  // to emit invalid tests; consider updating the library instead.
  //acceptVersion: '2.0.0'
});

stream.on('error', (err) => console.error('Something went wrong:', err));

let count = 0;
const fails = [];
const skipped = [];

if (options.progress) {
  spinner.start();
}

for await (const test of stream) {
  // the path to the file from which the test was derived, relative to the
  // provided Test262 directory
  //console.log(test.file);

  // the complete source text for the test; this contains any "includes"
  // files specified in the frontmatter, "prelude" content if specified (see
  // below), and any "scenario" transformations
  //console.log(test.contents);

  // an object representation of the metadata declared in the test's
  // "frontmatter" section
  //console.log(test.attrs);

  // the licensing information included within the test (if any)
  //console.log(test.copyright);

  // name describing how the source file was interpreted to create the test
  //console.log(test.scenario);

  // numeric offset within the `contents` string at which one or more
  // statements may be inserted without necessarily invalidating the test
  //console.log(test.insertionIndex);

  if (test.file.endsWith('.md')) {
    // Maybe this is a bug but TestStream collects non-JavaScript files...
    continue;
  }

  if (IGNORE_FILES.includes(test.file)) {
    continue;
  }

  const ignore = UNSUPPORTED_FEATURES.some((feature) => {
    return test.attrs.features?.includes(feature);
  });
  if (ignore) {
    continue;
  }

  test.toString = function() {
    let s = this.file;
    s += this.attrs.flags.module ? '#module' : '#script';
    if (this.scenario === 'strict mode') {
      s += '#strict';
    }
    if (this.attrs.features) {
      s += `#${this.attrs.features.join('#')}`;
    }
    return s;
  };

  count++;

  spinner.text = test.file;

  const source = test.contents;
  const sourceType = test.attrs.flags.module ? 'module' : 'script';

  let expected;
  if (test.attrs.negative?.phase === "parse" || test.attrs.negative?.phase === "early") {
    // Error cases.
    expected = null;
  } else {
    expected = Acorn.parse(source, sourceType);
    if (expected === null) {
      // Acorn cannot parse test.contents.
      skipped.push({ test, reason: 'acorn fails parsing' });
      continue;
    }
  }

  const actual = await server.parse(source, sourceType);
  if (actual === null && expected !== null) {
    fails.push({ test, reason: 'estree fails parsing' });
    continue;
  }
  if (actual !== null && expected === null) {
    fails.push({ test, reason: 'estree should fail parsing' });
    continue;
  }

  if (options.details) {
    const diffs = deepDiff(actual, expected);
    if (diffs) {
      fails.push({ test, reason: 'estree mismatch', diffs });
      continue;
    }
  } else {
    if (!equal(actual, expected)) {
      fails.push({ test, reason: 'estree mismatch' });
      continue;
    }
  }

  // passed
}

spinner.stop();
await server.stop();

if (options.details) {
  console.log('SKIPPED:');
  for (const { test, reason } of skipped) {
    console.log(`  ${test}: ${reason}`);
  }
  console.log('FAILED:');
  for (const { test, reason, diffs } of fails) {
    console.log(`  ${test}: ${reason}`);
    if (diffs) {
      showDiffs(diffs, '    ');
    }
  }
}

const passed = count - fails.length - skipped.length;
console.log(
  `${count} tests: ${passed} passed, ` +
    `${skipped.length} skipped, ${fails.length} failed`);

Deno.exit(fails.length === 0 ? 0 : 1);
