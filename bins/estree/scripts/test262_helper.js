'use strict';

import { TextLineStream, toTransformStream } from 'https://deno.land/std@0.209.0/streams/mod.ts';

import * as acorn from 'npm:acorn@8.11.2';
import JSON5 from 'npm:json5@2.2.3';

export class Acorn {
  static parse(source, sourceType) {
    try {
      return acorn.parse(source, {
        sourceType,
        ecmaVersion: 2022,
      });
    } catch {
      return null;
    }
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
      return JSON5.parse(decoder.decode(output.stdout));
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
          yield JSON5.parse(line);
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
  for (const diff of diffs) {
    console.log(`${indent}${diff.path.join('.')}`);
    if ('value' in diff) {
      console.log(`${indent}  acorn : ${JSON5.stringify(diff.value)}`);
    } else {
      console.log(`${indent}  acorn : -`);
    }
    if ('oldValue' in diff) {
      console.log(`${indent}  estree: ${JSON5.stringify(diff.oldValue)}`);
    } else {
      console.log(`${indent}  estree: -`);
    }
  }
}
