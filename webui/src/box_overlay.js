'use strict';

import { h } from './helper.js';
import Widget from './widget.js';

export default class BoxOverlay extends Widget {
  constructor() {
    super();

    this.marginBox_ = new BoxOutline('margin');
    this.borderBox_ = new BoxOutline('border');
    this.paddingBox_ = new BoxOutline('padding');
    this.contentBox_ = new BoxOutline('content');
  }

  // Widget

  render() {
    this.elem_ = h(
      'div',
      { 'class': 'box-overlay' },
      this.marginBox_.render(),
      this.borderBox_.render(),
      this.paddingBox_.render(),
      this.contentBox_.render(),
    );

    return this.elem_;
  }

  clear() {
    this.hide();
  }

  show(rects) {
    this.marginBox_.show(rects.marginRect);
    this.borderBox_.show(rects.borderRect);
    this.paddingBox_.show(rects.paddingRect);
    this.contentBox_.show(rects.contentRect);
  }

  hide() {
    this.contentBox_.hide();
    this.paddingBox_.hide();
    this.borderBox_.hide();
    this.marginBox_.hide();
  }
}

const STYLE_RECT_PROP_MAP_ = {
  top: 'y',
  left: 'x',
  width: 'width',
  height: 'height',
};

class BoxOutline extends Widget {
  constructor(type) {
    super();
    this.type_ = type;
  }

  render() {
    this.elem_ = h('div', { 'class': `${this.type_}-box-outline hide` });
    return this.elem_;
  }

  show(rect) {
    Object.entries(STYLE_RECT_PROP_MAP_).forEach(([styleProp, rectProp]) => {
      this.elem_.style[styleProp] = rect[rectProp] + 'px';
    });
    super.show();
  }
}
