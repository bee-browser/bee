'use strict';

import TreeNode from './tree_node';

export default class LayoutLayer extends TreeNode {
  constructor(id, owner) {
    super();
    this.id_ = id;
    this.owner_ = owner;
    this.container_ = null;
    this.newStackingContext_ = false;
    this.stackLevel_ = 0;
    this.position_ = { x: 0, y: 0 };
    this.staticPosition_ = { x: 0, y: 0 };
    this.extraOffset_ = { x: 0, y: 0 };
  }

  get owner() {
    return this.owner_;
  }

  get ownerId() {
    return this.owner_.id;
  }

  get containerId() {
    if (this.container_ !== null) {
      return this.container_.id;
    }
    return 0;
  }

  set container(container) {
    this.container_ = container;
  }

  get newStackingContext() {
    return this.newStackingContext_;
  }

  get stackLevel() {
    return this.stackLevel_;
  }

  get position() {
    return this.position_;
  }

  get staticPosition() {
    return this.staticPosition_;
  }

  get extraOffset() {
    return this.extraOffset_;
  }

  setStyle(style) {
    this.newStackingContext_ = style.new_stacking_context;
    this.stackLevel_ = style.stack_level;
  }

  setPosition(pos) {
    this.position_ = pos.position;
    this.staticPosition_ = pos.static_position;
    this.extraOffset_ = pos.extra_offset;
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
    const type = this.owner_.schema.layer;
    const ownerLabel = this.owner_.label;
    return `${type}-layer [${ownerLabel}]`;
  }
}
