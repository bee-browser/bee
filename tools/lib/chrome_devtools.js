'use strict';

import * as path from 'https://deno.land/std@0.197.0/path/mod.ts';
// npm:puppeteer doesn't work with deno.
// See https://github.com/denoland/deno/issues/17496
//import puppeteer from 'npm:puppeteer@19.8.2';
import { default as puppeteer } from 'https://deno.land/x/puppeteer@16.2.0/mod.ts';
import { RESOURCES_DIR, } from './consts.js';

const DEFAULT_LINUX_EXECUTABLE = '/opt/google/chrome/google-chrome';
const DEFAULT_MACOS_EXECUTABLE = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const DEFAULT_WINDOWS_EXECUTABLE = path.join(
  '%ProgramFiles%', 'Google', 'Chrome', 'Application', 'chrome.exe');

export const DEFAULT_EXECUTABLE =
  Deno.env.get('BEE_TOOLS_DOM_SCRAPER_DEFAULT_EXECUTABLE') ||
  (Deno.build.os === 'linux' && DEFAULT_LINUX_EXECUTABLE) ||
  (Deno.build.os === 'darwin' && DEFAULT_MACOS_EXECUTABLE) ||
  (Deno.build.os === 'windows' && DEFAULT_WINDOWS_EXECUTABLE) ||
  testing.unreachable();

export const DEFAULT_VIEWPORT_SIZE = '1280x720';

const TAKE_SNAPSHOT_SCRIPT = await Deno.readTextFile(
  path.join(RESOURCES_DIR, 'dom_scraper', 'take_snapshot.js'));

const TRANSFER_DATA_SCRIPT = await Deno.readTextFile(
  path.join(RESOURCES_DIR, 'dom_scraper', 'transfer_data.js'));

export async function scrape(url, options) {
  const browser = await launch_puppeteer(options);
  try {
    const page = await browser.newPage();
    await page.setViewport(options.viewport);
    await page.goto(url);
    const json = await scrapeDom(page, options);
    if (options.debug) {
      JSON.parse(json);
    }
    console.log(json);
  } catch (err) {
    console.error(err);
  } finally {
    if (browser.isConnected()) {
      // In Puppeteer 5.5.0, `browser.close()` hangs if the browser has already disconnected.
      await browser.close();
    }
  }
}

export async function tracing(url, options) {
  const browser = await launch_puppeteer(options);
  // tracing.stop() returns a promise which resolves to buffer with trace data, but it does NOT
  // contain meaningful data actually.  We use a temporal file as a trace file.
  const trace = await Deno.makeTempFile();
  try {
    const page = await browser.newPage();
    await page.setViewport(options.viewport);
    await page.tracing.start({
      path: trace,
      screenshots: true,
    });
    await page.goto(url);
    await page.tracing.stop();
    return await Deno.readTextFile(trace);
  } catch (err) {
    console.error(err);
  } finally {
    await Deno.remove(trace);
    if (browser.isConnected()) {
      // In Puppeteer 5.5.0, `browser.close()` hangs if the browser has already disconnected.
      await browser.close();
    }
  }
}

async function launch_puppeteer(options) {
  let opts = {
    executablePath: options.executable || DEFAULT_EXECUTABLE,
    devtools: options.debug || false,
    dumpio: options.logging || false,
    args: [],
  };

  if (options.noSandbox) {
    opts.args.push('--no-sandbox');
    opts.args.push('--disable-setuid-sandbox');
  }

  return await puppeteer.launch(opts);
}

async function scrapeDom(page, options) {
  if (options.cdp) {
    return await scrapeDomUsingCdp(page, options);
  }
  return await scrapeDomUsingScript(page, options);
}

// NOTE
// ----
// We cannot transfer scraped data at once due to some sort of a size limit on the communication
// channel between the Puppeteer and the Chrome.
async function scrapeDomUsingScript(page, options) {
  await takeSnapshot(page, options);

  let segments = [];
  for (;;) {
    const segment = await transferData(page, options);
    if (segment === undefined) {
      break;
    }
    segments.push(segment);
  }

  return segments.join('');
}

async function takeSnapshot(page, options) {
  await page.evaluate((script, options) => {
    const takeSnapshot = new Function('$OPTIONS', script);
    return takeSnapshot(options);
  }, TAKE_SNAPSHOT_SCRIPT, { debug: options.debug });
}

async function transferData(page, options) {
  return await page.evaluate((script, options) => {
    const transferData = new Function('$OPTIONS', script);
    return transferData(options);
  }, TRANSFER_DATA_SCRIPT, { debug: options.debug });
}

// experimental scraper using CDP.

// NOTE
// ----
// CSS.getComputedStyleForNode returns used values for some style properties.
//
// Changing the 'display' to 'none' might work as well as the case of Window.getComputedStyle,
// but we've never tried that yet.
async function scrapeDomUsingCdp(page, options) {
  const client = await page.target().createCDPSession();
  await client.send('DOM.enable');
  await client.send('CSS.enable');
  let { root } = await client.send('DOM.getDocument', { depth: -1, pierce: false });
  await collectStylesUsingCdp(client, root);
  // TODO: collect resources like images as data URLs
  // TODO: convert nodes into our data structures
  return JSON.Stringify({
    document: {
      url: page.url(),
      title: await page.title(),
      root,
    },
    resources: {},  // TODO
  });
}

// TODO
// ----
// The current implementation is very slow...
// Sending a request for each node results a lot of overhead due to IPC communication costs.
async function collectStylesUsingCdp(client, node) {
  if (node.nodeType === 1) {  // ELEMENT_NODE
    const { computedStyle } = await client.send(
      'CSS.getComputedStyleForNode', { nodeId: node.nodeId });
    node.computedStyle = computedStyle;
  }
  for (let i = 0; i < node.childNodeCount; ++i) {
    let child = node.children[i];
    await collectStylesUsingCdp(client, child);
  }
}
