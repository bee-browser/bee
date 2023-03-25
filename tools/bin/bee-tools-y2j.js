'use strict';

import * as path from 'std/path/mod.ts';
import * as yaml from 'std/encoding/yaml.ts';
import { parseCommand, readAllText } from '../lib/cli.js';

const PROGNAME = path.basename(Deno.mainModule);

const DOC = `
Convert YAML to JSON.

Usage:
  ${PROGNAME}
  ${PROGNAME} -h | --help

Description:
  This program reads a YAML string from STDIN, converts it to JSON and then
  prints the JSON to STDOUT.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(args.files, options));

async function run(options) {
  console.log(JSON.stringify(yaml.parse(await readAllText(Deno.stdin))));
}
