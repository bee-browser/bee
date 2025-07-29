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
    Show debug logs.

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

Description:
  This script performs the specified test cases in tc39/test262, outputs test results to STDOUT and
  shows the summary of the test results.

  The test results are output in the CTRF (Common Test Report Format).
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

  const tests = [];

  // Promises of `cmd.output()` are put into the following queue in order to improve throughput.
  // There are some test cases that are timed out.
  const jobs = [];
  const NUM_JOBS = navigator.hardwareConcurrency;

  for await (const test of stream) {
    spinner.text = test.file;
    const bjs = spawnBjs(options);
    const output = runTest(bjs, test);
    jobs.push({ test, output });
    if (jobs.length === NUM_JOBS) {
      await handleJobs(jobs, tests);
    }
  }

  if (jobs.length > 0) {
    await handleJobs(jobs, tests);
  }

  spinner.stop();

  console.log(JSON.stringify({
    reportFormat: 'CTRF',
    specVersion: '0.0.0',
    results: {
      tool: {
        name: 'bee-browser/bee;bins/bjs/scripts/test262.js',
      },
      summary: ctrfSummary(tests),
      tests,
    },
  }));

  return showSummary(tests) ? 0 : 1;
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
        path.join(PROJ_DIR, 'target', 'release', 'bjs'),
        'test262',
      ];
      break;
    case 'debug':
      commandOptions.args = [
        options.timeout,
        path.join(PROJ_DIR, 'target', 'debug', 'bjs'),
        'test262',
      ];
      break;
    case 'coverage':
      console.error('NOT SUPPORTED:', options.profile);
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

async function handleJobs(jobs, tests) {
  for (const job of jobs) {
    await handleJob(job, tests);
  }
  jobs.length = 0;
}

async function handleJob(job, tests) {
  try {
    const { code, stdout } = await job.output;
    handleTestResult(job.test, code, stdout, tests);
  } catch (error) {
    tests.push({
      name: job.test.file,
      status: 'other',
      duration: 0,
      start: 0,
      stop: 0,
      rawStatus: 'aborted',
      extra: {
        metadata: job.test.attrs,
        error: error,
      },
    });
  }
}

function handleTestResult(test, code, stdout, tests) {
  switch (code) {
    case 0:
      // finished
      break;
    case 124:
      tests.push({
        name: test.file,
        status: 'other',
        duration: 0,
        start: 0,
        stop: 0,
        rawStatus: 'timed-out',
        extra: {
          metadata: test.attrs,
        },
      });
      return;
    default:
      tests.push({
        name: test.file,
        status: 'other',
        duration: 0,
        start: 0,
        stop: 0,
        rawStatus: 'aborted',
        extra: {
          metadata: test.attrs,
          exitCode: code,
        },
      });
      return;
  }

  let start;
  const lines = new TextDecoder().decode(stdout).split('\n');
  for (const json of lines) {
    const event = JSON.parse(json);
    switch (event.type) {
      case 'start':
        start = event.data.timestamp;
        break;
      case 'pass':
        if (test.attrs?.negative) {
          tests.push({
            name: test.file,
            status: 'failed',
            duration: event.data.timestamp - start,
            start,
            stop: event.data.timestamp,
            message: 'negative mismatched',
            rawStatus: 'failed',
            extra: {
              metadata: test.attrs,
            },
          });
          return;
        }
        tests.push({
          name: test.file,
          status: 'passed',
          duration: event.data.timestamp - start,
          start,
          stop: event.data.timestamp,
          rawStatus: 'passed',
          extra: {
            metadata: test.attrs,
          },
        });
        return;
      case 'parse-error':
        if (test.attrs?.negative?.phase !== 'parse') {
          tests.push({
            name: test.file,
            status: 'failed',
            duration: event.data.timestamp - start,
            start,
            stop: event.data.timestamp,
            message: 'negative mismatched',
            rawStatus: 'failed',
            extra: {
              metadata: test.attrs,
            },
          });
          return;
        }
        tests.push({
          name: test.file,
          status: 'passed',
          duration: event.data.timestamp - start,
          start,
          stop: event.data.timestamp,
          rawStatus: 'passed',
          extra: {
            metadata: test.attrs,
          },
        });
        return;
      case 'runtime-error':
        // TODO: check error
        tests.push({
          name: test.file,
          status: 'failed',
          duration: event.data.timestamp - start,
          start,
          stop: event.data.timestamp,
          message: 'runtime-error',
          rawStatus: 'failed',
          extra: {
            metadata: test.attrs,
          },
        });
        return;
      case 'print':
        // TODO
        break;
      default:
        log.error(`invalid result.type: ${result.type}`);
        // ignore
        break;
    }
  }
}

function ctrfSummary(tests) {
  const summary = {
    tests: tests.length,
    passed: 0,
    failed: 0,
    pending: 0,
    skipped: 0,
    other: 0,
  };

  for (const test of tests) {
    switch (test.status) {
      case 'passed':
        summary.passed++;
        break;
      case 'failed':
        summary.failed++;
        break;
      case 'pending':
        summary.pending++;
        break;
      case 'skipped':
        summary.skipped++;
        break;
      case 'other':
        summary.other++;
        break;
    }
  }

  return summary;
}

function showSummary(tests) {
  let passed = 0;
  let skipped = 0;
  let aborted = 0;
  let timedout = 0;
  let failed = 0;

  for (const test of tests) {
    switch (test.rawStatus) {
      case 'passed':
        passed++;
        break;
      case 'skipped':
        skipped++;
        break;
      case 'aborted':
        aborted++;
        break;
      case 'timed-out':
        timedout++;
        break;
      case 'failed':
        failed++;
        break;
    }
  }

  log.info(
    `${tests.length} tests: ${passed} passed, ${skipped} skipped, ${aborted} aborted, ` +
      `${timedout} timed-out, ${failed} failed`,
  );

  return passed === tests.length;
}
