'use strict';

import * as path from '@std/path';
import Handlebars from 'handlebars';
import { RESOURCES_DIR } from '../lib/consts.js';
import { parseCommand, readAllText } from '../lib/cli.js';
import { DOT_MATRIX_FONT } from '../resources/dot_matrix/font.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DEFAULT_BOX_SIZE = '10x10';
const DEFAULT_COLOR = 'black';
const DEFAULT_LAYOUT = 'absolute';

const TEMPLATE_FILE = path.join(RESOURCES_DIR, 'dot_matrix', 'main.html.hbs');

const DOC = `
Convert a text to a html where the text is rendered with a dot matrix font.

USAGE:
  ${PROGNAME} [options] [<text>]
  ${PROGNAME} -h | --help

OPTIONS:
  -b, --box=<size>  [default: ${DEFAULT_BOX_SIZE}]
    Box size in pixels in the form "<width>x<height>".

  -c, --color=<color>  [default: ${DEFAULT_COLOR}]
    Text color.

ARGUMENTS:
  <text>
    The text to be renderer with the dot matrix font.

    Read the text from STDIN if no text is specified in the command.
`.trim();

const { options, args } = await parseCommand({
  doc: DOC,
  conv: async (name, value) => {
    switch (name) {
      case '--box':
        const [width, height] = value.split('x', 2);
        return { width: parseInt(width), height: parseInt(height) };
      case '<text>':
        if (value) {
          return value;
        }
        return await readAllText(Deno.stdin);
      default:
        return value;
    }
  },
});

Deno.exit(await run(args.text, options));

async function run(text, options) {
  const src = await Deno.readTextFile(TEMPLATE_FILE);
  const template = Handlebars.compile(src);
  console.log(template({
    font: DOT_MATRIX_FONT,
    glyphs: DOT_MATRIX_FONT.getGlyphs(text),
    box: options.box,
    color: options.color,
  }));
}
