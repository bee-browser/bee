'use strict';

// TODO(test): Use bins/test262.
//
// Currently, there are many unimplemented features that cause panics in jsparser/jsruntime.
// As a result, bjs may crash in many test cases.  Such test cases must be evaluated in separate
// processes.
//
// This script is slow because this script performs tests in separate bjs processes.  Performing
// all test cases takes about nearly 10 minutes.

import { unreachable } from '@std/assert';
import * as path from '@std/path';
import ora from 'ora';
import TestStream from 'test262-stream';
import { parseCommand } from '../../../tools/lib/cli.js';
import { PROJ_DIR, VENDOR_DIR } from '../../../tools/lib/consts.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const DEFAULT_TEST262_DIR = path.join(VENDOR_DIR, 'src', 'tc39', 'test262');
const DEFAULT_TIMEOUT = '5s';
const DEFAULT_TESTS = 'test';

const DOC = `
Usage:
  ${PROGNAME} [options] [<tests>...]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --details
    Show the details of failed tests.

  --profile=(release | debug | coverage) [default: release]
    Choice one of the following profiles:
      release: Use the release build
      debug: Use the debug build
      coverage: Use cargo-llvm-cov

  --test262-dir <test262-dir> [default: ${DEFAULT_TEST262_DIR}]
    Path to tc39/test262.

  --timeout <timeout> [default: ${DEFAULT_TIMEOUT}]
    Duration passed to 'timeout' commands.

Arguments:
  <tests> [default: ${DEFAULT_TESTS}]
    Tests to perform.
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

options.profile ||= 'release';
options.test262Dir ||= DEFAULT_TEST262_DIR;
options.timeout ||= DEFAULT_TIMEOUT;

if (args.tests.length === 0) {
  args.tests.push(DEFAULT_TESTS);
}

// The signal handler must be registered before starting the estree server.
Deno.addSignalListener('SIGINT', () => {
  spinner?.stop();
  // We cannot call server?.stop() here because it's async method...
  Deno.exit(0);
});

const spinner = ora({ spinner: 'line' });

const stream = new TestStream(options.test262Dir, { paths: args.tests });
stream.on('error', (err) => console.error('Something went wrong:', err));
if (options.progress) {
  spinner.start();
}

let count = 0;
const skipped = [];
const aborted = [];
const timedout = [];
const failed = [];

// Promises of `cmd.output()` are put into the following queue in order to improve throughput.
// There are some test cases that are timed out.
const jobs = [];
const NUM_JOBS = navigator.hardwareConcurrency;

async function handleJobs() {
  for (const job of jobs) {
    await handleJob(job);
  }
  jobs.length = 0;
}

async function handleJob(job) {
  try {
    const { code, stdout } = await job.output;
    handleTestResult(job.test, code, stdout);
  } catch (error) {
    aborted.push({ test, error });
  }
}

function handleTestResult(test, code, stdout) {
  switch (code) {
  case 0:
    // finished
    break;
  case 124:
    timedout.push(test);
    return;
  default:
    aborted.push({ test, code });
    return;
  }

  const json = new TextDecoder().decode(stdout);
  const result = JSON.parse(json);
  switch (result.type) {
  case 'pass':
    if (test.attrs?.negative) {
      failed.push({ test, reason: 'negative mismatched' });
    }
    break;
  case 'parse-error':
    if (test.attrs?.negative?.phase !== 'parse') {
      failed.push({ test, reason: 'negative mismatched' });
    }
    break;
  case 'runtime-error':
    failed.push({ test, reason: 'runtime-error', value: result.data });
    // TODO
    break;
  default:
    console.error('invalid result.type:', result.type);
    break;
  }
}

for await (const test of stream) {
  count++;

  test.toString = function () {
    let s = this.file;
    s += this.attrs.flags.module ? '#module' : '#script';
    if (this.scenario === 'strict mode') {
      s += '#strict';
    }
    if (this.attrs.features) {
      s += `#${this.attrs.features.join('#')}`;
    }
    return s;
  };

  spinner.text = test.file;

  const source = test.contents;
  const sourceType = test.attrs.flags.module ? 'module' : 'script';

  const bjs = path.join(PROJ_DIR, 'target', options.profile, 'bjs');
  const cmd = new Deno.Command('timeout', {
    args: [options.timeout, bjs, 'test262', test.file],
    stdin: 'null',
    stdout: 'piped',
    stderr: 'null',
    env: {
      RUST_LOG: 'off',
    },
  });

  jobs.push({ test, output: cmd.output() });
  if (jobs.length === NUM_JOBS) {
    await handleJobs();
  }
}

if (jobs.length > 0) {
  await handleJobs();
}

spinner.stop();

const passed = count - skipped.length - aborted.length - timedout.length - failed.length;
console.log(
  `${count} tests: ${passed} passed, ` +
    `${skipped.length} skipped, ${aborted.length} aborted, ` +
    `${timedout.length} timed-out, ${failed.length} failed`);

Deno.exit(failed.length === 0 ? 0 : 1);
