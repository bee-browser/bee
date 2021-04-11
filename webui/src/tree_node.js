'use strict';

import { h, t } from './helper.js';
import Widget from './widget.js';

export default class TreeNode extends Widget {
  constructor() {
    super();
    this.depth_ = 0;
    this.nodes_ = [];
  }

  get id() {
    throw new Error('pure virtual');
  }

  get label() {
    throw new Error('pure virtual');
  }

  insertBefore(node, sibling = null) {
    node.depth_ = this.depth_ + 1;
    if (sibling === null) {
      this.nodes_.push(node);
    } else {
      const i = this.nodes_.indexOf(sibling);
      this.nodes_.splice(i, 0, node);
    }
    if (this.elem_) {
      this.containerElem_.insertBefore(
        node.render(), sibling ? sibling.elem_ : null);
    }
  }

  select() {
    this.contentElem_.classList.add('selected');
  }

  deselect() {
    this.contentElem_.classList.remove('selected');
  }

  toggleSelection() {
    this.contentElem_.classList.toggle('selected');
  }

  target() {
    this.contentElem_.classList.add('targeted');
  }

  untarget() {
    this.contentElem_.classList.remove('targeted');
  }

  // Widget

  render() {
    this.elem_ =
      h('div', { class: 'tree-node' },
        h('div', {
          class: 'tree-node-content',
          style: `padding-left: ${this.depth_ * 10 + 'px'}`
        },
          h('span', { class: 'tree-node-label' },
            t(this.label))),
        h('div', { class: 'tree-node-container' }));

    this.contentElem_ = this.elem_.querySelector('.tree-node-content');
    this.contentElem_.addEventListener('click', (event) => {
      this.emit('click', this, event);
    });
    this.contentElem_.addEventListener('dblclick', (event) => {
      event.stopPropagation();
      event.preventDefault();
      getSelection().removeAllRanges();
      this.elem_.classList.toggle('collapsed');
    });
    this.contentElem_.addEventListener('mouseover', (event) => {
      event.stopPropagation();
      event.preventDefault();
      this.emit('mouseover', this);
    });
    this.contentElem_.addEventListener('mouseout', (event) => {
      event.stopPropagation();
      event.preventDefault();
      this.emit('mouseout', this);
    });

    this.containerElem_ =
      this.elem_.querySelector('.tree-node-container');

    for (const node of this.nodes_) {
      node.depth_ = this.depth_ + 1;
      this.containerElem_.insertBefore(node.render(), null);
    }

    return this.elem_;
  }

  scrollIntoView() {
    this.contentElem_.scrollIntoViewIfNeeded(true);  // works only on Chrome
  }
}
