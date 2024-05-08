'use strict';

import * as path from 'https://deno.land/std@0.224.0/path/mod.ts';
import * as base64 from 'https://deno.land/x/base64@v0.2.1/mod.ts';
import { parseCommand } from '../lib/cli.js';
import * as top_sites from '../lib/top_sites.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DEFAULT_LIMIT = 100;

const DOC = `
Show a list of popular web sites.

Usage:
  ${PROGNAME} <ranking> [<limit>]
  ${PROGNAME} -h | --help

Arguments:
  <ranking>
    Ranking site to use.

    The following ranking sites are supported:

      ${top_sites.RANKINGS.join(', ')}

  <limit>  [default: ${DEFAULT_LIMIT}]
    The number of URLs to be show.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
  conv: async (name, value) => {
    switch (name) {
      case '<ranking>':
        if (!top_sites.RANKINGS.includes(value)) {
          throw new Error(`Not supported: ${value}`);
        }
        return value;
      case '<limit>':
        if (value === null) {
          return DEFAULT_LIMIT;
        }
        const limit = parseInt(value);
        if (isNaN(limit)) {
          throw new Error(`Not a number: ${value}`);
        }
        return limit;
      default:
        return value;
    }
  },
});

Deno.exit(await top_sites.show(args.ranking, args.limit));
