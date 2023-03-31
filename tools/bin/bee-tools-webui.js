'use strict';

import * as path from 'https://deno.land/std@0.181.0/path/mod.ts';
import { parseCommand, runCommand } from '../lib/cli.js';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DOC = `
Web UI for testing.

Usage:
  bee-tools-webui serve <args>...
  bee-tools-webui stop <args>...
  bee-tools-webui check <args>...
  bee-tools-webui -h | --help
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
      Deno.exit(await runCommand([`bee-tools-webui-${cmd}`, '--help']));
    } else {
      Deno.exit(await runCommand([`bee-tools-webui-${cmd}`, ...Deno.args.filter((v) => v !== cmd)]));
    }
  } catch (err) {
    if (err instanceof Deno.errors.NotFound) {
      console.error(`'${cmd}' is not a bee-tools-webui command.  See 'bee-tools-webui --help'.`);
    } else {
      console.error(err.message);
    }
    Deno.exit(2);
  }
}
