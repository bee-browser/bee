// TODO(test): Use bins/test262.
//
// Currently, there are many unimplemented features that cause panics in jsparser/jsruntime.
// As a result, bjs may crash in many test cases.  Such test cases must be evaluated in separate
// processes.
//
// This script is slow because this script performs tests in separate bjs processes.  Performing
// all test cases takes about nearly 10 minutes.

'use strict';

import { unreachable } from '@std/assert';
import * as log from '@std/log';
import * as path from '@std/path';
import ora from 'ora';
import TestStream from 'test262-stream';
import { parseCommand } from '../../../tools/lib/cli.js';
import { PROJ_DIR, VENDOR_DIR } from '../../../tools/lib/consts.js';
import { setup } from '../../../tools/lib/log.js';

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

  --debug

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

if (import.meta.main) {
  const { options, args } = await parseCommand({
    doc: DOC,
  });

  options.profile ||= 'release';
  options.test262Dir ||= DEFAULT_TEST262_DIR;
  options.timeout ||= DEFAULT_TIMEOUT;

  if (args.tests.length === 0) {
    args.tests.push(DEFAULT_TESTS);
  }

  Deno.exit(await main(options, args));
}

async function main(options, args) {
  if (options.debug) {
    setup(PROGNAME, 'DEBUG');
  } else {
    setup(PROGNAME, 'INFO');
  }

  const spinner = ora({ spinner: 'line' });

  // The signal handler must be registered before starting the estree server.
  Deno.addSignalListener('SIGINT', () => {
    spinner.stop();
    // We cannot call server?.stop() here because it's async method...
    Deno.exit(0);
  });

  const stream = new TestStream(options.test262Dir, { paths: args.tests });
  stream.on('error', (err) => {
    log.error(`Something went wrong: ${err}`);
    spinner.stop();
  });

  if (options.progress) {
    spinner.start();
  }

  const results = {
    count: 0,
    skipped: [],
    aborted: [],
    timedout: [],
    failed: [],
  };

  // Promises of `cmd.output()` are put into the following queue in order to improve throughput.
  // There are some test cases that are timed out.
  const jobs = [];
  const NUM_JOBS = navigator.hardwareConcurrency;

  for await (const test of stream) {
    results.count++;

    spinner.text = test.file;

    const bjs = spawnBjs(options);
    const output = runTest(bjs, test);
    jobs.push({ test, output });
    if (jobs.length === NUM_JOBS) {
      await handleJobs(jobs, results);
    }
  }

  if (jobs.length > 0) {
    await handleJobs(jobs, results);
  }

  spinner.stop();

  const passed = results.count - results.skipped.length - results.aborted.length - results.timedout.length - results.failed.length;
  log.info(
    `${results.count} tests: ${passed} passed, ` +
      `${results.skipped.length} skipped, ${results.aborted.length} aborted, ` +
      `${results.timedout.length} timed-out, ${results.failed.length} failed`);

  return passed === results.count - results.skipped.length ? 0 : 1;
}

function spawnBjs(options) {
  const commandOptions = {
    stdin: 'piped',
    stdout: 'piped',
    stderr: 'null',
    env: {
      RUST_LOG: 'off',
    },
  };

  switch (options.profile) {
  case 'release':
    commandOptions.args = [
      options.timeout,
      path.join(PROJ_DIR, 'target', 'release', 'bjs'), 'test262',
    ];
    break;
  case 'debug':
    commandOptions.args = [
      options.timeout,
      path.join(PROJ_DIR, 'target', 'debug', 'bjs'), 'test262',
    ];
    break;
  case 'coverage':
    console.error("NOT SUPPORTED:", options.profile);
    unreachable();
  default:
    unreachable();
  }
  return new Deno.Command('timeout', commandOptions).spawn();
}

async function runTest(bjs, test) {
  const encoder = new TextEncoder();
  const writer = bjs.stdin.getWriter();
  await writer.write(encoder.encode(test.contents));
  writer.releaseLock();
  await bjs.stdin.close();
  return await bjs.output();
}

async function handleJobs(jobs, results) {
  for (const job of jobs) {
    await handleJob(job, results);
  }
  jobs.length = 0;
}

async function handleJob(job, results) {
  try {
    const { code, stdout } = await job.output;
    handleTestResult(job.test, code, stdout, results);
  } catch (error) {
    results.aborted.push({ test: job.test, error });
  }
}

function handleTestResult(test, code, stdout, results) {
  switch (code) {
  case 0:
    // finished
    break;
  case 124:
    results.timedout.push(test);
    return;
  default:
    results.aborted.push({ test, code });
    return;
  }

  const json = new TextDecoder().decode(stdout);
  const result = JSON.parse(json);
  switch (result.type) {
  case 'pass':
    if (test.attrs?.negative) {
      results.failed.push({ test, reason: 'negative mismatched' });
    }
    break;
  case 'parse-error':
    if (test.attrs?.negative?.phase !== 'parse') {
      results.failed.push({ test, reason: 'negative mismatched' });
    }
    break;
  case 'runtime-error':
    resutls.failed.push({ test, reason: 'runtime-error', value: result.data });
    // TODO
    break;
  default:
    log.error(`invalid result.type: ${result.type}`);
    break;
  }
}
