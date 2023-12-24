'use strict';

import * as log from 'https://deno.land/std@0.209.0/log/mod.ts';
import * as path from 'https://deno.land/std@0.209.0/path/mod.ts';
import { TextLineStream, toTransformStream } from 'https://deno.land/std@0.209.0/streams/mod.ts';

import ora from 'npm:ora@7.0.1';
import * as acorn from 'npm:acorn@8.11.2';
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
  --only=(pass|fail|early) [default: all]
    Run only specific tests.

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

options.only ||= 'all';
args.dir ||= DEFAULT_DIR;

// TODO: remove
const EXCLUDES = [
  // B.1.1 HTML-like Comments - SingleLineHTMLOpenComment
  'pass/1270d541e0fd6af8.js',
  'pass/b15ab152f8531a9f.js',
  'pass/4ae32442eef8a4e0.js',
  'pass/d3ac25ddc7ba9779.js',
  'pass/fbcd793ec7c82779.js',
  // B.1.1 HTML-like Comments - SingleLineHTMLCloseComment
  'pass/8ec6a55806087669.js',
  'pass/9f0d8eb6f7ab8180.js',
  'pass/5d5b9de6d9b95f3e.js',
  'pass/946bee37652a31fa.js',
  'pass/e03ae54743348d7d.js',
  'pass/4f5419fe648c691b.js',
  'pass/5a2a8e992fa4fe37.js',
  'pass/c532e126a986c1d4.js',
  'pass/ba00173ff473e7da.js',
  // B.3.2 Block-Level Function Declarations Web Legacy Compatibility Semantics
  'pass/3dabeca76119d501.js',
  'pass/a4d62a651f69d815.js',
  'pass/52aeec7b8da212a2.js',
  'pass/c06df922631aeabc.js',
  'pass/1c1e2a43fe5515b6.js',
  'pass/59ae0289778b80cd.js',
  // invalid character
  // bee_jsparser::lexer::dfa::input_element_reg_exp:
  //   opcode="next"
  //   state=State(423)
  //   unicode_set=UnicodeSet(69, Some('\u{202f}'))
  //   pos=45
  'pass/8b8edcb36909900b.js',
];

const spinner = ora({ spinner: 'line' });

// The signal handler must be registered before starting the bee-estree server.
Deno.addSignalListener("SIGINT", () => {
  spinner.stop();
  // We cannot call server?.stop() here because it's async method...
  Deno.exit(0);
});

function parse(source, sourceType) {
  try {
    return acorn.parse(source, {
      sourceType,
      ecmaVersion: 2022,
    });
  } catch (err) {
    return null;
  }
}

class EstreeServer {
  start() {
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

    let res;
    const reader = this.lines_.getReader();
    try {
      const res = JSON.parse((await reader.read()).value);
      return res.program;
    } catch (err) {
      this.start();
      return null;
    } finally {
      reader.releaseLock();
    }
  }

  async stop() {
    await this.child_.stdin.close();
    await this.child_.status;
  }
}

// Spawn bee-estree in the server mode in order to reduce overhead of process creations.
const server = new EstreeServer();
server.start();

let count = 0;
const fails = [];
const skipped = [];

if (options.progress) {
  spinner.start();
}

if (options.only === 'all' || options.only === 'pass') {
  for await (const entry of Deno.readDir(path.join(args.dir, 'pass'))) {
    if (!entry.isFile) {
      continue;
    }

    count++;

    const test = path.join('pass', entry.name);
    if (EXCLUDES.includes(test)) {
      continue;
    }

    spinner.text = test;

    const source = await Deno.readTextFile(path.join(args.dir, test));
    const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';

    const expected = parse(source, sourceType);
    if (expected === null) {
      skipped.push({ test, reason: 'acorn cannot parse' });
      continue;
    }

    const actual = await server.parse(source, sourceType);
    if (actual === null) {
      fails.push({ test, reason: 'bee-estree cannot parse' });
      continue;
    }

    const diffs = microdiff(actual, expected);
    if (diffs.length > 0) {
      fails.push({ test, reason: 'estree mismatch', diffs });
      continue;
    }

    const testExplicit = path.join('pass-explicit', entry.name);

    spinner.text = testExplicit;

    const sourceExplicit = await Deno.readTextFile(path.join(args.dir, test));

    const expectedExplicit = parse(sourceExplicit, sourceType);
    if (expectedExplicit === null) {
      skipped.push({ test, reason: 'acorn cannot parsr' });
      continue;
    }
    if (microdiff(expected, expectedExplicit).length > 0) {
      skipped.push({ test, reason: 'acorn failed' });
      continue;
    }

    const actualExplicit = await server.parse(sourceExplicit, sourceType);
    if (actualExplicit === null) {
      fails.push({ test, reason: 'bee-estree cannot parse' });
      continue;
    }

    const diffsExplicit = microdiff(actualExplicit, expectedExplicit);
    if (diffsExplicit.length > 0) {
      fails.push({ test, reason: 'estree mismatch', diffs: diffsExplicit });
      continue;
    }

    // passed
  }
}

if (options.only === 'all' || options.only === 'fail') {
  for await (const entry of Deno.readDir(path.join(args.dir, 'fail'))) {
    if (!entry.isFile) {
      continue;
    }

    count++;

    const test = path.join('fail', entry.name);
    if (EXCLUDES.includes(test)) {
      continue;
    }

    spinner.text = test;

    const source = await Deno.readTextFile(path.join(args.dir, test));
    const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';

    const expected = parse(source, sourceType);
    if (expected !== null) {
      skipped.push({ test, reason: 'acorn can parse' });
      continue;
    }

    const actual = await server.parse(source, sourceType);
    if (actual !== null) {
      fails.push({ test, reason: 'bee-estree can parse' });
      continue;
    }

    // passed
  }
}

if (options.only === 'all' || options.only === 'early') {
  for await (const entry of Deno.readDir(path.join(args.dir, 'early'))) {
    if (!entry.isFile) {
      continue;
    }

    count++;

    const test = path.join('early', entry.name);
    if (EXCLUDES.includes(test)) {
      continue;
    }

    spinner.text = test;

    const source = await Deno.readTextFile(path.join(args.dir, test));
    const sourceType = entry.name.endsWith('.module.js') ? 'module' : 'script';

    const expected = parse(source, sourceType);
    if (expected !== null) {
      skipped.push({ test, reason: 'acorn can parse' });
      continue;
    }

    const actual = await server.parse(source, sourceType);
    if (actual !== null) {
      fails.push({ test, reason: 'bee-estree can parse' });
      continue;
    }

    // passed
  }
}

spinner.stop();
await server.stop();

if (options.details) {
  console.log('EXCLUDED:');
  for (const test of EXCLUDES) {
    console.log(`  ${test}`);
  }
  console.log('SKIPPED:');
  for (const { test, reason } of skipped) {
    console.log(`  ${test}: ${reason}`);
  }
  console.log('FAILED:');
  for (const { test, reason, diffs } of fails) {
    console.log(`  ${test}: ${reason}`);
    if (diffs) {
      for (const diff of diffs) {
        const diffPath = diff
              .path
              .map((p) => typeof p === 'number' ? `[${p}]` : `.${p}`)
              .join('');
        console.log(`    ${diff.type}: ${diffPath}`);
      }
    }
  }
}

const passed = count - fails.length - skipped.length;
console.log(
  `${count} tests: ${passed} passed, ${EXCLUDES.length} excluded, ` +
    `${skipped.length} skipped, ${fails.length} failed`);

Deno.exit(fails.length > 0 ? 1 : 0);
