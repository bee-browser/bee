'use strict';

import * as path from 'std/path/mod.ts';
import * as yaml from 'std/encoding/yaml.ts';
import { deepmerge } from 'deepmerge';
import { parseCommand } from '../lib/cli.js';

const PROGNAME = path.basename(Deno.mainModule);

const DOC = `
Merge structured data stored in files like JSON and YAML.

Usage:
  ${PROGNAME} [options] <files>...
  ${PROGNAME} -h | --help

Options:
  -i, --input=<format>  [default: json]
    json or yaml.

  -o, --output=<format>  [default: json]
    json or yaml.

Arguments:
  <files>
    Files to be merged.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
  conv: async (name, value) => {
    switch (name) {
    case '--input':
    case '--output':
      if (!['json', 'yaml'].includes(value)) {
        throw new Error('json or yaml');
      }
      return value;
    default:
      return value;
    }
  },
});

Deno.exit(await run(args.files, options));

async function run(files, options) {
  let parse, stringify;
  switch (options.input) {
  case 'json':
    parse = JSON.parse;
    break;
  case 'yaml':
    parse = yaml.parse;
    break;
  }
  switch (options.output) {
  case 'json':
    stringify = JSON.stringify;
    break;
  case 'yaml':
    stringify = (o) => yaml.stringify(o, { lineWidth: -1 });
    break;
  }
  console.log(stringify(deepmerge(
    ...await Promise.all(files.map(async (file) => parse(await Deno.readTextFile(file)))))));
}