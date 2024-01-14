'use strict';

import * as path from 'https://deno.land/std@0.212.0/path/mod.ts';
import { default as docopt } from 'https://deno.land/x/docopt@v1.0.7/mod.ts';
import { parseCommand, readAllText } from '../lib/cli.js';
import { LayoutBuilder } from '../lib/layout_builder.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DOC = `
Build a layout tree from a DOM tree scraped by dom-scrapter.

Usage:
  ${PROGNAME} [options] [<file>]
  ${PROGNAME} -h | --help

Options:
  --json
    Output JSON instead of JSONL.

Arguments:
  <file>
    Path to a JSON file containing the DOM tree.

    Read a JSON from STDIN if no file is specified.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(args.file, options));

async function run(file, options) {
  const dom = JSON.parse(await loadJson(file));
  const builder = new LayoutBuilder(dom);
  for (let instruction of builder.build()) {
    console.log(JSON.stringify(instruction));
  }
}

async function loadJson(file) {
  if (file) {
    return await Deno.readTextFile(file);
  }
  return await readAllText(Deno.stdin);
}
