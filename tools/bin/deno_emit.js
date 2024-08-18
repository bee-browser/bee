'use strict';

import * as fs from '@std/fs';
import * as path from '@std/path';
import { bundle } from '@deno/emit';
import { parseCommand, readAllText } from '../lib/cli.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Take a root module and all its dependencies and emit a single JavaScript bundle.

USAGE:
  ${PROGNAME} <root>
  ${PROGNAME} -h | --help

ARGUMENTS:
  <root>
    Path to the root module file.
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(args, options));

// See https://github.com/denoland/deno_emit/blob/main/js/README.md#bundle
async function run(args, options) {
  const { code } = await bundle(args.root);
  console.log(code.trim());
}
