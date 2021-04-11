'use strict';

import TreeNode from './tree_node.js';

export default class LayoutObject extends TreeNode {
  constructor({ object_id, schema }) {
    super();
    this.id_ = object_id;
    this.schema_ = schema;
    this.label_ = '';
    this.style_ = {};
    this.width_ = 0;
    this.minWidth_ = 0;
    this.maxWidth_ = 0;
    this.height_ = 0;
    this.minHeight_ = 0;
    this.maxHeight_ = 0;
    this.padding_ = [];
    this.border_ = [];
    this.margin_ = [];
    this.renderInfo_ = {};
  }

  get schema() {
    return this.schema_;
  }

  set label(label) {
    this.label_ = label;
    this.emit('set-label');
  }

  get style() {
    return this.style_;
  }

  set style(style) {
    this.style_ = style;
    this.emit('set-style');
  }

  get dimension() {
    return {
      width: this.width_,
      minWidth: this.minWidth_,
      maxWidth: this.maxWidth_,
      height: this.height_,
      minHeight: this.minHeight_,
      maxHeight: this.maxHeight_,
      padding: {
        top: this.padding_[0],
        right: this.padding_[1],
        bottom: this.padding_[2],
        left: this.padding_[3],
      },
      border: {
        top: this.border_[0],
        right: this.border_[1],
        bottom: this.border_[2],
        left: this.border_[3],
      },
      margin: {
        top: this.margin_[0],
        right: this.margin_[1],
        bottom: this.margin_[2],
        left: this.margin_[3],
      },
    };
  }

  set dimension(dim) {
    this.width_ = dim.width;
    this.minWidth_ = dim.min_width;
    this.maxWidth_ = dim.max_width;
    this.height_ = dim.height;
    this.minHeight_ = dim.min_height;
    this.maxHeight_ = dim.max_height;
    this.padding_ = dim.padding;
    this.border_ = dim.border;
    this.margin_ = dim.margin;
  }

  set height(height) {
    this.height_ = height;
  }

  get renderInfo() {
    return this.renderInfo_;
  }

  set boxes(boxes) {
    this.renderInfo_.marginBox = boxes.margin_rect;
    this.renderInfo_.borderBox = boxes.border_rect;
    this.renderInfo_.paddingBox = boxes.padding_rect;
    this.renderInfo_.contentBox = boxes.content_rect;
  }

  inspect() {
    this.scrollIntoView();
    this.select();
  }

  // TreeNode

  get id() {
    return this.id_;
  }

  get label() {
    return this.label_;
  }
}
