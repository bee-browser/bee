'use strict';

import * as path from '@std/path';
import ora from 'ora';
import { parseCommand } from '../../../tools/lib/cli.js';
import { showDiffs } from './helpers.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  --progress
    Show progress.

  --details
    Show the details of failed tests.

  --mode=(release | debug | coverage) [default: release]
    Choice one of the following modes:
      release: Use the release build
      debug: Use the debug build
      coverage: Use cargo-llvm-cov
`;

const { options } = await parseCommand({
  doc: DOC,
});

options.mode ||= 'release';

const EXCLUDES = [
  // The following libraries consume a lot of memory and `deno` stops due to out of memory.
  // In addition, "buffer exceeds maximum length" error occurs in validate.js.
  'webkit.js',
  'unicorn.js',
  'mnist',
  // TODO: B.3.2 Block-Level Function Declarations Web Legacy Compatibility Semantics
  'merger',
];

// We use a worker because the spinner sometimes gets stuck due to a large EStree.
// Computation of JSON5.parse() and microdiff() takes a long time.
const worker = new Worker(
  new URL('./cdnjs_worker.js', import.meta.url).href,
  { type: 'module' },
);

const spinner = ora({ spinner: 'line' });

// The signal handler must be registered before starting the estree server.
Deno.addSignalListener('SIGINT', () => {
  spinner.stop();
  Deno.exit(0);
});

async function getJson(url) {
  return await (await fetch(url)).json();
}

if (options.progress) {
  spinner.start();
}

spinner.text = 'loading libraries from cdnjs.com...';
const libs = (await getJson('https://api.cdnjs.com/libraries?fields=fileType'))
  .results
  .filter((lib) => lib.latest !== null && lib.fileType === 'js');

const skipped = [];
const fails = [];

for (let i = 0; i < libs.length; ++i) {
  const name = libs[i].name;
  const url = libs[i].latest;
  if (EXCLUDES.includes(name)) {
    skipped.push({ url, reason: 'excluded' });
    continue;
  }
  const promise = new Promise((resolve, reject) => {
    worker.onmessage = ({ data }) => {
      switch (data.type) {
        case 'progress':
          spinner.text = `${i}/${libs.length} ${url}: ${data.message}`;
          break;
        case 'pass':
          resolve();
          break;
        case 'skip':
          skipped.push({ url, reason: data.reason });
          resolve();
          break;
        case 'fail':
          spinner.fail(`${url}: ${data.reason}`);
          if (options.progress) {
            spinner.start();
          }
          skipped.push({ url, reason: data.reason, diffs: data.diffs });
          resolve();
          break;
      }
    };
    worker.onerror = (event) => reject(event);
    worker.onmessageerror = (event) => reject(event);
  });
  worker.postMessage({ name, url, options });
  try {
    await promise;
  } catch {
    spinner.stop();
    Deno.exit(1);
  }
}

spinner.stop();

if (options.details) {
  console.log('SKIPPED:');
  for (const { url, reason } of skipped) {
    console.log(`  ${url}: ${reason}`);
  }
  console.log('FAILED:');
  for (const { url, reason, diffs } of fails) {
    console.log(`  ${url}: ${reason}`);
    if (diffs) {
      showDiffs(diffs, '    ');
    }
  }
}

const passed = libs.length - fails.length - skipped.length;
console.log(
  `${libs.length} urls: ${passed} passed, ${skipped.length} skipped, ${fails.length} failed`,
);

Deno.exit(fails.length === 0 ? 0 : 1);
