// TODO: This is a clone of //libs/jsparser/src/syntax/actions.js.
//
// The builder module will be completely re-implemented once we finish to implement the
// jsparser::syntax module.  In the meanwhile, we have to maintain this script when the
// original script is updated.

'use strict';

import { assert } from '@std/assert';
import * as fs from '@std/fs';
import * as log from '@std/log';
import * as yaml from '@std/yaml';
import { parseCommand } from '../../../../tools/lib/cli.js';
import { setup } from '../../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

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

  // A map from a rule to the index of the rule in the `actions` list.
  // Special terms in the rule are ignored.
  const indexMap = new Map();
  for (let i = 0; i < actions.length; ++i) {
    const entry = actions[i];
    const key = removeSpecialTermsFromRule(entry.rule);
    indexMap.set(key, i);
  }

  if (args.actionsYaml && await fs.exists(args.actionsYaml)) {
    log.debug(`Overridding with existing ${args.actionsYaml}...`);
    const actionsYaml = await Deno.readTextFile(args.actionsYaml);
    const current = yaml.parse(actionsYaml);
    for (const entry of current) {
      const key = removeSpecialTermsFromRule(entry.rule);
      if (indexMap.has(key)) {
        const i = indexMap.get(key);
        actions[i].action = entry.action;
        if (entry.note) {
          actions[i].note = entry.note;
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

// Remove special terms in a given rule.
// Special terms are inserted by transpile.js and not contained in the original rule.
function removeSpecialTermsFromRule(rule) {
  let terms = rule.split(' ').filter((term) => !term.startsWith('_'));
  assert(terms.length > 0);
  if (terms[0] === '->') {
    // It's a rule for a special term.  So, there is no original rule.
    // Just return the given rule.
    return rule;
  }
  return terms.join(' ');
}
