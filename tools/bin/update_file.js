'use strict';

import * as path from 'https://deno.land/std@0.210.0/path/mod.ts';
import { parseCommand } from '../lib/cli.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

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
  const newContent = await Deno.readAll(Deno.stdin);
  const file = path.resolve(args.file);
  try {
    const oldContent = await Deno.readFile(file);
    if (oldContent.length != newContent.length) {
      console.log(`Updated: ${file}`);
      await Deno.writeFile(file, newContent);
      return;
    }
    for (let i = 0; i < newContent.length; ++i) {
      if (oldContent[i] !== newContent[i]) {
        console.log(`Updated: ${file}`);
        await Deno.writeFile(file, newContent);
        return;
      }
    }
  } catch {
    console.log(`Created: ${file}`);
    await Deno.writeFile(file, newContent);
  }
}
