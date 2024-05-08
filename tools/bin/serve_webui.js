'use strict';

import * as path from 'https://deno.land/std@0.224.0/path/mod.ts';
import { parseCommand, runCommand } from '../lib/cli.js';
import * as webui from '../lib/webui.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DEFAULT_PORT = 3000;

const DOC = `
Start Web UI server.

Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  -p, --port=<port>  [default: ${DEFAULT_PORT}]
    Port to be listened.

  --debug-build
    Use debug-build binaries.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
  conv: (name, value) => {
    switch (name) {
      case '--port':
        return parseInt(value);
    }
    return value;
  },
});

await webui.serve({
  root: path.join(DIRNAME, '..', '..', 'webui', 'assets'),
  ...options,
});
