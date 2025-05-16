'use strict';

import * as io from '@std/io';
import * as path from '@std/path';
import { TextLineStream } from '@std/streams';

const TOOLS_DIR = path.resolve(path.dirname(path.fromFileUrl(import.meta.url)), '..', '..');
const TEXT_TO_DOT_MATRIX = path.join(TOOLS_DIR, 'bin', 'text_to_dot_matrix.js');
const DOM_SCRAPER = path.join(TOOLS_DIR, 'bin', 'scrape_dom.sh');
const LAYOUT_BUILDER = path.join(TOOLS_DIR, 'bin', 'build_layout.js');

class Logger {
  debug(data) {
    this.log_('debug', data);
  }

  info(data) {
    this.log_('info', data);
  }

  warn(data) {
    this.log_('warn', data);
  }

  error(data) {
    this.log_('error', data);
  }

  log_(level, data) {
    self.postMessage({
      type: 'log',
      data: { level, data },
    });
  }
}

const log = new Logger();

self.onmessage = async ({ data }) => {
  try {
    await layout(await scrape(data), data);
    const script = buildScript(data);
    await runScript(script);
  } catch (err) {
    self.postMessage({
      type: 'error',
      data: err,
    });
    //throw err;  // FIXME: this doesn't cause an error event
  } finally {
    self.postMessage({
      type: 'done',
    });
    self.close();
  }
};

async function scrape({ uri, width, height }) {
  log.debug({ event: 'scrape.start', uri, width, height });

  const commands = [];

  if (uri.startsWith('text:')) {
    const text = uri.slice(5);
    commands.push(`deno run -qA ${TEXT_TO_DOT_MATRIX} ${JSON.stringify(text)}`);
    commands.push(`sh ${DOM_SCRAPER} --viewport ${width}x${height}`);
  } else {
    commands.push(`sh ${DOM_SCRAPER} --viewport ${width}x${height} ${uri}`);
  }

  const script = commands.join(' | ');

  const cmd = new Deno.Command('sh', {
    args: ['-c', script],
    stdin: 'null',
    stdout: 'piped',
  });

  const shell = cmd.spawn();
  const { code, stdout } = await shell.output();
  if (code !== 0) {
    throw new Error(`Failed to scrape: ${code}: ${script}`);
  }

  // Send assets to the debug console.
  const dom = JSON.parse(new TextDecoder().decode(stdout));
  for (let asset of Object.values(dom.assets)) {
    self.postMessage({
      type: 'asset',
      data: JSON.stringify({
        type: 'asset.add',
        data: asset,
      }),
    });
  }

  log.debug({ event: 'scrape.end', uri, width, height });

  return stdout;
}

async function layout(input, { layouter }) {
  log.debug({ event: 'layout.start' });

  const commands = [];
  commands.push(`deno run -qA ${LAYOUT_BUILDER}`);
  commands.push(layouter);

  const script = commands.join(' | ');

  const cmd = new Deno.Command('sh', {
    args: ['-c', script],
    stdin: 'piped',
    stdout: 'piped',
  });

  const shell = cmd.spawn();

  const writer = shell.stdin.getWriter();
  await writer.write(input);
  writer.close();

  const lines = shell
    .stdout
    .pipeThrough(new TextDecoderStream())
    .pipeThrough(new TextLineStream());
  for await (let json of lines) {
    self.postMessage({
      type: 'render',
      data: json,
    });
  }

  log.debug({ event: 'layout.end' });
}

function buildScript({ uri, width, height, layouter }) {
  const commands = [];

  if (uri.startsWith('text:')) {
    const text = uri.slice(5);
    commands.push(`deno run -qA ${TEXT_TO_DOT_MATRIX} ${JSON.stringify(text)}`);
    commands.push(`sh ${DOM_SCRAPER} --viewport ${width}x${height}`);
  } else {
    commands.push(`sh ${DOM_SCRAPER} --viewport ${width}x${height} ${uri}`);
  }
  commands.push(`deno run -qA ${LAYOUT_BUILDER}`);
  commands.push(layouter);

  return commands.join(' | ');
}

async function runScript(script) {
  const cmd = new Deno.Command('sh', {
    args: ['-c', script],
    stdin: 'null',
    stdout: 'piped',
    stderr: 'null',
  });

  const shell = cmd.spawn();

  const lines = shell
    .stdout
    .pipeThrough(new TextDecoderStream())
    .pipeThrough(new TextLineStream());
  for await (let json of lines) {
    self.postMessage({
      type: 'render',
      data: json,
    });
  }
}
