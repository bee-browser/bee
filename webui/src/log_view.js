'use strict';

import { h, t } from './helper.js';
import Widget from './widget.js';

export default class LogView extends Widget {
  constructor(log) {
    super();
    this.log_ = log;
  }

  render() {
    this.elem_ = h('tr', {});
    this.elem_.appendChild(
      h('td', { 'class': 'log-type' }, t(this.log_.type)),
    );
    this.elem_.appendChild(
      h('td', { 'class': 'log-label' }, t(this.log_.label)),
    );
    this.elem_.appendChild(
      h('td', { 'class': 'log-file' }, t(this.log_.file)),
    );
    this.elem_.appendChild(
      h('td', { 'class': 'log-line' }, t(this.log_.line)),
    );
    this.elem_.appendChild(
      h('td', { 'class': 'log-func' }, t(this.log_.func)),
    );
    this.elem_.appendChild(
      h('td', { 'class': 'log-message' }, t(this.log_.message)),
    );
    return this.elem_;
  }
}
