'use strict';

const connect = require('connect');
const http = require('http');
const handler = require('serve-handler');
const tinylr = require('tiny-lr');
const path = require('path');
const readline = require('readline');
const ws = require('ws');
const { spawn } = require('child_process');
const consts = require('./consts');

class Middleware {
  constructor(mountPoint, middlewarePath) {
    this.mountPoint = mountPoint;
    this.path = middlewarePath;
  }

  toString() {
    return `${this.mountPoint}=${this.path}`;
  }
}

class MiddlewareList {
  constructor() {
    this.list_ = [];
  }

  * [Symbol.iterator]() {
    // Array.prototype.values is not defined in Node.js 10.x
    for (const v of this.list_) {
      yield v;
    }
  }

  toString() {
    return JSON.stringify(this.list_.map((m) => m.toString()));
  }

  static collect(pair, middlewares) {
    middlewares.list_.push(pair.split('=').map((v) => new Middleware(
      v[0], path.resolve(process.cwd(), v[1]))));
    return middlewares;
  }
}

class ExtensionList {
  constructor() {
    this.list_ = [];
  }

  * [Symbol.iterator]() {
    // Array.prototype.values is not defined in Node.js 10.x
    for (const v of this.list_) {
      yield v;
    }
  }

  toString() {
    return JSON.stringify(this.list_);
  }

  static collect(extension, extensions) {
    extensions.list_.push(path.resolve(process.cwd(), extention));
    return extensions;
  }
}

function check(port) {
  return new Promise((resolve, reject) => {
    const req = http.request(`http://localhost:${port}/api/check`, (res) => {
      let body = '';
      res.on('data', (chunk) => body += chunk);
      res.on('end', () => resolve(JSON.parse(body)));
    });
    req.on('error', reject);
    req.setTimeout(5000, () => reject(new Error('timed out')));
    req.end();
  });
}

function daemon(options) {
  let args = ['-p', options.port];
  for (const middleware of options.middlewares) {
    args.push('-m', middleware.toString());
  }
  if (options.debugBuild) {
    args.push('--debug-build');
  }
  const logging = options.logging ? 'inherit' : 'ignore'
  const proc = spawn('bee-webui-serve', args, {
    detached: true,
    stdio: ['ignore', logging, logging]
  });
  proc.unref();
}

function run(options) {
  process.env.PATH = `${consts.BINDIR}:${process.env.PATH}`;
  if (options.debugBuild) {
    process.env.PATH = `${consts.DEBUG_BUILD_DIR}:${process.env.PATH}`;
  } else {
    process.env.PATH = `${consts.RELEASE_BUILD_DIR}:${process.env.PATH}`;
  }

  const app = connect();

  app.use('/api/check', (req, res) => {
    const body = '{"middlewares":' + options.middlewares.toString()
      + ',"extensions":' + options.extensions.toString() + '}';
    res.writeHead(200, {
      'Content-Type': 'application/json',
      'Content-Length': body.length
    });
    res.end(body, 'utf8');
  });

  app.use('/api/stop', (req, res) => {
    res.end();
    setTimeout(() => process.exit(0), 1000);
  });

  app.use(require('connect-livereload')());

  for (const middleware of options.middlewares) {
    app.use(middleware.mountPoint, require(middleware.path));
  }

  app.use((req, res) => handler(req, res, {
    public: consts.WEBUI_ASSETS_DIR,
    cleanUrls: false,
    headers: [{
      source: '**/*',
      headers: [{ key: 'Cache-Control', value: 'no-cache' }]
    }],
  }));

  const server = http.createServer(app);

  const wss = new ws.Server({ server });
  wss.on('connection', (ws) => {
    ws.on('message', (json) => {
      const msg = JSON.parse(json);
      handleMessage(ws, msg);
    });
  });

  server.listen(options.port, () => {
    console.log(`Listening on port ${server.address().port}...`);
  });

  const lr = tinylr();
  lr.listen(undefined, () => {
    console.log(
      `Listening on port ${lr.server.address().port} for live-reloading...`);
  });

  for (const extension of options.extensions) {
    require(extension);
  }
}

function stop(port) {
  return new Promise((resolve, reject) => {
    const req = http.request(`http://localhost:${port}/api/stop`, (res) => {
      res.on('end', () => resolve());
    });
    req.on('error', reject);
    req.end();
  });
}

function handleMessage(ws, msg) {
  switch (msg.type) {
  case 'bee.navigation.go':
    handleNavigationGo(ws, msg.data);
    break;
  }
}

function handleNavigationGo(ws, { uri, viewport, remotes }) {
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

  console.log(process.env.PATH);
  let lms = null;
  if (uri.startsWith('text:')) {
    lms = spawn('bee-lms-text', ['--viewport', `${width}x${height}`, uri.slice(5)]);
  } else {
    lms = spawn('bee-lms-html', ['--viewport', `${width}x${height}`, uri]);
  }

  const lmp = spawn('bee-lmp', ['-d']);
  lms.stdout.pipe(lmp.stdin);

  readline
    .createInterface({ input: lmp.stdout })
    .on('line', (line) => ws.send(line.trim()));

  readline
    .createInterface({ input: lmp.stderr })
    .on('line', (line) => ws.send(line.trim()));
}

module.exports.MiddlewareList = MiddlewareList;
module.exports.ExtensionList = ExtensionList;
module.exports.check = check;
module.exports.daemon = daemon;
module.exports.run = run;
module.exports.stop = stop;
