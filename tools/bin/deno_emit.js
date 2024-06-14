'use strict';

import * as fs from 'https://deno.land/std@0.224.0/fs/mod.ts';
import * as path from 'https://deno.land/std@0.224.0/path/mod.ts';
import { bundle } from "https://deno.land/x/emit@0.40.0/mod.ts";
import { parseCommand, readAllText } from '../lib/cli.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Take a root module and all its dependencies and emit a single JavaScript bundle.

Usage:
  ${PROGNAME} <root>
  ${PROGNAME} -h | --help

Arguments:
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
