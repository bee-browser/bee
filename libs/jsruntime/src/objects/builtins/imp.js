'use strict';

import { unreachable } from '@std/assert';
import { EOL } from '@std/fs';
import * as log from '@std/log';
import * as path from '@std/path';
import { TextLineStream, toTransformStream } from '@std/streams';
import { constantCase } from 'change-case';
import { JSDOM } from 'jsdom';
import { parseCommand } from '../../../../../tools/lib/cli.js';
import { VENDOR_DIR } from '../../../../../tools/lib/consts.js';
import { setup } from '../../../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));
const ECMA262_SPEC_HTML = path.join(VENDOR_DIR, 'src', 'tc39', 'ecma262', 'spec.html');

const DOC = `
Usage:
  ${PROGNAME} [options] [<imp.rs>]
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
  log.debug(`Loading ${ECMA262_SPEC_HTML}...`);
  const spec = new JSDOM(await Deno.readTextFile(ECMA262_SPEC_HTML));

  const json = {
    metadata: {},
    constructor: null,
    constructorProperties: [],
    prototypeProperties: [],
  };

  for await (const data of dataStream(args.impRs)) {
    switch (data.kind) {
      case 'metadata':
        json.metadata[data.name] = data.value;
        break;
      case 'constructor':
        collectFunctionDataFromSpec(spec, data);
        json.constructor = data;
        break;
      case 'constructor.function':
        collectFunctionDataFromSpec(spec, data);
        json.constructorProperties.push(data);
        break;
      case 'prototype.function':
        collectFunctionDataFromSpec(spec, data);
        json.prototypeProperties.push(data);
        break;
      default:
        unreachable();
        break;
    }
  }

  console.log(JSON.stringify(json));
}

function dataStream(impRs) {
  function parseMetadata(line) {
    const parts = line.substring(3).split(' ', 2);
    if (parts.length < 2) {
      log.error(`Incorrect metadata line: ${line}`);
      Deno.exit(1);
    }
    return {
      kind: 'metadata',
      name: parts[0],
      value: parts[1],
    }
  }

  function parseId(line) {
    const parts = line.substring(3).split(' ');
    if (parts.length < 2) {
      log.error(`Incorrect ID line: ${line}`);
      Deno.exit(1);
    }
    return {
      id: parts[0],
      kind: parts[1],
      options: parts.slice(2),
    };
  }

  function parseImp(line) {
    const parts = line.split(' ');
    return parts[2].split('<')[0];
  }

  let stream = Deno.stdin;
  if (impRs !== null) {
    stream = Deno.open(impRs);
  }

  return stream
    .readable
    .pipeThrough(new TextDecoderStream())
    .pipeThrough(new TextLineStream())
    .pipeThrough(toTransformStream(async function* (lines) {
      let state = 'id';
      let data;
      for await (let line of lines) {
        line = line.trim();
        if (line.length === 0) {
          continue;
        }
        if (line.startsWith('//$')) {
          data = parseMetadata(line);
          yield data;
          data = undefined;
          continue;
        }
        switch (state) {
          case 'id':
            if (!line.startsWith('//#')) {
              continue;
            }
            data = parseId(line);
            state = 'imp';
            break;
          case 'imp':
            if (!line.startsWith('pub fn ')) {
              continue;
            }
            data.imp = parseImp(line);
            state = 'id';
            yield data;
            data = undefined;
            break;
          default:
            unreachable();
        }
      }
      if (state === 'imp') {
        log.error(`No implementation for ${data.id}`);
        Deno.exit(1);
      }
    }));
}

function collectFunctionDataFromSpec(spec, data) {
  let clause = spec.window.document.getElementById(data.id);
  data.signature = parseSignature(clause.firstElementChild.textContent.trim());
  data.alg = parseAlg(clause.getElementsByTagName('emu-alg').item(0).textContent);
  data.length = 0;
  for (const arg of data.signature.args) {
    if (!arg.optional) {
      data.length += 1;
    }
  }
  switch (data.kind) {
    case 'constructor':
      data.name = data.signature.name;
      data.symbol = constantCase(data.name);
      break;
    case 'constructor.function':
      data.name = data.signature.name.split('.')[1];
      data.symbol = constantCase(data.name);
      break;
    case 'prototype.function':
      data.name = data.signature.name.split('.')[2];
      data.symbol = constantCase(data.name);
      break;
    default:
      unreachable();
  }
  return data;
}

function parseSignature(text, data) {
  let parts = text.split('(');
  let name = parts[0].trim();
  let args = parseArgs(parts[1].split(')')[0].trim());
  return { name, args };
}

function parseArgs(text) {
  let args = [];
  let optional = false;
  let rest = false;
  for (const part of text.split(' ')) {
    switch (part) {
      case '[':
        optional = true;
        break;
      case ']':
        optional = false;
        break;
      case ',':
        break;
      case '...':
        rest = true;
        break;
      default:
        args.push({
          name: part.substring(1, part.length - 1), // remove '_'
          optional,
          rest,
        });
        rest = false;
        break;
    }
  }
  return args;
}

function parseAlg(text) {
  let steps = [];

  let base;
  for (const line of text.split(EOL)) {
    if (line.length === 0) {
      continue;
    }
    base = line.indexOf('1.');
    break;
  }

  const indent = 2;
  for (const line of text.split(EOL)) {
    if (line.length === 0) {
      continue;
    }
    let offset = line.indexOf('1.');
    if (offset === -1) {
      continue;
    }
    let depth = (offset - base) / indent;
    let description = line.substring(offset + 3);
    steps.push({ description, depth });
  }

  return steps;
}
