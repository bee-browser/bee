'use strict';

import * as path from 'https://deno.land/std@0.181.0/path/mod.ts';
import { parseCommand, runCommand } from '../lib/cli.js';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DOC = `
Tools for analyses.

Usage:
  bee-tools-analysis tracing <args>...
  bee-tools-analysis -h | --help
`.trim();

const { cmds, args, } = await parseCommand({
  doc: DOC,
  init: { optionsFirst: true },
});

Deno.exit(await run(cmds, args));

async function run(cmds, args, options) {
  const showHelp = cmds[0] === 'help';
  const cmd = showHelp ? args.cmd : cmds[0];
  try {
    if (showHelp) {
      Deno.exit(await runCommand([`bee-tools-analysis-${cmd}`, '--help']));
    } else {
      Deno.exit(await runCommand([`bee-tools-analysis-${cmd}`, ...Deno.args.filter((v) => v !== cmd)]));
    }
  } catch (err) {
    if (err instanceof Deno.errors.NotFound) {
      console.error(
        `'${cmd}' is not a bee-tools-analysis sub-command.  ` +
          `See 'bee-tools-analysis --help'.`);
    } else {
      console.error(err.message);
    }
    Deno.exit(2);
  }
}
