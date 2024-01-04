'use strict';

import * as path from 'https://deno.land/std@0.210.0/path/mod.ts';
import { equal } from "https://deno.land/std@0.210.0/testing/asserts.ts";

import deepDiff from 'npm:deep-diff@1.0.2';
import ora from 'npm:ora@7.0.1';

import { parseCommand } from '../../../tools/lib/cli.js';
import { VENDOR_DIR } from '../../../tools/lib/consts.js';
import { Acorn, ESTree, showDiffs } from './helpers.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const DEFAULT_DIR = path.join(VENDOR_DIR, 'tc39', 'test262-parser-tests');

const DOC = `
Usage:
  ${PROGNAME} [options] [<dir>]
  ${PROGNAME} -h | --help

Options:
  --only=(pass|fail|early) [default: all]
    Run only specific tests.

  --progress
    Show progress.

  --details
    Show the details of failed tests.

  --with-debug-build
    Test with the debug build binary.
    Testing with this option is better but 8x slower.

    Some kind of runtime errors such as arithmetic underflow cannot be
    detected in the release build.

Arguments:
  <dir> [default: ${DEFAULT_DIR}]
    Path to tc39/test262-parser-tests.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
});

options.only ||= 'all';
args.dir ||= DEFAULT_DIR;

// TODO: remove
const EXCLUDES = [
  // B.1.1 HTML-like Comments - SingleLineHTMLOpenComment
  'pass/1270d541e0fd6af8.js',
  'pass/b15ab152f8531a9f.js',
  'pass/4ae32442eef8a4e0.js',
  'pass/d3ac25ddc7ba9779.js',
  'pass/fbcd793ec7c82779.js',
  'pass/8c56cf12f007a392.js',
  'pass/47094fe8a994b7de.js',
  'pass/40215319424a8227.js',
  // B.1.1 HTML-like Comments - SingleLineHTMLCloseComment
  'pass/8ec6a55806087669.js',
  'pass/9f0d8eb6f7ab8180.js',
  'pass/5d5b9de6d9b95f3e.js',
  'pass/946bee37652a31fa.js',
  'pass/e03ae54743348d7d.js',
  'pass/4f5419fe648c691b.js',
  'pass/5a2a8e992fa4fe37.js',
  'pass/c532e126a986c1d4.js',
  'pass/ba00173ff473e7da.js',
  // B.3.2 Block-Level Function Declarations Web Legacy Compatibility Semantics
  'pass/3dabeca76119d501.js',
  'pass/a4d62a651f69d815.js',
  'pass/52aeec7b8da212a2.js',
  'pass/c06df922631aeabc.js',
  'pass/1c1e2a43fe5515b6.js',
  'pass/59ae0289778b80cd.js',
  // invalid character
  // jsparser::lexer::dfa::input_element_reg_exp:
  //   opcode="next"
  //   state=State(423)
  //   unicode_set=UnicodeSet(69, Some('\u{202f}'))
  //   pos=45
  'pass/8b8edcb36909900b.js',
  // LegacyOctalEscapeSequence
  'pass/d38771967621cb8e.js',
  'pass/cb095c303f88cd0b.js',
  'pass/71e066a0fa01825b.js',
  'pass/2e371094f1b1ac51.js',
  'pass/ade301f0d871c610.js',
  'pass/7b514406528ff126.js',
  'pass/3e48826018d23c85.js',
  'pass/b5cf21a87ec272d1.js',
  'pass/8e3f0660b32fbfd2.js',
  'pass/3fb07536eb5aea8d.js',
  'pass/fa736f4b0cf19c0c.js',
  'pass/20644d335e3cd008.js',
  'pass/3990bb94b19b1071.js',
  'pass/d483926898410cae.js',
  'pass/0b281915a3227177.js',
  'pass/95ab0d795c04ff38.js',
  'pass/27ca96102da82628.js',
  'pass/84b2a5d834daee2f.js',
  'pass/bf6aaaab7c143ca1.js',
];

const spinner = ora({ spinner: 'line' });

// The signal handler must be registered before starting the estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner.stop();
  // We cannot call server?.stop() here because it's async method...
  Deno.exit(0);
});

// Spawn estree in the server mode in order to reduce overhead of process creations.
let server = new ESTree(options);
server.start();

let count = 0;
const fails = [];
const skipped = [];

if (options.progress) {
  spinner.start();
}

if (options.only === 'all' || options.only === 'pass') {
  for await (const entry of Deno.readDir(path.join(args.dir, 'pass'))) {
    if (!entry.isFile) {
      continue;
    }

    count++;

    const test = path.join('pass', entry.name);
    if (EXCLUDES.includes(test)) {
      continue;
    }

    spinner.text = test;

    const source = await Deno.readTextFile(path.join(args.dir, test));
    const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';

    const expected = Acorn.parse(source, sourceType);
    if (expected === null) {
      skipped.push({ test, reason: 'acorn fails parsing' });
      continue;
    }

    const actual = await server.parse(source, sourceType);
    if (actual === null) {
      fails.push({ test, reason: 'estree fails parsing' });
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

    const testExplicit = path.join('pass-explicit', entry.name);

    spinner.text = testExplicit;

    const sourceExplicit = await Deno.readTextFile(path.join(args.dir, test));

    const expectedExplicit = Acorn.parse(sourceExplicit, sourceType);
    if (expectedExplicit === null) {
      skipped.push({ test, reason: 'acorn fails parsing' });
      continue;
    }

    if (!equal(expected, expectedExplicit)) {
      skipped.push({ test, reason: 'acorn fails' });
      continue;
    }

    const actualExplicit = await server.parse(sourceExplicit, sourceType);
    if (actualExplicit === null) {
      fails.push({ test, reason: 'estree fails parsing' });
      continue;
    }

    if (options.details) {
      const diffs = deepDiff(actualExplicit, expectedExplicit);
      if (diffs) {
        fails.push({ test, reason: 'estree mismatch (explicit)', diffs });
        continue;
      }
    } else {
      if (!equal(actualExplicit, expectedExplicit)) {
        fails.push({ test, reason: 'estree mismatch (explicit)' });
        continue;
      }
    }

    // passed
  }
}

if (options.only === 'all' || options.only === 'fail') {
  for await (const entry of Deno.readDir(path.join(args.dir, 'fail'))) {
    if (!entry.isFile) {
      continue;
    }

    count++;

    const test = path.join('fail', entry.name);
    if (EXCLUDES.includes(test)) {
      continue;
    }

    spinner.text = test;

    const source = await Deno.readTextFile(path.join(args.dir, test));
    const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';

    const expected = Acorn.parse(source, sourceType);
    if (expected !== null) {
      skipped.push({ test, reason: 'acorn should fail pasring' });
      continue;
    }

    const actual = await server.parse(source, sourceType);
    if (actual !== null) {
      fails.push({ test, reason: 'estree should fail parsing' });
      continue;
    }

    // passed
  }
}

if (options.only === 'all' || options.only === 'early') {
  for await (const entry of Deno.readDir(path.join(args.dir, 'early'))) {
    if (!entry.isFile) {
      continue;
    }

    count++;

    const test = path.join('early', entry.name);
    if (EXCLUDES.includes(test)) {
      continue;
    }

    spinner.text = test;

    const source = await Deno.readTextFile(path.join(args.dir, test));
    const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';

    const expected = Acorn.parse(source, sourceType);
    if (expected !== null) {
      skipped.push({ test, reason: 'acorn should fail parsing' });
      continue;
    }

    const actual = await server.parse(source, sourceType);
    if (actual !== null) {
      fails.push({ test, reason: 'estree should fail parsing' });
      continue;
    }

    // passed
  }
}

spinner.stop();
await server.stop();

if (options.details) {
  console.log('EXCLUDED:');
  for (const test of EXCLUDES) {
    console.log(`  ${test}`);
  }
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
  `${count} tests: ${passed} passed, ${EXCLUDES.length} excluded, ` +
    `${skipped.length} skipped, ${fails.length} failed`);

Deno.exit(fails.length === 0 ? 0 : 1);
