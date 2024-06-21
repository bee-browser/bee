'use strict';

export class EventEmitter {
  constructor() {
    this.listeners_ = {};
  }

  addEventListener(event, listener) {
    if (this.listeners_[event] === undefined) {
      this.listeners_[event] = [];
    }
    this.listeners_.push(listener);
  }

  emit(event, data) {
    if (this.listeners_[event] === undefined) {
      return;
    }
    for (const listener of this.listeners_[event]) {
      listener(data);
    }
  }
}
