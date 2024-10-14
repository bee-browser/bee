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
    func.args = [{ name: 'context', type: 'opaque' }].concat(func.args).map(({ name, type }) => {
      return { name, type, ctype: makeCType(type), llvmir_type: makeLLVMIRType(type) };
    });
    func.c_type = makeCFunc(func);
    func.c_ret = makeCType(func.ret);
    func.llvmir_ret = makeLLVMIRType(func.ret);
  }

  console.log(JSON.stringify(runtimeSpec));
}

function makeCFunc(func) {
  const args = func.args.map((arg) => `${arg.ctype} ${arg.name}`).join(', ');
  return `${makeCType(func.ret)} (*${func.name})(${args})`;
}

function makeLLVMIRType(type) {
  switch (type) {
    case 'bool':
      return 'builder_.getInt1Ty()';
    case 'u16':
      return 'builder_.getInt16Ty()';
    case 'i32':
    case 'u32':
      return 'builder_.getInt32Ty()';
    case 'f64':
      return 'builder_.getDoubleTy()';
    case '&std::ffi::CStr':
    case '&mut Variable':
    case '&mut Capture':
    case '&mut Closure':
    case '&mut Coroutine':
    case '&Value':
    case '&mut Value':
    case '*mut Value':
    case 'Lambda':
    case 'opaque':
      return 'builder_.getPtrTy()';
    case undefined:
      return 'builder_.getVoidTy()';
    default:
      log.error(`unsupported type: ${type}`);
      return '';
  }
}

function makeCType(type) {
  switch (type) {
    case 'bool':
      return 'bool';
    case 'u16':
      return 'uint16_t';
    case 'i32':
      return 'int32_t';
    case 'u32':
      return 'uint32_t';
    case 'f64':
      return 'double';
    case '&std::ffi::CStr':
      return 'const char*';
    case '&mut Variable':
      return 'Variable*';
    case '&mut Capture':
      return 'Capture*';
    case '&mut Closure':
      return 'Closure*';
    case '&mut Coroutine':
      return 'Coroutine*';
    case '&Value':
      return 'const Value*';
    case '&mut Value':
    case '*mut Value':
      return 'Value*';
    case 'Lambda':
      return 'Lambda';
    case 'opaque':
      return 'uintptr_t';
    case undefined:
      return 'void';
    default:
      log.error(`unsupported type: ${type}`);
      return '';
  }
}
