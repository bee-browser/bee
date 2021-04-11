'use strict';

import {
  oak,
  oak_logger,
  path,
} from '../deps.js';

import { PROJ_DIR, RESOURCES_DIR } from './consts.js';
import { scrape } from './dom_scraper.js';
import { LayoutBuilder } from './layout_builder.js';

const NAVIGATION_JS = path.join(RESOURCES_DIR, 'navigation.worker.js');

export async function serve(options) {
  const router = new oak.Router();
  router.get('/api/debcon', async (context) => {
    if (!context.isUpgradable) {
      throw new Error();
    }
    const socket = await context.upgrade();
    for await (const data of socket) {
      if (typeof data === 'string') {
        await handleJson_(socket, data, options);
      }
    }
  });

  const app = new oak.Application();
  app.use(oak_logger.logger);
  app.use(assetsMiddleware_(options));
  app.use(router.routes());
  app.use(router.allowedMethods());
  await app.listen({ port: options.port });
}

function assetsMiddleware_(options) {
  return async (context, next) => {
    if (context.request.url.pathname.startsWith('/api/')) {
      return await next();
    }
    return await oak.send(context, context.request.url.pathname, {
      root: options.root,
      index: 'index.html',
    });
  };
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
      console.debug(data);
      break;
    }
  };
  worker.onerror = (err) => {
    console.error(`Navigation worker failed: ${err}`);
  };
  worker.postMessage({ uri, width, height, layouter });

  socket.send(JSON.stringify({ type: 'navigation.start' }));
}
