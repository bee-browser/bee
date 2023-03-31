import * as log from "https://deno.land/std@0.181.0/log/mod.ts";

class StderrHandler extends log.handlers.ConsoleHandler {
  log(msg) {
    const encoder = new TextEncoder('utf-8');
    Deno.stderr.writeSync(encoder.encode(msg + '\n'));
  }
}

export async function setup(level) {
  await log.setup({
    handlers: {
      stderr: new StderrHandler('DEBUG'),
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
