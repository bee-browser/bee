'use strict';

import { assertNotEquals } from "https://deno.land/std@0.209.0/assert/mod.ts";
import { TextLineStream, toTransformStream } from 'https://deno.land/std@0.209.0/streams/mod.ts';

import * as acorn from 'npm:acorn@8.11.2';

export class Acorn {
  static parse(source, sourceType) {
    try {
      return refine(acorn.parse(source, {
        sourceType,
        ecmaVersion: 2022,
      }));
    } catch {
      return null;
    }
  }
}

// ESTree contains values that are not allowed in JSON.  This function converts such a value into
// some kind of a "tag".  This is enough because a string corresponding to the value is already
// stored in Literal.raw.
//
// We assume that there is no objects representing primitive types.
function refine(obj) {
  switch (typeof obj) {
  case 'boolean':
    return obj;
  case 'number':
    if (isNaN(obj)) {
      return { type: 'NaN' };
    }
    if (obj === Infinity) {
      return { type: 'Infinity' };
    }
    // `-Infinity` is represented with UnaryExpression('-', Infinity) in ESTree.
    assertNotEquals(obj, -Infinity);
    return obj;
  case 'bigint':
    return { type: 'BigInt' };
  case 'string':
    return obj;
  default:
    if (obj === null) {
      return null;
    }
    if (Array.isArray(obj)) {
      return obj.map(refine);
    }
    if (obj instanceof RegExp) {
      return { type: 'RegExp' };
    }
    let refined = {};
    for (const [key, value] of Object.entries(obj)) {
      refined[key] = refine(value);
    }
    return refined;
  }
}

export class ESTree {
  static async parse(source, sourceType, options = {}) {
    const args = ['run', '-r', '-q', '-p', 'estree', '--', 'parse', sourceType];
    if (!!options.withDebugBuild) {
      args.splice(1, 1);  // remove '-r'
    }
    const child = new Deno.Command('cargo', {
      args,
      stdin: 'piped',
      stdout: 'piped',
      stderr: 'null',
    }).spawn();
    const encoder = new TextEncoder();
    const writer = child.stdin.getWriter();
    writer.write(encoder.encode(source));
    writer.releaseLock();
    child.stdin.close();
    try {
      const decoder = new TextDecoder();
      const output = await child.output();
      return JSON.parse(decoder.decode(output.stdout));
    } catch (err) {
      return null;
    }
  }

  constructor(options = {}) {
    this.withDebugBuild_ = !!options.withDebugBuild;
  }

  start() {
    const args = ['run', '-r', '-q', '-p', 'estree', '--', "serve"];
    if (this.withDebugBuild_) {
      args.splice(1, 1);  // remove '-r'
    }
    const cmd = new Deno.Command('cargo', {
      args,
      stdin: 'piped',
      stdout: 'piped',
      stderr: 'null',
    });
    this.child_ = cmd.spawn();
    this.lines_ = this.child_.stdout
      .pipeThrough(new TextDecoderStream())
      .pipeThrough(new TextLineStream())
      .pipeThrough(toTransformStream(async function* (lines) {
        for await (const line of lines) {
          yield JSON.parse(line);
        }
      }));
    this.encoder_ = new TextEncoder();
  }

  async parse(source, sourceType) {
    const req = this.encoder_.encode(JSON.stringify({ sourceType, source }) + '\n');

    const writer = this.child_.stdin.getWriter();
    await writer.write(req);
    writer.releaseLock();

    const reader = this.lines_.getReader();
    try {
      return (await reader.read()).value.program;
    } catch {
      this.start();
      return null;
    } finally {
      reader.releaseLock();
    }
  }

  async stop() {
    await this.child_.stdin.close();
    await this.child_.status;
  }
}

export function showDiffs(diffs, indent = '') {
  function doShowDiff(path, actual, expected) {
    console.log(`${indent}${path.join('.')}`);
    if (expected) {
      console.log(`${indent}  acorn :`, expected);
    } else {
      console.log(`${indent}  acorn : -`);
    }
    if (actual) {
      console.log(`${indent}  estree:`, actual);
    } else {
      console.log(`${indent}  estree: -`);
    }
  }
  function showDiff(path, diff) {
    switch (diff.kind) {
    case 'A':
      showDiff([...path, diff.index], diff.item);
      break;
    default:
      doShowDiff(path, diff.lhs, diff.rhs);
      break;
    }
  }
  for (const diff of diffs) {
    showDiff(diff.path, diff);
  }
}
