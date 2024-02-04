import { readAll } from 'https://deno.land/std@0.214.0/io/read_all.ts';
import * as changeCase from 'https://deno.land/x/case@2.2.0/mod.ts';
import { default as docopt } from 'https://deno.land/x/docopt@v1.0.7/mod.ts';

export async function parseCommand({ doc, conv, init }) {
  try {
    const raw = docopt(doc, init);
    let cmds = [];
    let options = {};
    let args = {};
    for (const [name, value] of Object.entries(raw)) {
      if (name.startsWith('--')) {
        options[changeCase.camelCase(name.slice(2))] = conv ? await conv(name, value) : value;
      } else if (name.startsWith('<')) {
        args[changeCase.camelCase(name.slice(1, -1))] = conv ? await conv(name, value) : value;
      } else if (value) {
        cmds.push(name);
      }
    }
    return { cmds, options, args, };
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
