import * as log from 'https://deno.land/std@0.208.0/log/mod.ts';

class StderrHandler extends log.handlers.ConsoleHandler {
  constructor(label, level) {
    super(level);
    this.label_ = label;
    this.encoder_ = new TextEncoder('utf-8');
  }

  log(msg) {
    Deno.stderr.writeSync(this.encoder_.encode(`${this.label_}: ${msg}\n`));
  }
}

export async function setup(label, level) {
  await log.setup({
    handlers: {
      stderr: new StderrHandler(label, 'DEBUG'),
    },
    loggers: {
      default: {
        level,
        handlers: ['stderr'],
      },
    },
  });
}

export function info(msg) {
  log.getLogger('default').info(msg);
}

export function warn(msg) {
  log.getLogger('default').warn(msg);
}

export function error(msg) {
  log.getLogger('default').error(msg);
}

export function debug(msg) {
  log.getLogger('default').debug(msg);
}
