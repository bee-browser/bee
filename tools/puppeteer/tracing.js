// Input environment variables:
//
//   PAGE_URL
//   VIEWPORT_WIDTH
//   VIEWPORT_HEIGHT
//   LOGGING
//

'use strict';

const fs = require('node:fs/promises');
const puppeteer = require('puppeteer');

const PAGE_URL = process.env.PAGE_URL;
const VIEWPORT_WIDTH = parseInt(process.env.VIEWPORT_WIDTH);
const VIEWPORT_HEIGHT = parseInt(process.env.VIEWPORT_HEIGHT);
const LOGGING = process.env.LOGGING === '1';

// tracing.stop() returns a promise which resolves to buffer with trace data,
// but it does NOT contain meaningful data actually.
// We use a temporal file as a trace file.
const TRACE_FILE = './trace.json';

(async () => {
  const browser = await puppeteer.launch({ dumpio: LOGGING });
  const page = await browser.newPage();
  await page.setViewport({
    width: VIEWPORT_WIDTH,
    height: VIEWPORT_HEIGHT,
  });
  await page.tracing.start({
    path: TRACE_FILE,
    screenshots: true,
  });
  await page.goto(PAGE_URL);
  await page.tracing.stop();
  await browser.close();
  // We don't need to remove the trace file.
  // Because this script is evaluated inside a Docker container.
  const json = await readText(TRACE_FILE);
  console.log(json);
})();

async function readText(filepath) {
  return await fs.readFile(filepath, { encoding: 'utf8' });
}
