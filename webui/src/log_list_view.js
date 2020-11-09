'use strict';

import { h, t } from './helper';
import Widget from './widget';
import LogView from './log_view';

export default class LogListView extends Widget {
  constructor() {
    super();
    this.logListBody_ = null;
    this.views_ = [];
  }

  render() {
    this.elem_ =
      h('div', { 'class': 'log-list-view' },
        h('table', { 'class': 'log-list-table' },
          h('thead', { 'class': 'log-list-header' },
            h('tr', {},
              h('th', {}, t('Type')),
              h('th', {}, t('Label')),
              h('th', {}, t('File')),
              h('th', {}, t('Line')),
              h('th', {}, t('Func')),
              h('th', {}, t('Message')))),
          h('tbody', { 'class': 'log-list-body' })));
    this.logListBody_ = this.elem_.querySelector('.log-list-body');
    for (const view of this.views_) {
      this.logListBody_.appendChild(view.render());
    }
    return this.elem_;
  }

  clear() {
    this.logListBody_.innerHTML = '';
    this.views_ = [];
  }

  handleMessage(msg) {
    if (msg.type !== 'log') {
      return;
    }
    const view = new LogView(msg.data);
    this.views_.push(view);
    if (this.logListBody_) {
      this.logListBody_.appendChild(view.render());
    }
  }
}
