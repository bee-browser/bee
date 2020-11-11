'use strict';

const fs = require('fs');
const path = require('path');
const puppeteer = require('puppeteer');
const consts = require('../consts');

async function run(url, options) {
  let opt = {
    devtools: options.debug,
    dumpio: options.logging,
  };
  if (!options.sandbox) {
    opt.args = ['--no-sandbox', '--disable-setuid-sandbox'];
  }
  const browser = await puppeteer.launch(opt);
  try {
    const page = await browser.newPage();
    await page.setViewport(options.viewport);
    await page.goto(url);
    const script = fs.readFileSync(
      path.resolve(consts.RCDIR, 'lms', 'html.puppeteer.js'),
      { encoding: 'utf8' });
    const messages = await page.evaluate((script, options) => {
      const func = new Function('$OPTIONS', script);
      return func(options);
    }, script, { debug: options.debug });
    if (options.json) {
      console.log(JSON.stringify(messages));
    } else {
      messages.forEach((msg) => console.log(JSON.stringify(msg)));
    }
  } catch(e) {
    console.error(e);
  } finally {
    await browser.close();
  }
}

// exports

module.exports.run = run;
