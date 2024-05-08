export class LayoutBuilder {
  // public

  constructor(dom) {
    this.document_ = dom.document;
    this.assets_ = dom.assets;
    this.viewport_ = dom.viewport;
    this.instructions_ = [];
  }

  build() {
    const children = [this.build_(this.document_.root)];
    this.instructions_.push({
      type: 'layout.create_element',
      data: {
        id: 0, // must be 0
        style: {
          display: {
            outside: 'block',
            inside: 'flow_root',
          },
          positioning: 'absolute',
          box_model: {
            width: { scale: LayoutBuilder.toNumber_(1) },
            height: { scale: LayoutBuilder.toNumber_(1) },
          },
          layer: {
            z_index: { index: 0 },
          },
        },
        label: 'element-0 viewport',
        children: children.filter((id) => id !== null),
      },
    });
    this.instructions_.push({
      type: 'layout.visualize',
      data: {
        width: this.viewport_.width,
        height: this.viewport_.height,
      },
    });
    return this.instructions_;
  }

  // private

  build_(node) {
    switch (node.type) {
      case 'dom.element':
        return this.buildElement_(node);
      case 'dom.pseudo_element':
        return this.buildPseudoElement_(node);
      case 'dom.text':
        return this.buildText_(node);
      default:
        throw new Error(`Unsupported node type: ${node.type}`);
    }
  }

  buildElement_(node) {
    const style = this.buildStyle_(node.style);
    if (style === null) {
      return null;
    }
    const children = [];
    for (let child of node.childNodes) {
      children.push(this.build_(child));
    }
    this.instructions_.push({
      type: 'layout.create_element',
      data: {
        id: node.id,
        style,
        children: children.filter((id) => id !== null),
        label: '',
      },
    });
    return node.id;
  }

  buildPseudoElement_(node) {
    const style = this.buildStyle_(node.style);
    if (style === null) {
      return null;
    }
    this.instructions_.push({
      type: 'layout.create_element',
      data: {
        id: node.id,
        style,
        children: [],
        label: '',
      },
    });
    return node.id;
  }

  buildText_(node) {
    this.instructions_.push({
      type: 'layout.create_text',
      data: {
        id: node.id,
        text: node.text,
        label: '',
      },
    });
    return node.id;
  }

  buildStyle_(style) {
    if (style.display === 'none') {
      return null;
    }
    return {
      display: this.buildDisplayStyle_(style),
      positioning: this.buildPositioningStyle_(style),
      box_model: this.buildBoxModelStyle_(style),
      background: this.buildBackgroundStyle_(style),
      layer: this.buildLayerStyle_(style),
      flex: this.buildFlexStyle_(style),
      content: this.buildContentStyle_(style),
    };
  }

  buildDisplayStyle_(style) {
    const display = style['display'];

    switch (display) {
      case 'none':
        return { outside: 'none', inside: 'none' };
      case 'flex':
        return { outside: 'block', inside: 'flex' };
      case 'inline-flex':
        return { outside: 'inline', inside: 'flex' };
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
        return { outside: 'block', inside: 'flow' }; // TODO: marker + block
      case 'inline-block':
        return { outside: 'inline', inside: 'flow_root' };
      case 'inline-table':
        return { outside: 'inline', inside: 'table' };
      case 'inline-grid':
        return { outside: 'inline', inside: 'grid' };
      case 'contents':
        return { outside: 'none', inside: 'none' }; // TODO
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

    if (style['-bee-content-asset-id']) {
      inside = 'canvas';
    }

    return { outside, inside };
  }

  buildPositioningStyle_(style) {
    return style['position'];
  }

  buildBoxModelStyle_(style) {
    return {
      box_sizing: LayoutBuilder.buildKeywordValue_(style['box-sizing']),
      width: LayoutBuilder.buildLengthValue_(style['width']),
      height: LayoutBuilder.buildLengthValue_(style['height']),
      min_width: LayoutBuilder.buildLengthValue_(style['min-width']),
      min_height: LayoutBuilder.buildLengthValue_(style['min-height']),
      max_width: LayoutBuilder.buildLengthValue_(style['max-width']),
      max_height: LayoutBuilder.buildLengthValue_(style['max-height']),
      padding: LayoutBuilder.buildBoxQuad_(style, 'padding', LayoutBuilder.buildLengthValue_),
      border: LayoutBuilder.buildBorderQuad_(style),
      margin: LayoutBuilder.buildBoxQuad_(style, 'margin', LayoutBuilder.buildLengthValue_),
    };
  }

  buildBackgroundStyle_(style) {
    return {
      color: LayoutBuilder.buildColorValue_(style['background-color']),
      images: [], // TODO
    };
  }

  buildLayerStyle_(style) {
    return {
      z_index: LayoutBuilder.buildZIndex_(style['z-index']),
      offset: LayoutBuilder.buildBoxQuad_(style, '', LayoutBuilder.buildLengthValue_),
    };
  }

  buildFlexStyle_(style) {
    return {
      direction: LayoutBuilder.buildKeywordValue_(style['flex-direction']),
      wrap: LayoutBuilder.buildKeywordValue_(style['flex-wrap']),
      order: LayoutBuilder.buildIntegerValue_(style['order']),
      grow: LayoutBuilder.buildDecimalValue_(style['flex-grow']),
      shrink: LayoutBuilder.buildDecimalValue_(style['flex-shrink']),
      basis: LayoutBuilder.buildLengthValue_(style['flex-basis']),
    };
  }

  buildContentStyle_(style) {
    const id = style['-bee-content-asset-id'];
    if (id === undefined) {
      return undefined;
    }
    const asset = this.assets_[id];
    return {
      asset: {
        id,
        size: {
          pixel: [asset.width, asset.height],
        },
      },
    };
  }

  static buildZIndex_(str) {
    if (str === 'auto') {
      return 'auto';
    }
    return { index: parseInt(str) };
  }

  static buildBoxQuad_(style, name, func) {
    let prefix = `${name}-`;
    if (prefix === '-') {
      prefix = '';
    }
    return [
      func(style[`${prefix}top`]),
      func(style[`${prefix}right`]),
      func(style[`${prefix}bottom`]),
      func(style[`${prefix}left`]),
    ];
  }

  static buildBorderQuad_(style) {
    return [
      LayoutBuilder.buildBorderStyle_(style, 'top'),
      LayoutBuilder.buildBorderStyle_(style, 'right'),
      LayoutBuilder.buildBorderStyle_(style, 'bottom'),
      LayoutBuilder.buildBorderStyle_(style, 'left'),
    ];
  }

  static buildBorderStyle_(style_, edge) {
    const style = LayoutBuilder.buildKeywordValue_(style_[`border-${edge}-style`]);
    const width = LayoutBuilder.toNumber_(parseFloat(style_[`border-${edge}-width`]));
    const color = LayoutBuilder.buildColorValue_(style_[`border-${edge}-color`]);
    return { style, width, color };
  }

  static buildLengthValue_(str) {
    const s = str.trim();
    if (s.startsWith('calc(')) {
      return { calc: s.slice(5, -1) };
    }
    const n = parseFloat(s);
    if (isNaN(n)) {
      // TODO: check the keyword value
      return LayoutBuilder.buildKeywordValue_(s);
    }
    if (s.endsWith('%')) {
      return { scale: LayoutBuilder.toNumber_(n / 100.0) };
    }
    // TODO: check the unit
    return { pixel: LayoutBuilder.toNumber_(n) };
  }

  static buildKeywordValue_(str) {
    return str.replace(/-/g, '_');
  }

  static buildColorValue_(str) {
    const RE_RGB = /^rgb\s*\(\s*(.+)\s*,\s*(.+)\s*,\s*(.+)\s*\)$/i;
    const RE_RGBA = /^rgba\s*\(\s*(.+)\s*,\s*(.+)\s*,\s*(.+)\s*\,\s*(.+)\s*\)$/i;

    if (str.match(RE_RGB)) {
      return [
        parseInt(RegExp.$1),
        parseInt(RegExp.$2),
        parseInt(RegExp.$3),
        255,
      ];
    }
    if (str.match(RE_RGBA)) {
      return [
        parseInt(RegExp.$1),
        parseInt(RegExp.$2),
        parseInt(RegExp.$3),
        Math.round(parseFloat(RegExp.$4) * 255),
      ];
    }
    return [0, 0, 0, 0]; // transparent, black
  }

  static buildIntegerValue_(str) {
    return parseInt(str);
  }

  static buildDecimalValue_(str) {
    return LayoutBuilder.toNumber_(parseFloat(str));
  }

  static toNumber_(n) { // f32
    return n;
  }
}
