'use strict';

import * as fs from 'https://deno.land/std@0.220.1/fs/mod.ts';
import * as log from 'https://deno.land/std@0.220.1/log/mod.ts';
import * as path from 'https://deno.land/std@0.220.1/path/mod.ts';
import * as yaml from 'https://deno.land/std@0.220.1/yaml/mod.ts';
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
    func.c_type = makeCFunc(func);
    func.llvmir_ret = makeLLVMIRType(func.ret);
    func.llvmir_args = makeLLVMIRArgs(func);
  }

  console.log(JSON.stringify(runtimeSpec));
}

function makeCFunc(func) {
  const args = [{ name: 'context', type: 'opaque' }].concat(func.args).map(({ name, type }) =>
    `${makeCType(type)} ${name}`
  ).join(', ');
  return `${makeCType(func.ret)} (*${func.name})(${args})`;
}

function makeLLVMIRArgs(func) {
  let args = ['opaque'].concat(func.args.map((arg) => arg.type)).map(makeLLVMIRType).join(', ');
  return `{${args}}`;
}

function makeLLVMIRType(type) {
  switch (type) {
    case 'bool':
      return 'builder_.getInt1Ty()';
    case 'u16':
      return 'builder_.getInt16Ty()';
    case 'u32':
      return 'builder_.getInt32Ty()';
    case 'f64':
      return 'builder_.getDoubleTy()';
    case '&Value':
    case '&mut Value':
    case 'opaque':
      return 'builder_.getPtrTy()';
    case undefined:
      return 'builder_.getVoidTy()';
    default:
      log.error(`unsupported type: ${type}`);
      return;
  }
}

function makeCType(type) {
  switch (type) {
    case 'bool':
      return 'bool';
    case 'u16':
      return 'uint16_t';
    case 'u32':
      return 'uint32_t';
    case 'f64':
      return 'double';
    case '&Value':
      return 'const Value*';
    case '&mut Value':
      return 'Value*';
    case 'opaque':
      return 'uintptr_t';
    case undefined:
      return 'void';
    default:
      log.error(`unsupported type: ${type}`);
      return;
  }
}
