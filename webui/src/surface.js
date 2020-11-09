'use strict';

import { h } from './helper';
import Widget from './widget';

class Rect {
  constructor(data) {
    this.x = data.origin[0];
    this.y = data.origin[1];
    this.width = data.size[0];
    this.height = data.size[1];
  }
}

class Color {
  constructor(data) {
    this.r = data[0];
    this.g = data[1];
    this.b = data[2];
    this.a = data[3];
  }
}

export default class Surface extends Widget {
  constructor() {
    super();
    this.paintBoxes_ = [];  // in CSS painting order (back-to-front)
    this.selections_ = [];
    this.boxOutlines_ = {};
  }

  start({ width, height }) {
    this.elem_.style.width = width + 'px';
    this.elem_.style.height = height + 'px';
  }

  fillRect({ layout_id, rect, color }) {
    this.append_(new FillPaintBox(layout_id, rect, color));
  }

  drawBorder({ layout_id, rect, border }) {
    this.append_(new BorderPaintBox(layout_id, rect, border));
  }

  drawWidget({ layout_id, widget, rect, clip }) {
    this.append_(new WidgetPaintBox(layout_id, widget, rect, clip));
  }

  drawTiles({ layout_id, widget, rect, clip }) {
    this.append_(new TilePaintBox(layout_id, widget, rect, clip));
  }

  end() {
  }

  selectObject(layoutId) {
    this.deselectAll();
    const box = this.findPaintBoxByLayoutId_(layoutId);
    // TODO: there may be multiple boxes having the same layoutId
    if (box === undefined) {
      return;
    }
    this.select(box);
  }

  select(box) {
    this.selections_.push(box);
    box.select();
    box.scrollIntoView();
  }

  deselectAll() {
    while (this.selections_.length > 0) {
      const selected =  this.selections_.pop();
      selected.deselect();
    }
  }

  targetObject(layoutId) {
    if (this.selections_.length > 0) {
      return;
    }
    const box = this.findPaintBoxByLayoutId_(layoutId);
    // TODO: there may be multiple boxes having the same layoutId
    if (box === undefined) {
      return;
    }
    box.scrollIntoView();
  }

  // Widget

  render() {
    this.elem_ = h('div', { class: 'surface' });

    for (const eventType of ['click', 'mouseover', 'mouseout']) {
      this.elem_.addEventListener(eventType, (event) => {
        const box = this.findPaintBoxByElement_(event.target);
        if (box === undefined) {
          return;
        }
        event.userdata = { layoutId: box.layoutId };
        this.emit(event.type, event);
      });
    }

    return this.elem_;
  }

  clear() {
    super.clear();
    this.paintBoxes_ = [];
    this.selections_ = [];
    this.boxOutlines_ = {};
  }

  // Private Methods

  append_(box) {
    this.paintBoxes_.push(box)
    this.elem_.appendChild(box.render());
  }

  findPaintBoxByElement_(elem) {
    return this.paintBoxes_.find((box) => box.elem_ === elem);
  }

  findPaintBoxByLayoutId_(layoutId) {
    return this.paintBoxes_.find((box) => box.layoutId === layoutId);
  }
}

class PaintBox extends Widget {
  constructor(layoutId) {
    super();
    this.layoutId_ = layoutId;
  }

  get layoutId() {
    return this.layoutId_;
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'paint-box' });
    this.renderStyle_(this.elem_.style);
    return this.elem_;
  }

  // Protected Methods

  static setPositionAndSize_(style, rect) {
    style.top = rect.y + 'px';
    style.left = rect.x + 'px';
    style.width = rect.width + 'px';
    style.height = rect.height + 'px';
  }

  static convertColor_(color) {
    if (color.a === 0) {
      return "transparent";
    }
    if (color.a === undefined || color.a === 255) {
      return `rgb(${color.r},${color.g},${color.b})`;
    }
    return `rgba(${color.r},${color.g},${color.b},${color.a/255.0})`
  }

  renderStyle_(style) {
    throw new Error('must be overridden');
  }
}

class FillPaintBox extends PaintBox {
  constructor(layoutId, rect, color) {
    super(layoutId);
    this.rect_ = new Rect(rect);
    this.color_ = new Color(color);
  }

  // PaintBox

  renderStyle_(style) {
    PaintBox.setPositionAndSize_(style, this.rect_);
    style.backgroundColor = PaintBox.convertColor_(this.color_);
  }
}

const EDGES_ = ['top', 'right', 'bottom', 'left'];

class BorderPaintBox extends PaintBox {
  constructor(layoutId, rect, border) {
    super(layoutId);
    this.rect_ = new Rect(rect);
    this.border_ = border;
    for (const edge of EDGES_) {
      this.border_[edge].color = new Color(this.border_[edge].color)
    }
  }

  // PaintBox

  renderStyle_(style) {
    PaintBox.setPositionAndSize_(style, this.rect_);
    for (const edge of EDGES_) {
      style[`border-${edge}-width`] = this.border_[edge].width + 'px';
      style[`border-${edge}-style`] = this.border_[edge].style;
      style[`border-${edge}-color`] =
        PaintBox.convertColor_(this.border_[edge].color);
    }
  }
}

class WidgetPaintBox extends PaintBox {
  constructor(layoutId, widget, rect, clip) {
    super(layoutId);
    this.widget_ = widget;
    this.rect_ = new Rect(rect);
    this.clip_ = new Rect(clip);
  }

  // PaintBox

  renderStyle_(style) {
    PaintBox.setPositionAndSize_(style, this.rect_);
    style.backgroundImage = `url('${this.widget_}')`;
    style.backgroundSize = `${this.rect_.width}px ${this.rect_.height}px`;
    if (this.clip_) {
      const top = this.clip_.y - this.rect_.y;
      const left = this.clip_.x - this.rect_.x;
      const bottom = top + this.clip_.height;
      const right = left + this.clip_.width;
      style.clip = `rect(${top}px, ${right}px, ${bottom}px, ${left}px)`;
    }
  }
}

class TilePaintBox extends PaintBox {
  constructor(layoutId, widget, rect, clip) {
    super(layoutId);
    this.widget_ = widget;
    this.rect_ = new Rect(rect);
    this.clip_ = new Rect(clip);
  }

  // PaintBox

  renderStyle_(style) {
    PaintBox.setPositionAndSize_(style, this.clip_);
    style.backgroundImage = `url('${this.widget_}')`;
    style.backgroundOrigin = 'border-box';
    style.backgroundPosition =
      `${this.rect_.x - this.clip_.x}px ${this.rect_.y - this.clip_.y}px`;
    style.backgroundSize =
      `${this.rect_.width}px ${this.rect_.height}px`;
    style.backgroundRepeat = 'repeat';
  }
}
