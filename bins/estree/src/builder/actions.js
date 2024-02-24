'use strict';

import * as fs from 'https://deno.land/std@0.220.1/fs/mod.ts';
import * as log from 'https://deno.land/std@0.220.1/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.220.1/yaml/mod.ts';
import { parseCommand } from '../../../../tools/lib/cli.js';
import { setup } from '../../../../tools/lib/log.js';

const PROGNAME = 'actions.js';

const DOC = `
Usage:
  ${PROGNAME} collect <actions.yaml>
  ${PROGNAME} update <lalr.json> [<actions.yaml>]
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

async function collect(args, options) {
  log.debug(`Loading ${args.actionsYaml}...`);
  const actionsYaml = await Deno.readTextFile(args.actionsYaml);
  const spec = yaml.parse(actionsYaml);
  let actions = [];
  let entries = [];
  let actionMap = new Map();
  for (const entry of spec) {
    if (entry.action === null) {
      entries.push({
        rule: entry.rule,
        action: null,
      });
      continue;
    }
    let action = entry.action.trim();
    if (actionMap.has(action)) {
      entries.push({
        rule: entry.rule,
        action: actionMap.get(action),
      });
      continue;
    }
    let actionName = `action${actions.length}`;
    actions.push({
      name: actionName,
      code: action,
    });
    actionMap.set(action, actionName);
    entries.push({
      rule: entry.rule,
      action: actionName,
    });
  }
  console.log(JSON.stringify({ actions, entries }));
}

async function update(args, options) {
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

  const removed = [];
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
        removed.push(entry);
      }
    }
  }

  console.log(yaml.stringify(actions, YAML_OPTIONS).trim());

  if (removed.length > 0) {
    // std/yaml always uses '\n' as EOL.
    const EOL = '\n';
    const s = yaml.stringify(removed, YAML_OPTIONS).trim();
    console.log(s.split(EOL).map((line) => `# ${line}`).join(EOL));
  }
}

if (options.debug) {
  setup(PROGNAME, 'DEBUG');
} else {
  setup(PROGNAME, 'INFO');
}

switch (cmds[0]) {
  case 'collect':
    Deno.exit(await collect(args, options));
  case 'update':
    Deno.exit(await update(args, options));
}
