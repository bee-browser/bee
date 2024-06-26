'use strict';

import { h } from './helper.js';
import Widget from './widget.js';

export default class Surface extends Widget {
  constructor() {
    super();
    this.assets_ = {};
    this.boxes_ = []; // in CSS painting order (back-to-front)
    this.selections_ = [];
    this.boxOutlines_ = {};
  }

  addAsset(asset) {
    this.assets_[asset.id] = asset;
  }

  start(size) {
    this.elem_.style.width = size[0] + 'px';
    this.elem_.style.height = size[1] + 'px';
  }

  renderBox({ layout_id, rect, background, border }) {
    this.append_(new Box(this, layout_id, rect, background, border));
  }

  renderAsset({ layout_id, asset_id, rect }) {
    const asset = this.assets_[asset_id];
    if (asset === undefined) {
      console.error(`No such asset: ${asset_id}`);
      return;
    }
    this.append_(new Asset(this, layout_id, asset, rect));
  }

  end() {
  }

  selectObject(layoutId) {
    this.deselectAll();
    const box = this.findBoxByLayoutId_(layoutId);
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
      const selected = this.selections_.pop();
      selected.deselect();
    }
  }

  targetObject(layoutId) {
    if (this.selections_.length > 0) {
      return;
    }
    const box = this.findBoxByLayoutId_(layoutId);
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
        const box = this.findBoxByElement_(event.target);
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
    this.boxes_ = [];
    this.selections_ = [];
    this.boxOutlines_ = {};
  }

  // Private Methods

  append_(box) {
    this.boxes_.push(box);
    this.elem_.appendChild(box.render());
  }

  findBoxByElement_(elem) {
    return this.boxes_.find((box) => box.elem_ === elem);
  }

  findBoxByLayoutId_(layoutId) {
    return this.boxes_.find((box) => box.layoutId === layoutId);
  }
}

class Box extends Widget {
  constructor(surface, layoutId, rect, background, border) {
    super();
    this.surface_ = surface;
    this.layoutId_ = layoutId;
    this.rect_ = rect;
    this.background_ = background;
    this.border_ = border;
  }

  get layoutId() {
    return this.layoutId_;
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'box' });
    this.renderStyle_(this.elem_.style);
    return this.elem_;
  }

  // Private Methods

  static setPositionAndSize_(style, rect) {
    style.top = rect[1] + 'px';
    style.left = rect[0] + 'px';
    style.width = rect[2] + 'px';
    style.height = rect[3] + 'px';
  }

  static convertColor_(color) {
    if (color[3] === 0) {
      return 'transparent';
    }
    if (color[3] === 255) {
      return `rgb(${color[0]},${color[1]},${color[2]})`;
    }
    return `rgba(${color[0]},${color[1]},${color[2]},${color[3] / 255.0})`;
  }

  renderStyle_(style) {
    Box.setPositionAndSize_(style, this.rect_);
    if (this.background_) {
      style.backgroundColor = Box.convertColor_(this.background_.color);
      if (this.background_.images) {
        const images = this.background_.images.map((image) => {
          const asset = this.surface_.assets_[image.asset.id];
          if (asset !== undefined) {
            return `url(${asset.dataUrl})`;
          } else {
            return 'url(data:,)';
          }
        });
        style.backgroundImage = images.join(',');
        // TODO
      }
    }
    if (this.border_) {
      const EDGES_ = ['top', 'right', 'bottom', 'left'];
      for (let i = 0; i < EDGES_.length; ++i) {
        if (this.border_[i]) {
          style[`border-${EDGES_[i]}-width`] = this.border_[i].width + 'px';
          style[`border-${EDGES_[i]}-style`] = this.border_[i].style;
          style[`border-${EDGES_[i]}-color`] = Box.convertColor_(this.border_[i].color);
        }
      }
    }
  }
}

class Asset extends Box {
  constructor(surface, layoutId, asset, rect) {
    super(surface, layoutId, rect);
    this.asset_ = asset;
  }

  // Widget

  render() {
    switch (this.asset_.type) {
      case 'image':
      case 'canvas':
        return this.renderImage_();
      default:
        console.error(`Unknown asset type: ${asset.type}`);
        return;
    }
  }

  renderImage_() {
    this.elem_ = h('img', { 'class': 'asset box', 'src': this.asset_.url });
    this.renderStyle_(this.elem_.style);
    return this.elem_;
  }
}
