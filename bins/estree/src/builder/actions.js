'use strict';

import * as fs from 'https://deno.land/std@0.224.0/fs/mod.ts';
import * as log from 'https://deno.land/std@0.224.0/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.224.0/yaml/mod.ts';
import { parseCommand } from '../../../../tools/lib/cli.js';
import { setup } from '../../../../tools/lib/log.js';

const PROGNAME = 'actions.js';

const DOC = `
Usage:
  ${PROGNAME} <lalr.json> [<actions.yaml>]
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.
`.trim();

const { cmds, options, args } = await parseCommand({
  doc: DOC,
});

const YAML_OPTIONS = {
  lineWidth: 99,
};

if (options.debug) {
  setup(PROGNAME, 'DEBUG');
} else {
  setup(PROGNAME, 'INFO');
}

Deno.exit(await main(args, options));

async function main(args, options) {
  log.debug(`Loading ${args.lalrJson}...`);
  const lalrJson = await Deno.readTextFile(args.lalrJson);
  const lalrSpec = JSON.parse(lalrJson);

  const actions = lalrSpec.production_rules.map((rule) => {
    return {
      rule,
      action: null,
    };
  });

  const indexMap = new Map();
  for (let i = 0; i < actions.length; ++i) {
    const entry = actions[i];
    indexMap.set(entry.rule, i);
  }

  if (args.actionsYaml && await fs.exists(args.actionsYaml)) {
    log.debug(`Overridding with existing ${args.actionsYaml}...`);
    const actionsYaml = await Deno.readTextFile(args.actionsYaml);
    const current = yaml.parse(actionsYaml);
    for (const entry of current) {
      if (indexMap.has(entry.rule)) {
        actions[indexMap.get(entry.rule)].action = entry.action;
        if (entry.note) {
          actions[indexMap.get(entry.rule)].note = entry.note;
        }
      } else {
        log.warn(`${entry.rule} was removed`);
        log.warn(`  action: ${entry.action}`);
        if (entry.note) {
          log.warn(`  note: ${entry.note}`);
        }
      }
    }
  }

  console.log(yaml.stringify(actions, YAML_OPTIONS).trim());
}
