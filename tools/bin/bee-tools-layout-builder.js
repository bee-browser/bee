'use strict';

import * as path from 'std/path/mod.ts';
import * as changeCase from 'case';
import { default as docopt } from 'docopt';
import { default as puppeteer } from 'puppeteer';
import { parseCommand, readAllText } from '../lib/cli.js';
import { LayoutBuilder } from '../lib/layout_builder.js';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DOC = `
Build a layout tree from a DOM tree scraped by dom-scrapter.

Usage:
  bee-tools-layout-builder [options] [<file>]
  bee-tools-layout-builder -h | --help

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
