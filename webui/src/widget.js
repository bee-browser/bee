'use strict';

import { EventEmitter } from 'https://deno.land/std@0.85.0/node/events.ts';

export default class Widget extends EventEmitter {
  constructor() {
    super()
    this.elem_ = null;
  }

  render() {
    throw new Error('must be override');
  }

  clear() {
    if (!this.hasRendered()) {
      return;
    }
    this.elem_.innerHTML = '';
  }

  show() {
    if (!this.hasRendered()) {
      return;
    }
    this.elem_.classList.remove('hide');
  }

  hide() {
    if (!this.hasRendered()) {
      return;
    }
    this.elem_.classList.add('hide');
  }

  select() {
    if (!this.hasRendered()) {
      return;
    }
    this.elem_.classList.add('selected');
  }

  deselect() {
    if (!this.hasRendered()) {
      return;
    }
    this.elem_.classList.remove('selected');
  }

  scrollIntoView() {
    if (!this.hasRendered()) {
      return;
    }
    this.elem_.scrollIntoViewIfNeeded(true);  // works only on Chrome
  }

  hasRendered() {
    return this.elem_ !== null;
  }
}
