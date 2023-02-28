'use strict';

import * as path from 'std/path/mod.ts';
import { parseCommand } from '../lib/cli.js';

const PROGNAME = path.basename(Deno.mainModule);

const DOC = `
Update the content of a file iff it changes.

Usage:
  ${PROGNAME} [options] <file>
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(args, options));

// TODO: slow...
async function run(args, options) {
  const oldContent = await Deno.readFile(args.file);
  const newContent = await Deno.readAll(Deno.stdin);
  if (oldContent.length != newContent.length) {
    console.log("Size changed");
    await Deno.writeFile(args.file, newContent);
    return;
  }
  for (let i = 0; i < newContent.length; ++i) {
    if (oldContent[i] !== newContent[i]) {
      console.log("Content changed");
      await Deno.writeFile(args.file, newContent);
      return;
    }
  }
}
