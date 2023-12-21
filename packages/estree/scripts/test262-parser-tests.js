'use strict';

import * as log from 'https://deno.land/std@0.209.0/log/mod.ts';
import * as path from 'https://deno.land/std@0.209.0/path/mod.ts';
import { TextLineStream, toTransformStream } from 'https://deno.land/std@0.209.0/streams/mod.ts';

import ora from 'npm:ora@7.0.1';
import microdiff from 'https://deno.land/x/microdiff@v1.3.2/index.ts';

import { parseCommand } from '../../../tools/lib/cli.js';
import { VENDOR_DIR } from '../../../tools/lib/consts.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const DEFAULT_DIR = path.join(VENDOR_DIR, 'tc39', 'test262-parser-tests');

const DOC = `
Usage:
  ${PROGNAME} [options] [<dir>]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --details
    Show the details of failed tests.

  --with-debug-build
    Test with the debug build binary.
    Testing with this option is better but 8x slower.

    Some kind of runtime errors such as arithmetic underflow cannot be
    detected in the release build.

Arguments:
  <dir> [default: ${DEFAULT_DIR}]
    Path to tc39/test262-parser-tests.
`.trim();

const { cmds, options, args } = await parseCommand({
  doc: DOC,
});

args.dir ||= DEFAULT_DIR;

// TODO: remove
const EXCLUDES = [
  // infinite loop
  'fail/8ba15f5246ca756c.js',
];

// The signal handler must be registered before starting the bee-estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner?.stop();
  // We cannot call server?.stop() here because it's async method...
  Deno.exit(0);
});

const spinner = ora({ spinner: 'line' });

class EstreeServer {
  constructor() {
    const args = ['run', '-r', '-q', '-p', 'bee-estree', '--', "serve"];
    if (options.withDebugBuild) {
      args.splice(1, 1);  // remove '-r'
    }
    const cmd = new Deno.Command('cargo', {
      args,
      stdin: 'piped',
      stdout: 'piped',
      stderr: 'null',
    });
    this.child_ = cmd.spawn();
    this.lines_ = this.child_.stdout
      .pipeThrough(new TextDecoderStream())
      .pipeThrough(new TextLineStream());
    this.encoder_ = new TextEncoder();
  }

  async parse(source, sourceType) {
    const req = this.encoder_.encode(JSON.stringify({ sourceType, source }) + '\n');

    const writer = this.child_.stdin.getWriter();
    await writer.write(req);
    writer.releaseLock();

    const reader = this.lines_.getReader();
    const res = JSON.parse((await reader.read()).value);
    reader.releaseLock();

    return res.program;
  }

  async stop() {
    await this.child_.stdin.close();
    await this.child_.status;
  }
}

// Spawn bee-estree in the server mode in order to reduce overhead of process creations.
const server = new EstreeServer();

let count = 0;
const fails = [];
const skipped = [];

if (options.progress) {
  spinner.start();
}

for await (const entry of Deno.readDir(path.join(args.dir, 'pass'))) {
  if (!entry.isFile) {
    continue;
  }

  count++;

  const test = path.join('pass', entry.name);
  if (EXCLUDES.includes(test)) {
    skipped.push(test);
    continue;
  }

  spinner.text = test;

  const source = await Deno.readTextFile(path.join(args.dir, test));
  const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';
  let result;
  try {
    result = await server.parse(source, sourceType);
  } catch (err) {
    spinner.warn(`${test}: server.parse() aborted`);
  }

  const testExplicit = path.join('pass-explicit', entry.name);

  spinner.text = testExplicit;

  const sourceExplicit = await Deno.readTextFile(path.join(args.dir, test));
  let resultExplicit;
  try {
    resultExplicit = await server.parse(sourceExplicit, sourceType);
  } catch (err) {
    spinner.warn(`${testExplicit}: server.parse() aborted`);
  }

  if (result && resultExplicit) {
    const diff = microdiff(result, resultExplicit);
    if (diff.length > 0) {
      fails.push({ test, diff });
    }
  } else {
    fails.push({ test });
  }
}

for await (const entry of Deno.readDir(path.join(args.dir, 'fail'))) {
  if (!entry.isFile) {
    continue;
  }

  count++;

  const test = path.join('fail', entry.name);
  if (EXCLUDES.includes(test)) {
    skipped.push(test);
    continue;
  }

  spinner.text = test;

  const source = await Deno.readTextFile(path.join(args.dir, test));
  const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';
  let result;
  try {
    result = await server.parse(source, sourceType);
  } catch (err) {
    spinner.warn(`${test}: server.parse() aborted: ${err}`);
  }
  if (result !== null) {
    fails.push({ test });
  }
}

for await (const entry of Deno.readDir(path.join(args.dir, 'early'))) {
  if (!entry.isFile) {
    continue;
  }

  count++;

  const test = path.join('early', entry.name);
  if (EXCLUDES.includes(test)) {
    skipped.push(test);
    continue;
  }

  spinner.text = test;

  const source = await Deno.readTextFile(path.join(args.dir, test));
  const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';
  let result;
  try {
    result = await server.parse(source, sourceType);
  } catch (err) {
    spinner.warn(`${test}: server.parse() aborted: ${err}`);
  }
  if (result !== null) {
    fails.push({ test });
  }
}

spinner.stop();
await server.stop();

if (options.details) {
  console.log('SKIPPED TESTS:');
  for (const skip of skipped) {
    console.log(`  ${skip}`);
  }
  console.log('FAILED TESTS:');
  for (const fail of fails) {
    console.log(`  ${fail.test}`);
  }
}

const passed = count - fails.length - skipped.length;
console.log(
  `${count} tests: ${passed} passed, ${skipped.length} skipped, ${fails.length} failed`);

Deno.exit(fails.length > 0 ? 1 : 0);
