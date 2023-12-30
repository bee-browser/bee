'use strict';

import * as path from 'https://deno.land/std@0.209.0/path/mod.ts';

import ora from 'npm:ora@7.0.1';
import microdiff from 'https://deno.land/x/microdiff@v1.3.2/index.ts';

import { parseCommand, readAllText } from '../../../tools/lib/cli.js';
import { Acorn, ESTree, showDiffs } from './test262_helper.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --module
    Parse as a module.
`.trim();

const { cmds, options, args } = await parseCommand({
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

spinner.text = 'Reading source from STDIN...'
const source = await readAllText(Deno.stdin);
const sourceType = options.module ? 'module' : 'script';

spinner.text = 'Parsing with acorn...'
const expected = Acorn.parse(source, sourceType);

spinner.text = 'Parsing with estree...'
const actual = await ESTree.parse(source, sourceType, options);

spinner.text = 'Comparing ESTrees...'
const diffs = microdiff(actual, expected);

spinner.stop();

showDiffs(diffs);

Deno.exit(diffs.length === 0 ? 0 : 1);
