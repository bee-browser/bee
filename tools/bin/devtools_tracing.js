'use strict';

import * as path from 'https://deno.land/std@0.210.0/path/mod.ts';
import * as base64 from 'https://deno.land/x/base64@v0.2.1/mod.ts';
import { parseCommand } from '../lib/cli.js';
import * as chrome_devtools from '../lib/chrome_devtools.js';

const PROGNAME = path.basename(Deno.mainModule);

const DOC = `
Get a DevTools performance trace for a page load.

Usage:
  ${PROGNAME} [options] [<uri>]
  ${PROGNAME} -h | --help

Options:
  --debug
    Run Chrome without the headless mode, and audo-open devtools for debugging.

  --no-sandbox
    Run Chrome without the sandbox.

  --logging
    Enable logging.

  --executable=<path>  [default: ${chrome_devtools.DEFAULT_EXECUTABLE}]
    Path to an executable of Chrome.

  --viewport=<size>  [default: ${chrome_devtools.DEFAULT_VIEWPORT_SIZE}]
    Viewport size in pixels in the form "<width>x<height>".

Arguments:
  <uri>
    URI or path to a web page to be scraped.

    Read a HTML content from STDIN and convert it into a data URI if the <uri> is not specified.

Environment Variables:
  BEE_TOOLS_DOM_SCRAPER_DEFAULT_EXECUTABLE
    Path to an executable of Chrome.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
  conv: async (name, value) => {
    switch (name) {
    case '--viewport':
      const [width, height] = value.split('x', 2);
      return { width: parseInt(width), height: parseInt(height) };
    case '<uri>':
      if (value) {
        try {
          new URL(value);
          return value;
        } catch (err) {
          return path.toFileUrl(path.resolve(value));
        }
      } else {
        const html = base64.fromUint8Array(await Deno.readAll(Deno.stdin));
        return `data:text/html;charset=utf-8;base64,${html}`;
      }
    default:
      return value;
    }
  },
});

const json = await chrome_devtools.tracing(args.uri, options);
console.log(json);
