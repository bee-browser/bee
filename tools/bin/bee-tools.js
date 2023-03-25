'use strict';

import * as path from 'std/path/mod.ts';
import { parseCommand, runCommand } from '../lib/cli.js';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DOC = `
Tools for developers in the BEE project.

Usage:
  bee-tools analysis <args>...
  bee-tools codegen <args>...
  bee-tools deepmerge <args>...
  bee-tools dom-scraper <args>...
  bee-tools layout-builder <args>...
  bee-tools text-to-dot-matrix <args>...
  bee-tools top-sites <args>...
  bee-tools update-file <args>...
  bee-tools y2j
  bee-tools webui <args>...
  bee-tools help <cmd>
  bee-tools -h | --help
`.trim();

const { cmds, args, } = await parseCommand({
  doc: DOC,
  init: {
    optionsFirst: true,
  },
});

Deno.exit(await run(cmds, args));

async function run(cmds, args, options) {
  const showHelp = cmds[0] === 'help';
  const cmd = showHelp ? args.cmd : cmds[0];
  try {
    if (showHelp) {
      Deno.exit(await runCommand([`bee-tools-${cmd}`, '--help']));
    } else {
      Deno.exit(await runCommand([`bee-tools-${cmd}`, ...Deno.args.filter((v) => v !== cmd)]));
    }
  } catch (err) {
    if (err instanceof Deno.errors.NotFound) {
      console.error(`'${cmd}' is not a bee-tools command.  See 'bee-tools --help'.`);
    } else {
      console.error(err.message);
    }
    Deno.exit(2);
  }
}
