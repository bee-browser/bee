'use strict';

import { TextLineStream } from 'https://deno.land/std@0.209.0/streams/mod.ts';

import * as acorn from 'npm:acorn@8.11.2';
import JSON5 from 'npm:json5@2.2.3';

export class Acorn {
  static parse(source, sourceType) {
    try {
      return acorn.parse(source, {
        sourceType,
        ecmaVersion: 2022,
      });
    } catch (err) {
      return null;
    }
  }
}

export class ESTree {
  static async parse(source, sourceType, options) {
    const args = ['run', '-r', '-q', '-p', 'bee-estree', '--', 'parse', sourceType];
    if (options.withDebugBuild) {
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

  constructor(options) {
    this.withDebugBuild_ = options.withDebugBuild;
  }

  start() {
    const args = ['run', '-r', '-q', '-p', 'bee-estree', '--', "serve"];
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
      .pipeThrough(new TextLineStream());
    this.encoder_ = new TextEncoder();
  }

  async parse(source, sourceType) {
    const req = this.encoder_.encode(JSON.stringify({ sourceType, source }) + '\n');

    const writer = this.child_.stdin.getWriter();
    await writer.write(req);
    writer.releaseLock();

    const reader = this.lines_.getReader();
    try {
      let line = await reader.read();
      const res = JSON5.parse(line.value);
      return res.program;
    } catch (err) {
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
    if (diff.value) {
      console.log(`${indent}  acorn : ${JSON5.stringify(diff.value)}`);
    } else {
      console.log(`${indent}  acorn : -`);
    }
    if (diff.oldValue) {
      console.log(`${indent}  estree: ${JSON5.stringify(diff.oldValue)}`);
    } else {
      console.log(`${indent}  estree: -`);
    }
  }
}
