'use strict';

import * as log from 'https://deno.land/std@0.208.0/log/mod.ts';
import * as path from 'https://deno.land/std@0.208.0/path/mod.ts';
import { TextLineStream, toTransformStream } from 'https://deno.land/std@0.208.0/streams/mod.ts';

import ora from 'npm:ora@7.0.1';
import * as acorn from 'npm:acorn@8.11.2';
import TestStream from 'npm:test262-stream@1.4.0';
import microdiff from 'https://deno.land/x/microdiff@v1.3.2/index.ts';

import { parseCommand } from '../../../tools/lib/cli.js';
import { VENDOR_DIR } from '../../../tools/lib/consts.js';

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

Arguments:
  <test262-dir> [default: ${DEFAULT_TEST262_DIR}]
    Path to tc39/test262.
`.trim();

const { cmds, options, args } = await parseCommand({
  doc: DOC,
});

args.test262Dir = args.test262Dir || DEFAULT_TEST262_DIR;

// TODO: Remove
const IGNORE_FILES = [
  // panicked in bee_jsparser::literal_content_to_string().
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
  // panicked at /home/masnagam/workspace/bee-browser/bee/packages/jsparser/src/parser/mod.rs:315:30
  'test/language/module-code/import-assertions/import-assertion-newlines.js',
  'test/language/module-code/import-attributes/import-attribute-newlines.js',
];

const UNSUPPORTED_FEATURES = [
  // ES2023
  'hashbang',
];

function parse(test) {
  try {
    return acorn.parse(test.contents, {
      sourceType: test.attrs.flags.module ? 'module' : 'script',
      ecmaVersion: 2022,
    });
  } catch (err) {
    return null;
  }
}

// The signal handler must be registered before starting the bee-estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner?.stop();
  // We cannot call server?.stop() here because it's async method...
  Deno.exit(0);
});

const spinner = ora({ spinner: 'line' });

class EstreeServer {
  constructor() {
    const cmd = new Deno.Command('cargo', {
      args: ['run', '-r', '-q', '-p', 'bee-estree', '--', "serve"],
      stdin: 'piped',
      stdout: 'piped',
      stderr: 'null',
    });
    this.child_ = cmd.spawn();
    this.lines_ = this.child_.stdout
      .pipeThrough(new TextDecoderStream())
      .pipeThrough(new TextLineStream());
    this.encoder_ = new TextEncoder();
  }

  async parse(test) {
    const req = this.encoder_.encode(JSON.stringify({
      sourceType: test.attrs.flags.module ? 'module' : 'script',
      source: test.contents,
    }) + '\n');

    const writer = this.child_.stdin.getWriter();
    await writer.write(req);
    writer.releaseLock();

    const reader = this.lines_.getReader();
    const res = JSON.parse((await reader.read()).value);
    reader.releaseLock();

    return res.program;
  }

  async stop() {
    await this.child_.stdin.close();
    await this.child_.status;
  }
}

// Spawn bee-estree in the server mode in order to reduce overhead of process creations.
const server = new EstreeServer();

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
    s += this.attrs.flags.module ? ': module' : ': script';
    if (this.scenario === 'strict mode') {
      s += '/strict';
    }
    if (this.attrs.features) {
      s += ': ' + this.attrs.features.join(' ');
    }
    return s;
  };

  count++;
  spinner.text = test.file;

  let expected;
  if (test.attrs.negative?.phase === "parse" || test.attrs.negative?.phase === "early") {
    // Error cases.  We don't need to run acorn.parse().
    expected = null;
  } else {
    expected = parse(test);
    if (expected === null) {
      // Acorn cannot parse test.contents.
      skipped.push(test);
      continue;
    }
  }

  const actual = await server.parse(test);

  if (expected === null) {
    if (expected !== actual) {
      fails.push({ test });
    }
  } else {
    let diff;
    if (actual === null) {
      diff = microdiff({}, expected)
    } else {
      diff = microdiff(actual, expected);
    }
    if (diff.length > 0) {
      fails.push({ test,  diff });
    }
  }
}

spinner.stop();
await server.stop();

if (options.details) {
  console.log('SKIPPED TESTS:');
  for (const skip of skipped) {
    console.log(`  ${skip}`);
  }
  console.log('FAILED TESTS:');
  for (const fail of fails) {
    console.log(`  ${fail.test}`);
  }
}

const passed = count - fails.length - skipped.length;
console.log(
  `${count} tests: ${passed} passed, ${skipped.length} skipped, ${fails.length} failed`);

Deno.exit(fails.length > 0 ? 1 : 0);
