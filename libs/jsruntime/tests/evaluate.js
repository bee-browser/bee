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
  function mapValue(value, strings) {
    if (strings.includes(value)) {
      const index = strings.indexOf(value);
      return `Value::String(StringHandle::new(&STRING${index}))`;
    }
    switch (value) {
      case undefined:
        return undefined;
      case 'undefined':
        return 'Value::Undefined';
      case 'null':
        return 'Value::Null';
      case 'NaN':
        return 'Value::Number(f64::NAN)';
      case 'Infinity':
        return 'Value::Number(f64::INFINITY)';
      case '-Infinity':
        return 'Value::Number(-f64::INFINITY)';
      case 'object':
        return 'Value::dummy_object()';
      default:
        return `Value::from(${value})`;
    }
  }

  const tests = [];
  for (const test of args.tests) {
    log.debug(`Reading ${test}...`);
    const script = await Deno.readTextFile(test);
    const lines = script.split('\n').map((line) => line.trim());
    const strings = lines
      .filter((line) => !line.startsWith('// '))
      .filter((line) => line.includes('///='))
      .map((line) => line.split('///=')[1].trim())
      .filter((line) => line.startsWith('"') || line.startsWith("'"))
      .reduce((strings, literal) => {
        if (!strings.includes(literal)) {
          strings.push(literal);
        }
        return strings;
      }, []);
    const sequencedValues = lines
      .filter((line) => !line.startsWith('// '))
      .filter((line) => line.includes('///='))
      .map((line) => line.split('///=')[1].trim())
      .map((value) => mapValue(value, strings));
    const orderedValues = lines
      .filter((line) => line.includes('///#'))
      .map((line) => line.split('///#')[1].trim().split('='))
      .map(([i, v]) => [i, mapValue(v, strings)])
      .reduce((acc, [i, v]) => { acc[i] = v; return acc; }, []);
    const throws = mapValue(lines.find((line) => line.includes('///!'))?.split('///!')[1].trim(), strings);
    const name = path.basename(test).replace('.', '_');
    const module = test.endsWith('.js') ? false : true;
    tests.push({
      filename: test,
      name,
      module,
      strings: strings.map((literal) => {
        const string = JSON.parse(literal);  // use JSON.parse() for parsing the string literal.
        const data = [];
        for (let i = 0; i < string.length; ++i) {
          data.push(string.charCodeAt(i));
        }
        const size = string.length;
        return { data, size };
      }),
      expectedValues: sequencedValues.length > 0 ? sequencedValues : orderedValues,
      throws,
    });
  }
  console.log(JSON.stringify({ tests }));
  return 0;
}
