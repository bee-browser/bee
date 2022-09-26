'use strict';

// The import map is NOT applied to workers:
// https://github.com/denoland/deno/issues/6675
import * as io from 'https://deno.land/std@0.157.0/io/mod.ts';
import * as path from 'https://deno.land/std@0.157.0/path/mod.ts';

const TOOLS_DIR = path.resolve(path.dirname(path.fromFileUrl(import.meta.url)), '..', '..');
const TEXT_TO_DOT_MATRIX = path.join(TOOLS_DIR, 'bin', 'bee-tools-text-to-dot-matrix');
const DOM_SCRAPER = path.join(TOOLS_DIR, 'bin', 'bee-tools-dom-scraper');
const LAYOUT_BUILDER = path.join(TOOLS_DIR, 'bin', 'bee-tools-layout-builder');

self.onmessage = async ({ data }) => {
  try {
    await layout(await scrape(data), data);
    //const script = buildScript(data);
    //await runScript(script);
  } catch (err) {
    self.postMessage({
      type: 'error',
      data: err.toString(),
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
  const commands = [];

  if (uri.startsWith('text:')) {
    const text = uri.slice(5);
    commands.push(`${TEXT_TO_DOT_MATRIX} ${JSON.stringify(text)}`);
    commands.push(`${DOM_SCRAPER} --viewport=${width}x${height}`);
  } else {
    commands.push(`${DOM_SCRAPER} --viewport=${width}x${height} ${uri}`);
  }

  const script = commands.join(' | ');

  const shell = Deno.run({
    cmd: ['sh', '-c', script],
    stdin: 'null',
    stdout: 'piped',
  });

  const output = await shell.output();
  const { code } = await shell.status();
  if (code !== 0) {
    throw new Error(`Failed to scrape: ${code}: ${script}`);
  }

  // Send assets to the debug console.
  const dom = JSON.parse(new TextDecoder().decode(output));
  for (let asset of Object.values(dom.assets)) {
    self.postMessage({
      type: 'asset',
      data: JSON.stringify({
        type: 'asset.add',
        data: asset,
      }),
    });
  }

  shell.close();

  return output;
}

async function layout(input, { layouter }) {
  const commands = [];
  commands.push(LAYOUT_BUILDER);
  commands.push(layouter);

  const script = commands.join(' | ');

  const shell = Deno.run({
    cmd: ['sh', '-c', script],
    stdin: 'piped',
    stdout: 'piped',
  });

  await Deno.writeAll(shell.stdin, input);
  await shell.stdin.close();

  for await (let json of io.readLines(shell.stdout)) {
    self.postMessage({
      type: 'render',
      data: json,
    });
  }

  shell.close();
}

function buildScript({ uri, width, height, layouter}) {
  const commands = [];

  if (uri.startsWith('text:')) {
    const text = uri.slice(5);
    commands.push(`${TEXT_TO_DOT_MATRIX} ${JSON.stringify(text)}`);
    commands.push(`${DOM_SCRAPER} --viewport=${width}x${height}`);
  } else {
    commands.push(`${DOM_SCRAPER} --viewport=${width}x${height} ${uri}`);
  }
  commands.push(LAYOUT_BUILDER);
  commands.push(layouter);

  return commands.join(' | ');
}

async function runScript(script) {
  const shell = Deno.run({
    cmd: ['sh', '-c', script],
    stdin: 'null',
    stdout: 'piped',
    stderr: 'null',
  });

  const { code } = await shell.status();
  if (code !== 0) {
    throw new Error(`Failed to scrape: ${code}: ${script}`);
  }

  for await (let json of io.readLines(shell.stdout)) {
    self.postMessage({
      type: 'render',
      data: json,
    });
  }

  shell.close();
}
