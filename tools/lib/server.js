// Copyright 2018 BEE project contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
  const logging = options.logging ? 'inherit' : 'ignore'
  const proc = spawn('server-run', args, {
    detached: true,
    stdio: ['ignore', logging, logging]
  });
  proc.unref();
}

function run(options) {
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
    public: consts.OUTPUT_DIR,
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

  let lms = null;
  if (uri.startsWith('text:')) {
    lms = spawn('lms-text', ['--surface', `${width}x${height}`, uri.slice(5)]);
  } else {
    lms = spawn('lms-html', ['--viewport', `${width}x${height}`, uri]);
  }

  const lmp = spawn('lmp', ['-m']);
  lms.stdout.pipe(lmp.stdin);

  const sinks = Object
    .keys(remotes)
    .filter((remote) => ['pusher'].includes(remote))
    .map((remote) => spawn(`msg-sink-${remote}`, remotes[remote]));

  readline
    .createInterface({ input: lmp.stdout })
    .on('line', (line) => {
      ws.send(line);
      sinks.forEach((sink) => sink.stdin.write(line + '\n'));
    });

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
