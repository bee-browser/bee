'use strict';

import * as log from 'https://deno.land/std@0.204.0/log/mod.ts';
import { assert } from "https://deno.land/std@0.204.0/assert/mod.ts";
import { TextLineStream } from "https://deno.land/std@0.204.0/streams/mod.ts";
import { toTransformStream } from "https://deno.land/std@0.204.0/streams/to_transform_stream.ts";

import * as acorn from 'npm:acorn@8.11.2';
import TestStream from 'npm:test262-stream@1.4.0';
import microdiff from "https://deno.land/x/microdiff@v1.3.2/index.ts";

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
  // automatically inserted semicolon causes infinite loop.
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-error.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-accessor-get-meth.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-accessor-set-meth.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-async-meth.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-field-init.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-field.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-meth.case.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-accessor-get-meth.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-accessor-set-meth.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-async-meth.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-field-init.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-field.js',
  'test/language/expressions/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-meth.js',
  'test/language/statements/class/decorator/syntax/valid/class-element-decorator-call-expr-identifier-reference.js',
  'test/language/statements/class/decorator/syntax/valid/class-element-decorator-member-expr-decorator-member-expr.js',
  'test/language/statements/class/decorator/syntax/valid/class-element-decorator-member-expr-identifier-reference.js',
  'test/language/statements/class/decorator/syntax/valid/class-element-decorator-parenthesized-expr-identifier-reference.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-error.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-accessor-get-meth.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-accessor-set-meth.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-async-meth.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-field-init.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-field.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-meth.case.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-accessor-get-meth.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-accessor-set-meth.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-async-meth.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-field-init.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-field.js',
  'test/language/statements/class/elements/syntax/early-errors/grammar-privatename-whitespace-error-static-meth.js',
];

const UNSUPPORTED_FEATURES = [
  // ES2023
  'hashbang',
];

const stream = new TestStream('/home/masnagam/workspace/bee-browser/bee/vendor/tc39/test262', {
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

const cmd = new Deno.Command('cargo', {
  args: [
    'run',
    '-r',
    '-q',
    '-p',
    'bee-estree',
    '--',
    "server",
  ],
  stdin: 'piped',
  stdout: 'piped',
  stderr: 'null',
});
const server = cmd.spawn();
const lines = server.stdout
  .pipeThrough(new TextDecoderStream())
  .pipeThrough(new TextLineStream())
const encoder = new TextEncoder();

function testToString(test) {
  let s = test.file;
  s += test.attrs.flags.module ? ': module' : ': script';
  if (test.scenario === 'strict mode') {
    s += '/strict';
  }
  if (test.attrs.features) {
    s += ': ' + test.attrs.features.join(' ');
  }
  return s;
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

  const skip = UNSUPPORTED_FEATURES.some((feature) => {
    return test.attrs.features?.includes(feature);
  });
  if (skip) {
    continue;
  }

  log.info(`${testToString(test)}`);

  let expected = null;
  try {
    expected = acorn.parse(test.contents, {
      sourceType: test.attrs.flags.module ? 'module' : 'script',
      ecmaVersion: 2022,
    });
  } catch (err) {}

  const query = encoder.encode(JSON.stringify({
    sourceType: test.attrs.flags.module ? 'module' : 'script',
    source: test.contents,
  }) + '\n')

  const writer = server.stdin.getWriter();
  await writer.write(query);
  writer.releaseLock();

  const reader = lines.getReader();
  const reply = JSON.parse((await reader.read()).value);
  reader.releaseLock();
  const actual = reply.program;

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

  count++;
}

if (fails.length === 0) {
  Deno.exit(0);
} else {
  console.log(`FAILED TESTS:`);
  for (const fail of fails) {
    console.error(`  ${testToString(fail.test)}`);
  }
  console.log(`FAILED: ${fails.length}/${count}`);
  Deno.exit(1);
}
