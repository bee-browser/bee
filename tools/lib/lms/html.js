'use strict';

const fs = require('fs');
const path = require('path');
const puppeteer = require('puppeteer');
const consts = require('../consts');

async function run(url, options) {
  let opt = { headless: options.headless };
  if (!options.sandbox) {
    opt.args = ['--no-sandbox', '--disable-setuid-sandbox'];
  }
  if (options.logging) {
    opt.dumpio = true;
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
      const func = new Function('options', script);
      return func(options);
    }, script, {});
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
