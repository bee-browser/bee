'use strict';

import { h, t } from '../../../../webui/src/helper.js';
import Widget from '../../../../webui/src/widget.js';

export default class Toolbar extends Widget {
  constructor() {
    super();

    this.running_ = false;

    this.actionButton_ = h('button', { id: 'action' }, t('Start'));
    this.actionButton_.userdata_ = { started: false };
    this.actionButton_.addEventListener('click', () => {
      if (this.running_) {
        this.emit('pause');
        this.actionButton_.replaceChildren('Start');
      } else {
        this.emit('start');
        this.actionButton_.replaceChildren('Pause');
      }
      this.running_ = !this.running_;
      this.nextButton_.disabled = this.running_;
    });

    this.nextButton_ = h('button', { id: 'next' }, t('Next'));
    this.nextButton_.addEventListener('click', () => this.emit('next'));

    this.resetButton_ = h('button', { id: 'reset' }, t('Reset'));
    this.resetButton_.addEventListener('click', () => this.emit('reset'));
  }

  render() {
    this.elem_ =
      h('div', { id: 'toolbar' },
        this.actionButton_,
        this.nextButton_,
        this.resetButton_);
    return this.elem_;
  }
}
