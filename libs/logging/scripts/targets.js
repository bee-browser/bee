'use strict';

import * as fs from 'https://deno.land/std@0.211.0/fs/mod.ts';
import * as path from 'https://deno.land/std@0.211.0/path/mod.ts';
import * as toml from 'https://deno.land/std@0.211.0/toml/mod.ts';
import * as yaml from 'https://deno.land/std@0.211.0/yaml/mod.ts';

import { parseCommand } from '../../../tools/lib/cli.js';
import { PROJ_DIR } from '../../../tools/lib/consts.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC =`
Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help
`;

const { options } = await parseCommand({
  doc: DOC,
});

const manifest = toml.parse(await Deno.readTextFile(path.join(PROJ_DIR, 'Cargo.toml')));
const patterns = manifest.workspace.members.map((glob) => {
  return path.globToRegExp(path.joinGlobs(['**', glob, 'logging.yaml']));
});

const targets = [];
for await (const entry of fs.walk(PROJ_DIR, { match: patterns })) {
  const logging = yaml.parse(await Deno.readTextFile(entry.path));
  for (const target of logging.targets) {
    targets.push({
      name: target,
      symbol: target.split('::').slice(1).map((s) => s.toUpperCase()).join('_'),
      loggerPath: path.join(path.dirname(entry.path), 'src', ...target.split('::').slice(2), 'logger.rs'),
    });
  }
}

targets.push({
  name: 'bee::tests',
  symbol: 'TESTS',
  loggerPath: null,
});

console.log(JSON.stringify({ targets }));
