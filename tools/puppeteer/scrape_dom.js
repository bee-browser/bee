// Input environment variables:
//
//   PAGE_URL
//   VIEWPORT_WIDTH
//   VIEWPORT_HEIGHT
//   LOGGING
//   USE_CDP
//   TODO: DEBUG
//
// Input files:
//
//   /scrape_dom/<page-file> (optional)
//   /scrape_dom/take_snapshot.js
//   /scrape_dom/transfer_data.js
//

'use strict';

const fs = require('node:fs');
const puppeteer = require('puppeteer');

const PAGE_URL = process.env.PAGE_URL;
const VIEWPORT_WIDTH = parseInt(process.env.VIEWPORT_WIDTH);
const VIEWPORT_HEIGHT = parseInt(process.env.VIEWPORT_HEIGHT);
const LOGGING = process.env.LOGGING === '1';
const USE_CDP = process.env.USE_CDP === '1';
const DEBUG = process.env.DEBUG === '1';

const TAKE_SNAPSHOT_SCRIPT = readText('/scrape_dom/take_snapshot.js');
const TRANSFER_DATA_SCRIPT = readText('/scrape_dom/transfer_data.js');

(async () => {
  const browser = await puppeteer.launch({ dumpio: LOGGING });
  const page = await browser.newPage();
  await page.setViewport({
    width: VIEWPORT_WIDTH,
    height: VIEWPORT_HEIGHT,
  });
  await page.goto(PAGE_URL);
  const json = await scrapeDom(page, {
    cdp: USE_CDP,
    debug: DEBUG,
  });
  console.log(json);
  await browser.close();
})();

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
  await page.evaluate(
    (script, options) => {
      const takeSnapshot = new Function('$OPTIONS', script);
      return takeSnapshot(options);
    },
    TAKE_SNAPSHOT_SCRIPT,
    { debug: options.debug },
  );
}

async function transferData(page, options) {
  return await page.evaluate(
    (script, options) => {
      const transferData = new Function('$OPTIONS', script);
      return transferData(options);
    },
    TRANSFER_DATA_SCRIPT,
    { debug: options.debug },
  );
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
    resources: {}, // TODO
  });
}

// TODO
// ----
// The current implementation is very slow...
// Sending a request for each node results a lot of overhead due to IPC communication costs.
async function collectStylesUsingCdp(client, node) {
  if (node.nodeType === 1) { // ELEMENT_NODE
    const { computedStyle } = await client.send(
      'CSS.getComputedStyleForNode',
      { nodeId: node.nodeId },
    );
    node.computedStyle = computedStyle;
  }
  for (let i = 0; i < node.childNodeCount; ++i) {
    let child = node.children[i];
    await collectStylesUsingCdp(client, child);
  }
}

function readText(filepath) {
  return fs.readFileSync(filepath, { encoding: 'utf8' });
}
