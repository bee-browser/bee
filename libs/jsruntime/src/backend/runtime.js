'use strict';

import * as fs from '@std/fs';
import * as log from '@std/log';
import * as path from '@std/path';
import * as yaml from '@std/yaml';
import { parseCommand } from '../../../../tools/lib/cli.js';
import { setup } from '../../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} <runtime.yaml>
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
  log.debug(`Loading ${args.runtimeYaml}...`);
  const runtimeYaml = await Deno.readTextFile(args.runtimeYaml);
  const runtimeSpec = yaml.parse(runtimeYaml);

  for (const func of runtimeSpec.functions) {
    func.args = [{ name: 'runtime', type: 'VoidPtr' }].concat(func.args).map(({ name, type }) => {
      return {
        name,
        type,
        clir_type: makeCraneliftIRType(type),
      };
    });
    func.clir_ret = makeCraneliftIRType(func.ret);
  }

  console.log(JSON.stringify(runtimeSpec));
}

function makeCraneliftIRType(type) {
  switch (type) {
    case 'bool':
      return 'ir::types::I8';
    case 'u16':
      return 'ir::types::I16';
    case 'i32':
    case 'u32':
    case 'Status':
      return 'ir::types::I32';
    case 'f64':
      return 'ir::types::F64';
    case '&mut c_void':
    case '&std::ffi::CStr':
    case '&U16Chunk':
    case '&mut Variable':
    case '&Capture':
    case '&mut Capture':
    case '&mut Closure':
    case '&mut Coroutine':
    case '&mut Object':
    case '&Value':
    case '&mut Value':
    case '*mut Value':
    case '&PropertyKey':
    case 'Lambda':
    case 'VoidPtr':
      return 'addr_type';
    case undefined:
      return '';
    default:
      log.error(`unsupported type: ${type}`);
      return '';
  }
}
