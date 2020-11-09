'use strict';

import { h } from './helper';
import Widget from './widget';

export default class TreeView extends Widget {
  constructor() {
    super();
    this.root_ = null;
    this.selections_ = [];
  }

  render() {
    this.elem_ =
      h('div', { class: 'tree-view' },
        h('div', { class: 'tree-node-container' }));

    this.containerElem_ =
      this.elem_.querySelector('.tree-node-container');

    return this.elem_;
  }

  clear() {
    if (!this.hasRendered()) {
      return;
    }
    this.containerElem_.innerHTML = '';
    this.root_ = null;
    this.selections_ = [];
  }

  insertBefore(node, parent, sibling) {
    if (!this.hasRendered()) {
      return;
    }
    node.on('click', (target, event) => this.emit('click', target, event));
    node.on('mouseover', (n) => this.emit('mouseover', n));
    node.on('mouseout', (n) => this.emit('mouseout', n));

    if (parent === null) {
      this.root_ = node;
      this.containerElem_.appendChild(node.render());
      return;
    }

    parent.insertBefore(node, sibling);
  }

  select(node) {
    if (!this.hasRendered()) {
      return;
    }
    this.selections_.push(node);
    node.select();
    node.scrollIntoView();
  }

  deselectAll() {
    if (!this.hasRendered()) {
      return;
    }
    while (this.selections_.length > 0) {
      const selected =  this.selections_.pop();
      selected.deselect();
    }
  }

  target(node) {
    node.target();
    if (this.selections_.length === 0) {
      node.scrollIntoView();
    }
  }

  untarget(node) {
    node.untarget();
  }
}
