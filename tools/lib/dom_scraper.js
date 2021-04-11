'use strict';

import {
  path,
  puppeteer,
} from '../deps.js';

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

export async function scrape(url, options) {
  let opts = {
    executablePath: options.executable || DEFAULT_EXECUTABLE,
    devtools: options.debug || false,
    dumpio: options.logging || false,
  };

  if (!options.sandbox) {
    opts.args = ['--no-sandbox', '--disable-setuid-sandbox'];
  }

  const browser = await puppeteer.launch(opts);
  try {
    const page = await browser.newPage();
    await page.setViewport(options.viewport);
    await page.goto(url);
    const dom = await scrapeDom(page, options);
    console.log(JSON.stringify(dom));
  } catch (err) {
    console.error(err);
  } finally {
    if (browser.isConnected()) {
      // In Puppeteer 5.5.0, `browser.close()` hangs if the browser has already disconnected.
      await browser.close();
    }
  }
}

async function scrapeDom(page, options) {
  if (options.cdp) {
    return await scrapeDomUsingCdp(page, options);
  }
  return await scrapeDomUsingScript(page, options);
}

async function scrapeDomUsingScript(page, options) {
  const script = Deno.readTextFileSync(path.join(RESOURCES_DIR, 'dom_scraper.js'));
  const dom = await page.evaluate((script, options) => {
    const func = new Function('$OPTIONS', script);
    return func(options);
  }, script, { debug: options.debug });
  return dom;
}

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
  return {
    document: {
      url: page.url(),
      title: await page.title(),
      root,
    },
    resources: {},  // TODO
  };
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
