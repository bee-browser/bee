'use strict';

const _ = require('lodash');
const path = require('path');
const punycode = require('punycode');
const readline = require('readline');
const consts = require('../consts');

// helpers

const NUM_ROWS_IN_GLYPH = 7;
const NUM_DOTS_IN_ROW = 5;

const GLYPHS = require(path.join(consts.RCDIR, 'lms', 'text.font.js'));

function toFont(c) {
  const cp = c.codePointAt(0);
  if (cp in GLYPHS) {
    return { char: c, glyph: GLYPHS[cp] };
  }
  return { char: c, glyph: GLYPHS[0xFFFD] };  // REPLACEMENT CHARACTER
}

let nextNodeId = 1;

function getNextNodeId() {
  const id = nextNodeId++;
  return id;
}

function toNumber(n) {  // f32
  return n;
}

// absolutely positioned boxes

function renderByPositionedBoxes(context, font, index) {
  const id = getNextNodeId();
  const width = NUM_DOTS_IN_ROW * context.box.width;
  const height = NUM_ROWS_IN_GLYPH * context.box.height;
  let left = index * NUM_DOTS_IN_ROW * context.box.width;
  left += index * context.box.width;  // letter space

  output({
    create_element: {
      id,
      style: {
        schema: {
          node: 'block',
          container: 'flow_root',
          positioning: 'absolute',
        },
        box_model: {
          width: { pixel: toNumber(width) },
          height: { pixel: toNumber(height) },
        },
        layer: {
          offset: [{ pixel: toNumber(0) }, 'auto', 'auto', { pixel: toNumber(left) }],
          z_index: { index: 0 },
        },
      },
      label: `#glyph-box-${id}.char-${font.char}`,
      children: font.glyph
        .flatMap((row, index) => renderRowByPositionedBoxes(context, id, row, index))
        .filter((id) => id !== null),
    },
  });

  return id;
}

function renderRowByPositionedBoxes(context, glyphId, row, index) {
  const top = index * context.box.height;
  return row.split('')
    .map((dot, index) => renderDotByPositionedBox(context, glyphId, top, dot, index));
}

function renderDotByPositionedBox(context, glyphId, top, dot, index) {
  if (dot === ' ') {
    return null;
  }
  const width = context.box.width;
  const height = context.box.height;
  const left = index * context.box.width;
  const id = getNextNodeId();
  output({
    create_element: {
      id,
      style: {
        schema: {
          node: 'block',
          container: 'flow_root',
          positioning: 'absolute',
        },
        box_model: {
          width: { pixel: toNumber(width) },
          height: { pixel: toNumber(height) },
        },
        background: {
          color: context.color,
        },
        layer: {
          offset: [{ pixel: toNumber(top) }, 'auto', 'auto', { pixel: toNumber(left) }],
        },
      },
      label: `#glyph-box-${glyphId}.dot-${top}-${left}`,
      children: [],
    }
  });
  return id;
}

// inline-block boxes

function renderByInlineBlockBoxes(context, font) {
  const glyphBoxId = getNextNodeId();
  const width = NUM_DOTS_IN_ROW * context.box.width;
  const height = NUM_ROWS_IN_GLYPH * context.box.height;
  return [{
    type: 'layout.append_box',
    data: {
      id: glyphBoxId,
      parent_id: context.parentId,
      label: `#glyph-box-${glyphBoxId}.char-${font.char}`,
      style: {
        display: 'inline-block',
        width: `${width}px`,
        height: `${height}px`,
        margin_left: `${context.box.width}px`,
        margin_bottom: `${context.box.height}px`
      }
    }
  }, _.map(font.glyph, _.partial(
    renderRowByInlineBlockBoxes, context, glyphBoxId))];
}

function renderRowByInlineBlockBoxes(context, glyphBoxId, row) {
  return _.map(row, _.partial(
    renderDotByInlineBlockBox, context, glyphBoxId));
}

function renderDotByInlineBlockBox(context, glyphBoxId, dot) {
  const width = context.box.width;
  const height = context.box.height;
  const color = dot === ' ' ? 'transparent' : context.color;
  return {
    type: 'layout.append_box',
    data: {
      id: getNextNodeId(),
      parent_id: glyphBoxId,
      label: `$glyph-box-${glyphBoxId}.dot`,
      style: {
        display: 'inline-block',
        width: `${width}px`,
        height: `${height}px`,
        background_color: color
      }
    }
  };
}

// floating box

function renderByFloatingBoxes(context, font) {
  const glyphBoxId = getNextNodeId();
  const width = NUM_DOTS_IN_ROW * context.box.width;
  const height = NUM_ROWS_IN_GLYPH * context.box.height;
  return [{
    type: 'layout.append_box',
    data: {
      id: glyphBoxId,
      parent_id: context.parentId,
      label: `#glyph-box-${glyphBoxId}.char-${font.char}`,
      style: {
        float: 'left',
        width: `${width}px`,
        height: `${height}px`,
        margin_left: `${context.box.width}px`,
        margin_bottom: `${context.box.height}px`
      }
    }
  }, _.map(font.glyph, _.partial(
    renderRowByFloatingBoxes, context, glyphBoxId))];
}

function renderRowByFloatingBoxes(context, glyphBoxId, row) {
  return _.map(row, _.partial(
    renderDotByFloatingBox, context, glyphBoxId));
}

function renderDotByFloatingBox(context, glyphBoxId, dot) {
  const width = context.box.width;
  const height = context.box.height;
  const color = dot === ' ' ? 'transparent' : context.color;
  return {
    type: 'layout.append_box',
    data: {
      id: getNextNodeId(),
      parent_id: glyphBoxId,
      label: `$glyph-box-${glyphBoxId}.dot`,
      style: {
        float: 'left',
        width: `${width}px`,
        height: `${height}px`,
        background_color: color
      }
    }
  };
}

function output(cmd) {
  process.stdout.write(JSON.stringify(cmd));
  process.stdout.write('\n');
}

const RENDERERS = {
  absolute: renderByPositionedBoxes,
  inline: renderByInlineBlockBoxes,
  float: renderByFloatingBoxes
};

function render(text, renderer, context) {
  output({
    create_element: {
      id: 0,
      style: {
        schema: {
          node: 'block',
          container: 'flow_root',
          positioning: 'absolute',
          overflow_x: 'scroll',
          overflow_y: 'scroll',
        },
        box_model: {
          width: { scale: toNumber(1), },
          height: { scale: toNumber(1), },
        },
        layer: {
          z_index: { index: 0, },
        },
      },
      label: "#viewport",
      children: text.split('')
        .map((c) => toFont(c))
        .map((font, index) => renderer(context, font, index))
        .filter((id) => id !== null),
    },
  });

  output({
    visualize: {
      width: context.viewport.width,
      height: context.viewport.height,
    },
  });
}

// entry point

function run(text, options, logger) {
  const renderer = RENDERERS[options.mode];

  const context = {
    box: options.box,
    color: options.color,
    viewport: options.viewport,
  };

  return new Promise((resolve, reject) => {
    if (text !== undefined) {
      render(text, renderer, context);
      resolve();
    } else {
      const rl = readline.createInterface({
        input: process.stdin
      });

      rl.on('line', (text) => {
        render(text, renderer, context);
      });

      rl.on('close', resolve);
    }
  });
}

// exports

module.exports.run = run;
