'use strict';
import * as path from 'https://deno.land/std@0.209.0/path/mod.ts';
import { TextLineStream, toTransformStream } from 'https://deno.land/std@0.209.0/streams/mod.ts';

import ora from 'npm:ora@7.0.1';
import * as acorn from 'npm:acorn@8.11.2';
import JSON5 from 'npm:json5@2.2.3';
import microdiff from 'https://deno.land/x/microdiff@v1.3.2/index.ts';

import { parseCommand, readAllText } from '../../../tools/lib/cli.js';
import { VENDOR_DIR } from '../../../tools/lib/consts.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --module
    Parse as a module.
`.trim();

const { cmds, options, args } = await parseCommand({
  doc: DOC,
});

class Acorn {
  static parse(source, sourceType) {
    try {
      return acorn.parse(source, {
        sourceType,
        ecmaVersion: 2022,
      });
    } catch (err) {
      return null;
    }
  }
}

class Estree {
  static async parse(source, sourceType) {
    const args = ['run', '-r', '-q', '-p', 'bee-estree', '--', 'parse', sourceType];
    if (options.withDebugBuild) {
      args.splice(1, 1);  // remove '-r'
    }
    const child = new Deno.Command('cargo', {
      args,
      stdin: 'piped',
      stdout: 'piped',
      stderr: 'null',
    }).spawn();
    const encoder = new TextEncoder();
    const writer = child.stdin.getWriter();
    writer.write(encoder.encode(source));
    writer.releaseLock();
    child.stdin.close();
    try {
      const decoder = new TextDecoder();
      const output = await child.output();
      return JSON5.parse(decoder.decode(output.stdout));
    } catch (err) {
      return null;
    }
  }
}

const spinner = ora({ spinner: 'line' });

// The signal handler must be registered before starting the bee-estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner.stop();
  Deno.exit(0);
});

if (options.progress) {
  spinner.start();
}

spinner.text = 'Reading source from STDIN...'
const source = await readAllText(Deno.stdin);
const sourceType = options.module ? 'module' : 'script';

spinner.text = 'Parsing with acorn...'
const expected = Acorn.parse(source, sourceType);

spinner.text = 'Parsing with estree...'
const actual = await Estree.parse(source, sourceType);

spinner.text = 'Comparing ESTrees...'
const diffs = microdiff(actual, expected);

spinner.stop();

if (diffs.length === 0) {
  Deno.exit(0);
}

for (const diff of diffs) {
  console.log(`${diff.path.join('.')}`);
  if (diff.value) {
    console.log(`  acorn : ${JSON5.stringify(diff.value)}`);
  } else {
    console.log('  acorn : -');
  }
  if (diff.oldValue) {
    console.log(`  estree: ${JSON5.stringify(diff.oldValue)}`);
  } else {
    console.log('  estree: -');
  }
  console.log('');
}
Deno.exit(1);
