'use strict';

import * as fs from '@std/fs';
import * as path from '@std/path';
import { pascalCase, snakeCase } from 'change-case';
import Handlebars from 'handlebars';
import { parseCommand, readAllText } from '../lib/cli.js';
import { PROJ_DIR } from '../lib/consts.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Generate a source file using a Handlebars template file and an input object.

USAGE:
  ${PROGNAME} [options] <template> [<input>]
  ${PROGNAME} -h | --help

OPTIONS:
  --no-escape
    Disable to HTML-escape results of expressions in mustaches.

  -p, --partials-dir <partials-dir>
    Load partial template files in <partial-dir>.  Files in descendant folders won't loaded.

    Name of every partial template file must start with '_' and it can be used in other templates
    without the '_'.  For example, \`_partial.html\` template file can be rendered with
    \`{{< partial.html}}\`.

  --input-inline
    A JSON string as the input object is specified in the command line.

  --input-stdin
    Read a JSON string as the input object from STDIN.

  --deps <target>
    Print a Makefile which contains dependencies of the <target>.

ARGUMENTS:
  <template>
    The path to the template file to use.

  <input>
    A data source of the input object.

DESCRIPTION:
  This command processes the template function in the strict mode and stops the processing if
  identifies used in the template are undefined.

CUSTOM DATA:
  @command
    The command that generated the source file.  The JSON string of the input object is not
    included if it's read from STDIN.

  @template
    Relatie path to the template file from the project root.
`.trim();

const HELPERS = {
  eq: (a, b) => a === b,

  join: (array, sep) => {
    return array.join(sep);
  },

  length: (v) => {
    if (typeof v.length === 'number') {
      return v.length;
    } else {
      return Object.keys(v).length;
    }
  },

  padStart: (v, n, pad) => {
    return v.toString().padStart(n, pad);
  },

  padEnd: (v, n, pad) => {
    return v.toString().padEnd(n, pad);
  },

  json: JSON.stringify,

  pascalCase,
  snakeCase,
  lowerCase: (str) => str.toLowerCase(),
  upperCase: (str) => str.toUpperCase(),

  escapeForRust: (str) => {
    const CHARMAP = {
      '\0': '\\0',
      '\n': '\\n',
      '\r': '\\r',
      '\\': '\\\\',
      "'": "\\'",
      '"': '\\"',
    };
    let escaped = '';
    for (let i = 0; i < str.length; ++i) {
      const ch = str[i];
      if (ch in CHARMAP) {
        escaped += CHARMAP[ch];
      } else {
        escaped += ch;
      }
    }
    return escaped;
  },

  escapeUnicodeForRust: (str) => {
    let escaped = '';
    let i = 0;
    while (i < str.length) {
      const cp = str.codePointAt(i);
      const hex = cp.toString(16).toUpperCase();
      escaped += `\\u{${hex}}`;
      i += String.fromCodePoint(cp).length;
    }
    return escaped;
  },
};

const { options, args } = await parseCommand({
  doc: DOC + `

HELPERS:
${Object.keys(HELPERS).map((k) => '  * ' + k).join('\n')}
`,
});

Deno.exit(await run(args, options));

async function run(args, options) {
  const src = await Deno.readTextFile(args.template);
  const input = await loadJson(args.input, options);
  Handlebars.registerHelper(HELPERS);
  if (options.deps) {
    return await depsgen(src, input, args, options);
  }
  await codegen(src, input, args, options);
}

async function codegen(src, input, args, options) {
  if (options.partialsDir) {
    await loadPartials(options.partialsDir, (name, path) => {
      let partial = null;
      Handlebars.registerPartial(name, () => {
        if (partial === null) {
          partial = Deno.readTextFileSync(path);
        }
        return partial;
      });
    });
  }
  const template = Handlebars.compile(src, {
    noEscape: options.noEscape,
    strict: true,
  });
  console.log(
    template(input, {
      data: {
        command: `${PROGNAME} ${Deno.args.join(' ')}`,
        template: path.relative(PROJ_DIR, args.template),
      },
    }).trim(),
  );
}

async function depsgen(src, input, args, options) {
  const deps = new Set();
  if (options.partialsDir) {
    await loadPartials(options.partialsDir, (name, path) => {
      Handlebars.registerPartial(name, () => {
        deps.add(path);
        return '';
      });
    });
  }
  const template = Handlebars.compile(src, {
    noEscape: options.noEscape,
    strict: true,
  });
  template(input, {
    data: {
      command: `${PROGNAME} ${Deno.args.join(' ')}`,
      template: path.relative(PROJ_DIR, args.template),
    },
  });
  console.log(`${options.deps}: ${Array.from(deps).join(' ')}`);
}

async function loadJson(input, options) {
  if (options.inputStdin) {
    return JSON.parse(await readAllText(Deno.stdin));
  }
  if (!input) {
    return {};
  }
  if (options.inputInline) {
    return JSON.parse(input);
  }
  return JSON.parse(await Deno.readTextFile(input));
}

async function loadPartials(dir, register) {
  const options = {
    maxDepth: 1,
  };
  for await (const entry of fs.walk(dir, options)) {
    if (!entry.isFile) {
      continue;
    }
    const filename = path.basename(entry.path);
    if (!filename.startsWith('_')) {
      continue;
    }
    const name = filename.slice(1);
    register(name, entry.path);
  }
}
