'use strict';

import { EOL } from '@std/fs';
import * as log from '@std/log';
import * as path from '@std/path';
import { parseCommand } from '../../../../../tools/lib/cli.js';
import { setup } from '../../../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} <README.md> <imp.json>
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

if (options.debug) {
  setup(PROGNAME, 'DEBUG');
} else {
  setup(PROGNAME, 'INFO');
}

Deno.exit(await main(args, options));

async function main(args, options) {
  const ECMA262_SPEC_URL_BASE = 'https://tc39.es/ecma262/#';

  const readme = await Deno.readTextFile(args.readmeMd);
  const impJson = await Deno.readTextFile(args.impJson);
  const data = JSON.parse(impJson);

  const lines = [];
  for (const line of readme.split(EOL)) {
    if (!line.startsWith('* [')) {
      lines.push(line);
      continue;
    }
    const parts = line.split(ECMA262_SPEC_URL_BASE);
    const id = parts[1].substring(0, parts[1].length - 1); // remove the last ')'
    // TODO(feat): constructor
    let func = data.constructorProperties.functions.find((func) => func.id === id);
    if (func) {
      lines.push(`* [x] [${func.signature.name}](${ECMA262_SPEC_URL_BASE}${func.id})`);
      continue;
    }
    func = data.prototypeProperties.functions.find((func) => func.id === id);
    if (func) {
      lines.push(`* [x] [${func.signature.name}](${ECMA262_SPEC_URL_BASE}${func.id})`);
      continue;
    }
    lines.push(line.replace('[x]', '[ ]'));
  }

  const newReadme = lines.join(EOL);
  if (newReadme !== readme) {
    await Deno.writeTextFile(args.readmeMd, newReadme);
    log.info(`Updated: ${args.readmeMd}`);
  }
}
