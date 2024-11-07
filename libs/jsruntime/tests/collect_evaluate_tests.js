'use strict';

import * as log from '@std/log';
import * as path from '@std/path';
import { parseCommand } from '../../../tools/lib/cli.js';
import { setup } from '../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} <tests>...
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
  function mapValue(value) {
    switch (value) {
      case 'undefined':
        return 'Value::Undefined';
      case 'null':
        return 'Value::Null';
      case 'NaN':
        return 'f64::NAN';
      case 'Infinity':
        return 'f64::INFINITY';
      case '-Infinity':
        return '-f64::INFINITY';
      default:
        return value;
    }
  }

  const tests = [];
  for (const test of args.tests) {
    log.debug(`Reading ${test}...`);
    const script = await Deno.readTextFile(test);
    const lines = script.split('\n').map((line) => line.trim());
    const sequencedValues = lines
      .filter((line) => line.includes('///='))
      .map((line) => line.split('///=')[1].trim())
      .map(mapValue);
    const orderedValues = lines
      .filter((line) => line.includes('///#'))
      .map((line) => line.split('///#')[1].trim().split('='))
      .map(([i, v]) => [i, mapValue(v)])
      .reduce((acc, [i, v]) => { acc[i] = v; return acc; }, []);
    const throws = mapValue(lines.find((line) => line.includes('///!'))?.split('///!')[1].trim());
    const name = path.basename(test).replace('.', '_');
    const module = test.endsWith('.js') ? false : true;
    tests.push({
      filename: test,
      name,
      module,
      expectedValues: sequencedValues.length > 0 ? sequencedValues : orderedValues,
      throws,
    });
  }
  console.log(JSON.stringify({ tests }));
  return 0;
}
