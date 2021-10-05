'use strict';

import * as log from 'std/log/mod.ts';
import * as path from 'std/path/mod.ts';
import * as servest from 'servest';
import { PROJ_DIR, WORKERS_DIR } from './consts.js';
import { scrape } from './chrome_devtools.js';
import { LayoutBuilder } from './layout_builder.js';

const NAVIGATION_JS = path.join(WORKERS_DIR, 'navigation.js');

await log.setup({
  handlers: {
    console: new log.handlers.ConsoleHandler('DEBUG', {
      formatter: (rec) => {
        const timestamp = rec.datetime.toISOString();
        return `${timestamp} ${rec.levelName.padEnd(7)} ${rec.msg}`;
      }
    }),
  },
  loggers: {
    default: {
      level: 'DEBUG',
      handlers: ['console'],
    },
  },
});

export async function serve(options) {
  const app = servest.createApp({
    logger: logBridge_,
  });

  app.ws('/api/debcon', async (socket) => {
    for await (const data of socket) {
      if (typeof data === 'string') {
        await handleJson_(socket, data, options);
      }
    }
  });

  app.use(servest.serveStatic(options.root));

  await app.listen({ port: options.port });
}

function logBridge_(level, msg, ...args) {
  switch (level) {
  case servest.Loglevel.DEBUG:
    log.debug(msg, ...args);
    break;
  case servest.Loglevel.INFO:
    log.info(msg, ...args);
    break;
  case servest.Loglevel.WARN:
    log.warning(msg, ...args);
    break;
  case servest.Loglevel.ERROR:
    log.error(msg, ...args);
    break;
  }
}

async function handleJson_(socket, json, options) {
  const msg = JSON.parse(json);
  switch (msg.type) {
  case 'navigation.go':
    return await handleNavigationGo_(socket, msg.data, options);
  }
}

async function handleNavigationGo_(socket, { uri, viewport, remotes }, { debugBuild }) {
  if (!uri) {  // required
    return;
  }

  let width = 1280;
  let height = 720;
  if (typeof viewport === 'object') {
    if ('width' in viewport) {
      width = viewport.width;
    }
    if ('height' in viewport) {
      height = viewport.height;
    }
  }

  if (typeof remotes !== 'object') {
    remotes = {};
  }

  let layouter = path.join(PROJ_DIR, 'target', debugBuild ? 'debug' : 'release', 'bee-lmp');

  // TODO
  // ----
  // Create a worker used as an event source.
  //
  // At this point, Deno.Process.[stdout, stderr] don't implement the EventTarget interface.
  // So, we cannot write code like blow:
  //
  //   let child = Deno.run();
  //   child.on('exit', (code, signal) => { ... });
  //   child.stdout.on('data', (data) => { ... });
  //
  // We build a command pipeline on the worker and feed output from the pipeline back to
  // the main thread as messages.

  const worker = new Worker(new URL(NAVIGATION_JS, import.meta.url).href, {
    type: "module",
    deno: {
      namespace: true,
      permissions: 'inherit',
    },
  });
  worker.onmessage = async ({ data }) => {
    switch (data.type) {
    case 'render':
      socket.send(data.data);
      break;
    case 'done':
      socket.send(JSON.stringify({ type: 'navigation.end' }));
      break;
    default:
      log.debug(data);
      break;
    }
  };
  worker.onerror = (err) => {
    log.error(`Navigation worker failed: ${err}`);
  };
  worker.postMessage({ uri, width, height, layouter });

  socket.send(JSON.stringify({ type: 'navigation.start' }));
}
