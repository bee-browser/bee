'use strict';

import * as log from '@std/log';
import * as path from '@std/path';
import { Application, Router } from '@oak/oak';
import { PROJ_DIR, WORKERS_DIR } from './consts.js';
import { LayoutBuilder } from './layout_builder.js';

const NAVIGATION_JS = path.join(WORKERS_DIR, 'navigation.js');
const DEFAULT_VIEWPORT_WIDTH = 1280;
const DEFAULT_VIEWPORT_HEIGHT = 720;

await log.setup({
  handlers: {
    console: new log.ConsoleHandler('DEBUG', {
      formatter: (rec) => {
        const timestamp = rec.datetime.toISOString();
        return `${timestamp} ${rec.levelName.padEnd(7)} ${rec.msg}`;
      },
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
  const router = new Router();

  router.get('/api/debcon', (context) => {
    let socket = context.upgrade();
    socket.addEventListener('message', (event) => {
      if (typeof event.data === 'string') {
        handleJson_(socket, event.data, options);
      }
    });
  });

  const app = new Application();

  // error handler
  app.use(async (context, next) => {
    try {
      await next();
    } catch (err) {
      log.error(`${context.request.url.pathname}: ${err}`);
    }
  });

  // api endpoints
  app.use(router.routes());

  // static files
  app.use(async (context) => {
    await context.send({
      root: options.root,
      index: 'index.html',
    });
  });

  await app.listen({ port: options.port });
}

function handleJson_(socket, json, options) {
  const msg = JSON.parse(json);
  switch (msg.type) {
    case 'navigation.go':
      return handleNavigationGo_(socket, msg.data, options);
  }
}

function handleNavigationGo_(socket, { uri, viewport, remotes }, { debugBuild }) {
  log.info(`navigation.go: ${uri}`);

  if (!uri) { // required
    return;
  }

  let width = viewport?.width ?? DEFAULT_VIEWPORT_WIDTH;
  let height = viewport?.height ?? DEFAULT_VIEWPORT_HEIGHT;

  if (typeof remotes !== 'object') {
    remotes = {};
  }

  let layouter = path.join(PROJ_DIR, 'target', debugBuild ? 'debug' : 'release', 'lmp');

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
    type: 'module',
    deno: {
      namespace: true,
      permissions: 'inherit',
    },
  });
  worker.onmessage = async ({ data }) => {
    switch (data.type) {
      case 'asset':
        log.debug(`navigation.worker.asset: ${data.data}`);
        socket.send(data.data);
        break;
      case 'render':
        log.debug(`navigation.worker.render: ${data.data}`);
        socket.send(data.data);
        break;
      case 'done':
        log.debug('navigation.worker.done');
        socket.send(JSON.stringify({ type: 'navigation.end' }));
        log.info('navigation.end');
        break;
      case 'log':
        switch (data.data.level) {
          case 'debug':
            log.debug(`navigation.worker.log: ${JSON.stringify(data.data.data)}`);
            break;
          case 'info':
            log.info(`navigation.worker.log: ${JSON.stringify(data.data.data)}`);
            break;
          case 'warn':
            log.warn(`navigation.worker.log: ${JSON.stringify(data.data.data)}`);
            break;
          case 'error':
            log.error(`navigation.worker.log: ${JSON.stringify(data.data.data)}`);
            break;
        }
        break;
      default:
        log.debug(data);
        break;
    }
  };
  worker.onerror = (err) => {
    log.error(`navigation.worker.error: ${err}`);
  };

  log.debug('navigation.worker.start');
  worker.postMessage({ uri, width, height, layouter });

  socket.send(JSON.stringify({ type: 'navigation.start' }));
  log.info('navigation.start');
}
