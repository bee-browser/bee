'use strict';

import * as log from '@std/log';
import * as path from '@std/path';
import { parseCommand } from '../../../tools/lib/cli.js';
import { setup } from '../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} <tests>...
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

if (options.debug) {
  setup(PROGNAME, 'DEBUG');
} else {
  setup(PROGNAME, 'INFO');
}

Deno.exit(await main(args, options));

async function main(args, options) {
  const tests = [];
  for (const test of args.tests) {
    log.debug(`Reading ${test}...`);
    const script = await Deno.readTextFile(test);
    const lines = script.split('\n').map((line) => line.trim());
    const expectedValues = lines.filter((line) => line.includes('///=')).map((line) => line.split('///=')[1].trim());
    const throws = lines.find((line) => line.includes('///!'))?.split('///!')[1].trim();
    tests.push({
      name: path.basename(test, '.js'),
      expectedValues,
      throws,
    });
  }
  console.log(JSON.stringify({ tests }));
  return 0;
}
