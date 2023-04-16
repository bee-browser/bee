import * as path from "https://deno.land/std@0.183.0/path/mod.ts";
import * as yaml from 'https://deno.land/std@0.183.0/encoding/yaml.ts';
import { pascalCase } from 'https://deno.land/x/case@2.1.1/mod.ts';
import { Command } from "https://deno.land/x/cliffy@v0.25.7/command/mod.ts";
import Handlebars from 'npm:handlebars@4.7.7';
import * as log from '../../../../tools/lib/log.js';

const PROGNAME = path.basename(Deno.mainModule);

const rulesCommand = new Command()
  .description('Collect rules for tokens')
  .arguments('<name> <tokens...>')
  .action(collectRules);

const tagsCommand = new Command()
  .description('Collect tag names')
  .action(collectTags);

await new Command()
  .name(PROGNAME)
  .globalOption(
    '-s, --spec-yaml=<spec.yaml>',
    'Path to spec.yaml.',
    { required: true },
  )
  .globalOption(
    '-l, --log-level=<level>',
    'Log level',
    { default: undefined },
  )
  .command('rules', rulesCommand)
  .command('tags', tagsCommand)
  .parse();

async function collectRules(options, name, ...tokens) {
  await log.setup(options.logLevel);

  const specYaml = await Deno.realPath(options.specYaml);

  log.debug(`Loading ${specYaml}...`);
  const spec = yaml.parse(await Deno.readTextFile(specYaml));
  const modes = normalize(spec);

  const data = { name, tokens: {} };
  for (let token of tokens) {
    log.debug(`Collecting rules for ${token}...`);
    let label = getLabelFromToken(token);
    data.tokens[label] = minimize(collect(spec, modes, token));
  }

  log.debug(`Generating JSON for ${name}...`);
  console.log(JSON.stringify(data));
}

async function collectTags(options) {
  await log.setup(options.logLevel);

  const specYaml = await Deno.realPath(options.specYaml);

  log.debug(`Loading ${specYaml}...`);
  const spec = yaml.parse(await Deno.readTextFile(specYaml));

  log.debug('Collecting tag names...');
  let tagNames = new Set();
  for (let mode of spec.modes) {
    for (let rule of mode.rules) {
      for (let token of rule.match.split(' ')) {
        if (!token.startsWith('<')) {
          continue;
        }
        const tagName = getTagNameFromToken(token)
        if (tagName !== '_') {
          tagNames.add(tagName);
        }
      }
    }
  }

  console.log(JSON.stringify(Array.from(tagNames).sort()));
}

function normalize(spec) {
  let modes = {};
  for (let mode of spec.modes) {
    let rules = {};
    let ids = {};
    for (let rule of mode.rules) {
      let matches = rule.match.split(' ');
      for (let match of matches) {
        rules[match] = {
          run: rule.run,
        };
        if (rule.id) {
          ids[rule.id] = rules[match];
        }
      }
    }
    modes[mode.name] = { rules, ids };
  }
  return modes;
}

function collect(spec, modes, token) {
  let collected = [];
  for (let i = 0; i < spec.modes.length; ++i) {
    const name = spec.modes[i].name;
    log.debug(`${token}: ${name}: lookup rules...`);
    const mode = modes[name];
    const [match, run] = lookupRun(mode, token, '=>');
    const rule = {
      name: pascalCase(name),
      run: render(spec, modes, mode.ids, token, run, '->'),
    };
    logRule(rule);
    collected.push(rule);
  }
  return collected;
}

function minimize(modes) {
  let reachables = {};
  let unreachables = [];
  for (let mode of modes) {
    if (mode.run === undefined) {
      unreachables.push(mode.name);
      continue;
    }
    if (mode.run in reachables) {
      reachables[mode.run].push(mode.name);
    } else {
      reachables[mode.run] = [mode.name];
    }
  }
  let result = [];
  for (let [run, names] of Object.entries(reachables)) {
    result.push({
      modes: names,
      run,
    });
  }
  if (unreachables.length > 0) {
    result.push({
      modes: unreachables,
      run: null,
    });
  }
  return result;
}

function lookupRun(mode, token, arrow) {
  let rule = mode.rules[token];
  if (rule) {
    log.debug(`${arrow} match ${token}`);
    return [token, rule.run];
  }
  let match;
  if (token.startsWith('</')) {
    match = '</_>';
    rule = mode.rules['</_>'];
  } else if (token.startsWith('<')) {
    match = '<_>';
    rule = mode.rules['<_>'];
  }
  if (rule) {
    log.debug(`${arrow} match ${match}`);
    return [match, rule.run];
  }
  rule = mode.rules['any'];
  if (rule) {
    log.debug(`${arrow} match any`);
    return ['any', rule.run];
  }
  log.debug(`${arrow} no rule matched`);
  return [undefined, undefined];
}

function render(spec, modes, ids, token, run, arrow) {
  if (run === undefined) {
    return undefined;
  }
  const hbs = Handlebars.create();
  hbs.registerHelper('use', (ref) => {
    let targetToken = token;
    if (ref.startsWith('<')) {
      const i = ref.indexOf('>');
      targetToken = ref.slice(0, i + 1);
      ref = ref.slice(i + 1);
    }
    if (ref.startsWith('@')) {
      let modeName = ref.slice(1);
      log.debug(`${arrow} using modes.${modeName}`);
      let mode = modes[modeName];
      if (mode === undefined) {
        log.error(`No such mode is defined in modes: ${modeName}`);
        Deno.exit(1);
      }
      const [match, run] = lookupRun(mode, targetToken, arrow.charAt(0) + arrow);
      if (match === undefined) {
        log.error('No rule matching the token is defined');
        Deno.exit(1);
      }
      return render(spec, modes, mode.ids, targetToken, run, arrow.charAt(0) + arrow);
    }
    if (ref.startsWith('$')) {
      let snippetName = ref.slice(1);
      log.debug(`${arrow} using snippets.${snippetName}`);
      let snippet = spec.snippets[snippetName];
      if (snippet === undefined) {
        log.error(`No such name is defined in snippets: ${snippetName}`);
        Deno.exit(1);
      }
      return render(spec, modes, ids, targetToken, snippet.code, arrow.charAt(0) + arrow);
    }
    if (ref in ids) {
      log.debug(`${arrow} using ${ref} in the mode`);
      return render(spec, modes, ids, targetToken, ids[ref].run, arrow);
    }
    log.error("Invalid reference: ${ref}");
    Deno.exit(1);
  });
  const template = hbs.compile(run.trim(), {
    noEscape: true,
    strict: true,
  });
  let tag_name = getTagNameFromToken(token);
  if (tag_name === '_') {
    tag_name = 'unknown';
  }
  return template({
    tag_name,
    TagName: pascalCase(tag_name),
  });
}

function getLabelFromToken(token) {
  if (token.startsWith('</')) {
    return 'endTag';
  }
  if (token.startsWith('<')) {
    return 'startTag';
  }
  return token;
}

function getTagNameFromToken(token) {
  if (token.startsWith('</')) {
    return token.slice(2, -1);
  }
  if (token.startsWith('<')) {
    return token.slice(1, -1);
  }
  return undefined;
}

function logRule(rule) {
  log.debug(`----- ${rule.name}`);
  if (rule.run) {
    for (let line of rule.run.split('\n')) {
      log.debug(line);
    }
  } else {
    log.debug('!!! UNDEFINED !!!');
  }
  log.debug('');
}
