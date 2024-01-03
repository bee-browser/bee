'use strict';

import * as path from 'https://deno.land/std@0.209.0/path/mod.ts';

import deepDiff from 'npm:deep-diff@1.0.2';
import ora from 'npm:ora@7.0.1';

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
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
});

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
  spinner.fail(`estree cannot parse ${sourceType}`);
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
