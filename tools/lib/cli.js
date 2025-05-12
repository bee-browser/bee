'use strict';

import { readAll } from '@std/io';
import { camelCase } from 'change-case';
import { default as docoptModule } from 'docopt';
const docopt = docoptModule.default;

export async function parseCommand({ doc, conv, init }) {
  try {
    const raw = docopt(doc, init);
    let cmds = [];
    let options = {};
    let args = {};
    for (const [name, value] of Object.entries(raw)) {
      if (name.startsWith('--')) {
        options[camelCase(name.slice(2))] = conv ? await conv(name, value) : value;
      } else if (name.startsWith('<')) {
        args[camelCase(name.slice(1, -1))] = conv ? await conv(name, value) : value;
      } else if (value) {
        cmds.push(name);
      }
    }
    return { cmds, options, args };
  } catch (err) {
    console.error(err.message);
    // FIXME
    // -----
    // `docopt()` throws an `Exit` error object when one of `-h`, `--help` and `--version` was
    // specified.  These cases should be treated as normal cases and the command should exit with
    // status code 0.
    //
    // It's difficult for the current implementation of `docopt()` to distinguish a normal status
    // and an abnormal status only using the error object thrown.
    Deno.exit(0);
  }
}

export async function runCommand(cmd) {
  const status = await Deno.run({ cmd }).status();
  return status.code;
}

export async function readAllText(reader) {
  const decoder = new TextDecoder('utf-8');
  return decoder.decode(await readAll(reader));
}
