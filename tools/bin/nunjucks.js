'use strict';

import * as fs from '@std/fs';
import * as path from '@std/path';
import * as changeCase from 'change-case';
import nunjucks from 'nunjucks';
import { parseCommand, readAllText } from '../lib/cli.js';
import { PROJ_DIR } from '../lib/consts.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Render a Nunjucks template file with an input data.

USAGE:
  ${PROGNAME} [options] <template> [<data>]
  ${PROGNAME} -h | --help

OPTIONS:
  --escape
    Enable HTML-escape.

  -p, --partials-dir <partials-dir>
    Load partial template files in <partial-dir>.  Files in descendant folders won't loaded.

    Name of every partial template file must start with '_' and it can be used in other templates
    without the '_'.  For example, \`_partial.html\` template file can be rendered with
    \`{{< partial.html}}\`.

ARGUMENTS:
  <template>
    The path to the template file to use.

  <data>
    A data source of the input object.

DESCRIPTION:
  This command processes the template function in the strict mode and stops the processing if
  identifies used in the template are undefined.

Custom @data:
  @command
    The command that generated the source file.  The JSON string of the input object is not
    included if it's read from STDIN.

  @template
    Relatie path to the template file from the project root.

FILTERS:
  * npm:change-case

TESTS:
  * startsWith

EXAMPLES:
  The following commands output the same result:
    echo '{ "name": "value1" }' | ${PROGNAME} template.njk
    ${PROGNAME} template.njk '{ "name": "value2" }'
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

Deno.exit(await run(args, options));

async function run(args, options) {
  const template = await Deno.readTextFile(args.template);
  const data = await loadJson(args.data, options);
  const env = new nunjucks.Environment(new nunjucks.FileSystemLoader(options.partialDirs), {
    autoescape: options.escape,
  });
  registerFilters(env);
  registerTests(env);
  console.log(
    env.renderString(template, {
      data,
      command: `${PROGNAME} ${Deno.args.join(' ')}`,
      template: path.relative(PROJ_DIR, args.template),
    }).trim(),
  );
  return 0;
}

function registerFilters(env) {
  for (var name in changeCase) {
    env.addFilter(name, changeCase[name]);
  }
}

function registerTests(env) {
  env.addTest('startsWith', (value, prefix) => {
    if (typeof value !== 'string') {
      return false;
    }
    return value.startsWith(prefix);
  });
}

async function loadJson(data, options) {
  if (data === null) {
    return JSON.parse(await readAllText(Deno.stdin));
  }
  if (data.length === 0) {
    return {};
  }
  return JSON.parse(data);
}
