'use strict';

import * as path from 'https://deno.land/std@0.209.0/path/mod.ts';

import ora from 'npm:ora@7.0.1';

import { parseCommand } from '../../../tools/lib/cli.js';
import { showDiffs } from './test262_helper.js';

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

  --with-debug-build
    Test with the debug build binary.
    Testing with this option is better but 8x slower.

    Some kind of runtime errors such as arithmetic underflow cannot be
    detected in the release build.
`.trim();

const { options } = await parseCommand({
  doc: DOC,
});

// We use a worker because the spinner sometimes gets stuck due to a large EStree.
// Computation of JSON5.parse() and microdiff() takes a long time.
const worker = new Worker(
  new URL("./cdnjs_worker.js", import.meta.url).href, { type: 'module' });

const spinner = ora({ spinner: 'line' });

// The signal handler must be registered before starting the estree server.
Deno.addSignalListener("SIGINT", () => {
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
const libs =
    (await getJson('https://api.cdnjs.com/libraries?fields=fileType'))
      .results
      .filter((lib) => lib.latest !== null && lib.fileType === 'js');

const skipped = [];
const fails = [];

for (let i = 0; i < libs.length; ++i) {
  const url = libs[i].latest;
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
  worker.postMessage(url);
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
console.log(`${libs.length} urls: ${passed} passed, ${skipped.length} skipped, ${fails.length} failed`);

Deno.exit(fails.length === 0 ? 0 : 1);
