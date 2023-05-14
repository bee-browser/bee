'use strict';

import * as log from 'https://deno.land/std@0.187.0/log/mod.ts';
import * as path from 'https://deno.land/std@0.187.0/path/mod.ts';
import * as servest from 'https://deno.land/x/servest@v1.3.4/mod.ts';
import { PROJ_DIR, WORKERS_DIR } from './consts.js';
import { scrape } from './chrome_devtools.js';
import { LayoutBuilder } from './layout_builder.js';

const NAVIGATION_JS = path.join(WORKERS_DIR, 'navigation.js');
const DEFAULT_VIEWPORT_WIDTH = 1280;
const DEFAULT_VIEWPORT_HEIGHT = 720;

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
      level: Deno.env.get('BEE_TOOLS_LOG') ?? 'INFO',
      handlers: ['console'],
    },
  },
});

export async function serve(options) {
  const app = servest.createApp({
    logger: (level, msg, ...args) => {
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
    },
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

async function handleJson_(socket, json, options) {
  const msg = JSON.parse(json);
  switch (msg.type) {
  case 'navigation.go':
    return await handleNavigationGo_(socket, msg.data, options);
  }
}

async function handleNavigationGo_(socket, { uri, viewport, remotes }, { debugBuild }) {
  log.info(`navigation: navigation.go: ${uri}`);

  if (!uri) {  // required
    return;
  }

  let width = viewport?.width ?? DEFAULT_VIEWPORT_WIDTH;
  let height = viewport?.height ?? DEFAULT_VIEWPORT_HEIGHT;

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
    case 'asset':
      log.debug('navigation: worker: asset message');
      socket.send(data.data);
      break;
    case 'render':
      log.debug('navigation: worker: render message');
      socket.send(data.data);
      break;
    case 'done':
      socket.send(JSON.stringify({ type: 'navigation.end' }));
      log.info('navigation: navigation.end');
      break;
    default:
      log.debug(data);
      break;
    }
  };
  worker.onerror = (err) => {
    log.error(`navigation: worker: ${err}`);
  };

  log.debug('navigation: start worker');
  worker.postMessage({ uri, width, height, layouter });

  socket.send(JSON.stringify({ type: 'navigation.start' }));
  log.info('navigation: navigation.start')
}
