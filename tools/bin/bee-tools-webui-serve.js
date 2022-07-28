'use strict';

import * as path from 'std/path/mod.ts';
import { parseCommand, runCommand } from '../lib/cli.js';
import * as webui from '../lib/webui.js';

const DIRNAME = path.dirname(path.fromFileUrl(import.meta.url));

const DEFAULT_PORT = 3000;

const DOC = `
Start Web UI server.

Usage:
  bee-tools-webui-serve [options]
  bee-tools-webui-serve -h | --help

Options:
  -p, --port=<port>  [default: ${DEFAULT_PORT}]
    Port to be listened.

  --debug-build
    Use debug-build binaries.
`.trim();

const { options, args, } = await parseCommand({
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