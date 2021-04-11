'use strict';

import { path } from '../deps.js';
import { PROJ_DIR, TOOLS_DIR } from '../lib/consts.js';

let TEXT_TO_DOT_MATRIX = path.join(TOOLS_DIR, 'bin', 'bee-tools-text-to-dot-matrix');
let DOM_SCRAPER = path.join(TOOLS_DIR, 'bin', 'bee-tools-dom-scraper');
let LAYOUT_BUILDER = path.join(TOOLS_DIR, 'bin', 'bee-tools-layout-builder');

self.onmessage = async ({ data }) => {
  try {
    const script = buildScript(data);
    await runScript(script)
  } catch (err) {
    self.postMessage({
      type: 'error',
      data: err.toString(),
    });
    throw err;  // FIXME: this doesn't cause an error event
  } finally {
    self.postMessage({
      type: 'done',
    });
    self.close();
  }
};

function buildScript({ uri, width, height, layouter}) {
  const commands = [];

  if (uri.startsWith('text:')) {
    const text = uri.slice(5);
    commands.push(`${TEXT_TO_DOT_MATRIX} ${JSON.stringify(text)}`);
    commands.push(`${DOM_SCRAPER} --viewport=${width}x${height}`);
  } else {
    scripts.push(`${DOM_SCRAPER} --viewport=${width}x${height} ${uri}`);
  }
  commands.push(LAYOUT_BUILDER);
  commands.push(layouter);

  const encoder = new TextEncoder();
  return encoder.encode(commands.join(' | '));
}

async function runScript(script) {
  const shell = Deno.run({
    cmd: ['sh'],
    stdin: 'piped',
    stdout: 'piped',
    stderr: 'piped',
  });

  await shell.stdin.write(script);
  await shell.stdin.close();

  const [status, stdout, stderr] = await Promise.all([
    shell.status(),
    shell.output(),
    shell.stderrOutput(),
  ]);

  const decoder = new TextDecoder();
  const jsonl = decoder.decode(stdout);
  for (const json of jsonl.split('\n')) {
    self.postMessage({
      type: 'render',
      data: json,
    });
  }
}
