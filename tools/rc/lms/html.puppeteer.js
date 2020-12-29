'use strict';

if ($OPTIONS.debug) {
  debugger;
}

let nextNodeId = 1;

function getNextNodeId() {
  const id = nextNodeId++;
  return id;
}

let nextWidgetId = 1;

function getNextWidgetId() {
  const id = nextWidgetId++;
  return id;
}

function snakeCase(str) {
  return str.replace(/-/g, '_');
}

function toNumber(n) {  // f32
  return n;
}

function scanDisplayStyle(styleMap) {
  const display = styleMap.get('display').toString();

  switch (display) {
  case 'none':
    return { outside: 'none', inside: 'none' };
  case 'table':
    return { outside: 'block', inside: 'table' };
  case 'table-caption':
    return { outside: 'table_caption', inside: 'flow_root' };
  case 'table-header-group':
    return { outside: 'table_header_group', inside: 'table_row_group' };
  case 'table-footer-group':
    return { outside: 'table_footer_group', inside: 'table_row_group' };
  case 'table-row-group':
    return { outside: 'table_row_group', inside: 'table_row_group' };
  case 'table-row':
    return { outside: 'table_row', inside: 'table_row' };
  case 'table-column-group':
    return { outside: 'table_column_group', inside: 'flow_column_group' };
  case 'table-column':
    return { outside: 'table_column', inside: 'none' };
  case 'table-cell':
    return { outside: 'table_cell', inside: 'flow_root' };
  case 'list-item':
    return { outside: 'block', inside: 'flow' };  // TODO: marker + block
  case 'inline-block':
    return { outside: 'inline', inside: 'flow_root' };
  case 'inline-table':
    return { outside: 'inline', inside: 'table' };
  case 'inline-flex':
    return { outside: 'inline', inside: 'flex' };
  case 'inline-grid':
    return { outside: 'inline', inside: 'grid' };
  case 'contents':
    return { outside: 'none', inside: 'none' };  // TODO
  }

  const components = display.split(' ');

  let outside = 'inline';
  if (components.includes('block')) {
    outside = 'block';
  } else if (components.includes('inline')) {
    outside = 'inline';
  }

  let inside = 'flow';
  if (components.includes('flow')) {
    inside = 'flow';
  } else if (components.includes('flow-root')) {
    inside = 'flow_root';
  } else if (components.includes('table')) {
    inside = 'table';
  } else if (components.includes('flex')) {
    inside = 'flex';
  } else if (components.includes('grid')) {
    inside = 'grid';
  }

  return { outside, inside };
}

function scanTableCellAttributes(tr, style) {
  // There are no standard properties corresponding the rowspan and the colspan
  // attributes on the table cell element.
  style._bee_table_column_span = tr.colSpan;
  style._bee_table_row_span = tr.rowSpan;
}

function scanCSSKeywordValue(value) {
  return snakeCase(value.value);
}

function scanCSSUnitValue(value) {
  if (value.unit == 'px' || value.unit == 'number') {
      return { pixel: toNumber(value.value) };
  }
  if (value.unit == 'percent') {
    return { scale: toNumber(value.value / 100.0) };
  }
  return { not_supported: value.toString() };
}

function scanCSSMathValue(value) {
  return { calc: value.toString().slice(5, -1) }
}

function scanCSSLengthValue(value) {
  if (value instanceof CSSKeywordValue) {
    return scanCSSKeywordValue(value);
  }
  if (value instanceof CSSUnitValue) {
    return scanCSSUnitValue(value);
  }
  if (value instanceof CSSMathValue) {
    return scanCSSMathValue(value);
  }
  return { not_supported: value.toString() };
}

function scanBoxQuad(styleMap, name, scan) {
  let prefix = `${name}-`;
  if (prefix === '-') {
    prefix = '';
  }
  return [
    scan(styleMap.get(`${prefix}top`)),
    scan(styleMap.get(`${prefix}right`)),
    scan(styleMap.get(`${prefix}bottom`)),
    scan(styleMap.get(`${prefix}left`)),
  ];
}

const RE_RGB = /^rgb\s*\(\s*(.+)\s*,\s*(.+)\s*,\s*(.+)\s*\)$/i;
const RE_RGBA = /^rgba\s*\(\s*(.+)\s*,\s*(.+)\s*,\s*(.+)\s*\,\s*(.+)\s*\)$/i;

function scanCSSColor(value) {
  if (value.toString().match(RE_RGB)) {
    return [
      parseInt(RegExp.$1),
      parseInt(RegExp.$2),
      parseInt(RegExp.$3),
      255,
    ];
  }
  if (value.toString().match(RE_RGBA)) {
    return [
      parseInt(RegExp.$1),
      parseInt(RegExp.$2),
      parseInt(RegExp.$3),
      Math.round(parseFloat(RegExp.$4) * 255),
    ];
  }
  return [0, 0, 0, 0];  // transparent, black
}

function scanBorderStyle(styleMap, edge) {
  const style = scanCSSKeywordValue(styleMap.get(`border-${edge}-style`))
  const width = toNumber(styleMap.get(`border-${edge}-width`).value);
  const color = scanCSSColor(styleMap.get(`border-${edge}-color`));
  return { style, width, color };
}

function scanBorderQuad(styleMap) {
  return [
    scanBorderStyle(styleMap, 'top'),
    scanBorderStyle(styleMap, 'right'),
    scanBorderStyle(styleMap, 'bottom'),
    scanBorderStyle(styleMap, 'left'),
  ];
}

function scanBoxModelStyle(style, styleMap) {
  return {
    box_sizing: scanCSSKeywordValue(styleMap.get('box-sizing')),
    width: scanCSSLengthValue(styleMap.get('width')),
    height: scanCSSLengthValue(styleMap.get('height')),
    min_width: scanCSSLengthValue(styleMap.get('min-width')),
    min_height: scanCSSLengthValue(styleMap.get('min-height')),
    max_width: scanCSSLengthValue(styleMap.get('max-width')),
    max_height: scanCSSLengthValue(styleMap.get('max-height')),
    padding: scanBoxQuad(styleMap, 'padding', scanCSSLengthValue),
    border: scanBorderQuad(styleMap),
    margin: scanBoxQuad(styleMap, 'margin', scanCSSLengthValue),
  };
}

async function scanBackgroundStyle(style, styleMap) {
  return {
    color: scanCSSColor(styleMap.get('background-color')),
    images: await scanBackgroundImages(style, styleMap),
  };
}

function scanZIndex(value) {
  if (value.value == 'auto') {
    return 'auto';
  }
  return { index: value.value };
}

function scanLayerStyle(style, styleMap) {
  return {
    z_index: scanZIndex(styleMap.get('z-index')),
    offset: scanBoxQuad(styleMap, '', scanCSSLengthValue),
  };
}

async  function scanStyleMap(style, styleMap) {
  return {
    display: scanDisplayStyle(styleMap),
    positioning: styleMap.get('position').value,
    box_model: scanBoxModelStyle(style, styleMap),
    layer: scanLayerStyle(style, styleMap),
    background: await scanBackgroundStyle(style, styleMap),
  };
}

function scanStyle(style) {
  let result = {};

  for (let i = 0; i < style.length; ++i) {
    const prop = style[i];
    if (prop.startsWith('-webkit')) {
      // Vendor-specific properties are basically ignored, except for properties
      // handled in this block.

      // Longhand properties for border-spacing.  There are no such standard
      // properties defined in CSS specifications.
      if (prop === '-webkit-border-horizontal-spacing') {
        result._bee_border_horizontal_spacing = style.getPropertyValue(prop);
      } else if (prop === '-webkit-border-vertical-spacing') {
        result._bee_border_vertical_spacing = style.getPropertyValue(prop);
      }
    } else if (prop === 'display') {
      // skip
    } else {
      // TOOD:
      // The property can be ignored when its value is equal to the initial
      // value of beeLayoutStyle.  That improve performances of the code
      // generation and compilication.

      // Convert the property name from the kebab-case to the snake-case to make
      // it possible to access to the value with the dot notation.  That's
      // useful for some command line tools like `jq`.
      result[snakeCase(prop)] = style.getPropertyValue(prop);
    }
  }

  // Special styles
  result.background_position_x = style.backgroundPositionX;
  result.background_position_y = style.backgroundPositionY;

  return result;
}

async function scanElementStyle(element) {
  const style = window.getComputedStyle(element, null);
  const styleMap = element.computedStyleMap();
  if (styleMap.get('display') == 'none') {
    return null;
  }
  return await scanStyleMap(style, styleMap);
}

function scanPseudoElementStyle(element, pseudo) {
  const style = window.getComputedStyle(element, pseudo);
  if (style.display === 'none' || style.content === 'none') {
    return null;
  }
  return scanStyle(style);
}

function makeElementLabel(element, id, pseudo) {
  const tagName = element.tagName.toLowerCase();
  let idName = '';
  if (typeof element.id === 'string' && element.id.length > 0) {
    idName = '#' + element.id;
  }
  let classNames = '';
  if (typeof element.className === 'string' && element.className.length > 0) {
    classNames = '.' + element.className.replace(/\s+/g, '.');
  }
  if (pseudo) {
    return `pseudo-element-${id} ${tagName}${idName}${classNames}::${pseudo}`;
  }
  return `element-${id} ${tagName}${idName}${classNames}`;
}

async function scanPseudoElement(result, element, pseudo) {
  const style = scanPseudoElementStyle(element, pseudo);
  if (!style) {
    return null;
  }
  const id = getNextNodeId();
  result.push({
    type: 'layout.create_element',
    data: {
      id,
      style,
      label: makeElementLabel(element, id, pseudo),
      children: [],
    },
  });
  // TODO: CSS content property
  // TODO: Nested pseudo elements
  return id;
}

async function scanImageNaturalSize(url) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => {
      resolve({
        natural_width: img.naturalWidth,
        natural_height: img.naturalHeight,
      });
    };
    img.onerror = reject;
    img.src = url;
  });
}

async function scanImages(str) {
  const images =
    str.split(/(?:\))\s*,\s*/g).filter((image) => {
      const urlMatch = image.match(/^url\(['"]?(.*?)['"]?\)$/);
      if (urlMatch && urlMatch[1].length > 0) {
        return true;
      }
      console.error(`Unsupported image type: ${image}`);
      return false;
    });
  const promises = images.map(async (image) => {
    const urlMatch = image.match(/^url\(['"]?(.*?)['"]?\)$/);
    if (urlMatch) {
      const url = urlMatch[1];
      const { natural_width, natural_height } = await scanImageNaturalSize(url);
      return { id: getNextWidgetId(), type: 'url',
               url, natural_width, natural_height, };
    }
    return {};
  });
  return await Promise.all(promises);
}

async function scanBackgroundImages(style, styleMap) {
  if (style.backgroundImage === 'none') {
    return [];
  }
  const images = await scanImages(style.backgroundImage);
  const attachments = style.backgroundAttachment.split(/\s*,\s*/g).map(snakeCase);
  const clips = style.backgroundClip.split(/\s*,\s*/g).map(snakeCase);
  const origins = style.backgroundOrigin.split(/\s*,\s*/g).map(snakeCase);
  // TODO:
  // Simplify the code by using the backgroundPositionX and backgroundPostionY
  // properties.
  //
  // The backgroundPositionX and backgroundPositionY properties seems to be
  // supported in Chrome, but experimentally.  They return only the
  // <length-percentage> perts.  So, parsing the backgroundPosition property is
  // needed for getting the correct values.
  let positions_x = [], positions_y = [];
  style.backgroundPosition.split(/\s*,\s*/g).map((pos) => {
    let result = {
      x: { edge: 'start', offset: { scale: 0.0 }, },
      y: { edge: 'start', offset: { scale: 0.0 }, },
    };
    let prop = 'x';
    pos.split(/\s+/g).forEach((v) => {
      switch (v) {
      case 'center':
        result[prop].offset = { scale: 0.5 };
        prop = 'y'
        break;
      case 'top':
        prop = 'y';
        break;
      case 'bottom':
        result.y.edge = 'end';
        prop = 'y';
        break;
      case 'left':
        prop = 'x';
        break;
      case 'right':
        result.x.edge = 'end'
        prop = 'x';
        break;
      default:
        result[prop].offset = scanCSSUnitValue(CSSNumericValue.parse(v));
        prop = prop === 'x' ? 'y' : 'x';
        break;
      }
    });
    return result;
  }).reduce((a, v) => {
    let x = {};
    x[v.x.edge] = v.x.offset;
    let y = {};
    y[v.y.edge] = v.y.offset;
    a.positions_x.push(x);
    a.positions_y.push(y);
    return a;
  }, { positions_x, positions_y });
  const repeats = style.backgroundRepeat.split(/\s*,\s*/g);
  const repeats_x = repeats.map((repeat) => {
    const [repeat_x, repeat_y] = repeat.split(/\s+/g);
    switch (repeat_x) {
    case 'repeat-x':
      return 'repeat';
    case 'repeat-y':
      return 'no-repeat';
    default:
      return repeat_x;
    }
  }).map(snakeCase);
  const repeats_y = repeats.map((repeat) => {
    const [repeat_x, repeat_y] = repeat.split(/\s+/g);
    if (repeat_y)
      return repeat_y;
    switch (repeat_x) {
    case 'repeat-x':
      return 'no-repeat';
    case 'repeat-y':
      return 'repeat';
    default:
      return repeat_x;
    }
  }).map(snakeCase);
  const sizes = style.backgroundSize.split(/\s*,\s*/g);
  const widths = sizes.map((size) => {
    const [width, height] = size.split(/\s+/g);
    try {
      return scanCSSLengthValue(CSSNumericValue.parse(width));
    } catch (e) {
      return snakeCase(width);
    }
  });
  const heights = sizes.map((size) => {
    const [width, height] = size.split(/\s+/g);
    if (width === 'contain') {
      return 'contain';
    }
    if (width === 'cover') {
      return 'cover';
    }
    if (height === undefined) {
      return 'auto';
    }
    try {
      return scanCSSLengthValue(CSSNumericValue.parse(height));
    } catch (e) {
      return snakeCase(height);
    }
  });
  return images.map((image, i) => {
    return {
      media: {
        size: { pixel: [image.natural_width, image.natural_height] }
      },
      attachment: attachments[i],
      clip: clips[i],
      origin: origins[i],
      position_x: positions_x[i],
      position_y: positions_y[i],
      repeat_x: repeats_x[i],
      repeat_y: repeats_y[i],
      width: widths[i],
      height: heights[i],
    };
  });
}

async function scanElement(result, element) {
  const style = await scanElementStyle(element);
  if (!style) {
    return null;
  }

  const id = getNextNodeId();

  let children = [];
  children.push(await scanPseudoElement(result, element, 'before'));
  for (let i = 0; i < element.childNodes.length; ++i) {
    children.push(await scanNode(result, element.childNodes[i]));
  }
  children.push(await scanPseudoElement(result, element, 'after'));
  children = children.filter((id) => id !== null);

  const label = makeElementLabel(element, id);

  result.push({
    type: 'layout.create_element',
    data: { id, style, children, label, },
  });

  // switch (element.tagName) {
  // case 'IMG':
  //   const widget = {
  //     id: getNextWidgetId(),
  //     type: 'url',
  //     url: element.src,
  //     natural_width: element.naturalWidth,
  //     natural_height: element.naturalHeight,
  //   };
  //   result.push({
  //     type: 'embed_image',
  //     data: { id, widget, },
  //   });
  //   break;
  // }

  // if (style.background_image !== 'none') {
  //   const bgimgs = await scanBackgroundImages(style);
  //   bgimgs.forEach((bgimg) => {
  //     const { image, style } = bgimg;
  //     result.push({
  //       type: 'add_background_image',
  //       data: { id, image, style, },
  //     });
  //   });
  // }

  return id;
}

function scanText(result, textNode) {
  const id = getNextNodeId();
  result.push({
    type: 'layout.create_text',
    data: {
      id,
      text: textNode.nodeValue,
      label: `text-${id}`,
    },
  });
  return id;
}

async function scanNode(result, node) {
  // TODO: pseudo-elements
  switch (node.nodeType) {
  case Node.ELEMENT_NODE:
    return await scanElement(result, node);
  case Node.TEXT_NODE:
    return scanText(result, node);
  default:
    return null;
  }
}

// Scan nodes in the post-order traversal.
async function scanDocument(result, document) {
  result.push({
    type: 'layout.create_element',
    data: {
      id: 0,
      style: {
        display: {
          outside: 'block',
          inside: 'flow_root',
        },
        positioning: 'absolute',
        overflow_x: 'scroll',
        overflow_y: 'scroll',
        box_model: {
          width: { scale: toNumber(1), },
          height: { scale: toNumber(1), },
        },
        layer: {
          z_index: { index: 0, },
        },
      },
      label: "element-0 viewport",
      children: [
        await scanNode(result, document.documentElement),
      ].filter((id) => id !== null),
    },
  });
}

async function scan() {
  let result = [];
  // TODO: frames in the window
  await scanDocument(result, window.document);
  result.push({
    type: 'layout.visualize',
    data: {
      width: window.innerWidth,
      height: window.innerHeight,
    },
  });
  return result;
}

return new Promise(async (resolve, reject) => {
  try {
    const result = await scan();
    resolve(result);
  } catch (e) {
    reject(e);
  }
});
