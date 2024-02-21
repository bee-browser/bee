'use strict';

import * as path from 'https://deno.land/std@0.216.0/path/mod.ts';

import deepDiff from 'npm:deep-diff@1.0.2';
import ora from 'npm:ora@^7.0.1';  // 8.0.1 does not work w/ deno

import { parseCommand, readAllText } from '../../../tools/lib/cli.js';
import { Acorn, ESTree, showDiffs } from './helpers.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --mode=(release | debug | coverage) [default: release]
    Choice one of the following modes:
      release: Use the release build
      debug: Use the debug build
      coverage: Use cargo-llvm-cov
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
});

options.mode ||= 'release';

const spinner = ora({ spinner: 'line' });

// The signal handler must be registered before starting the estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner.stop();
  Deno.exit(0);
});

if (options.progress) {
  spinner.start();
}

spinner.text = 'loading source from STDIN...';
const source = await readAllText(Deno.stdin);

let sourceType, expected;
for (sourceType of ['script', 'module']) {
  spinner.text = 'parsing as ${sourceType}...';
  expected = Acorn.parse(source, sourceType);
  if (expected !== null) {
    break;
  }
}
if (expected === null) {
  sourceType = 'script';
}

spinner.text = 'estree...';
const actual = await ESTree.parse(source, sourceType, options);
if (actual === null && expected !== null) {
  spinner.fail(`estree fails parsing ${sourceType}`);
  Deno.exit(1);
}
if (actual !== null && expected === null) {
  spinner.fail(`estree should fail parsing ${sourceType}`);
  Deno.exit(1);
}

spinner.text = 'comparing...';
const diffs = deepDiff(actual, expected);

spinner.stop();

if (diffs) {
  showDiffs(diffs);
  Deno.exit(1);
}

Deno.exit(0);
