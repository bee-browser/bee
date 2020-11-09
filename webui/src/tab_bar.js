'use strict';

import { h, t } from './helper';
import Widget from './widget';

export default class TabBar extends Widget {
  constructor() {
    super();
    this.tabs_ = [];
  }

  get selectedTab() {
    const selected = this.elem_.querySelector('.tab-label.selected');
    if (selected !== null) {
      return this.findTabById_(selected.dataset.id);
    }
    if (this.tabs_.length > 0) {
      return this.tabs_[0];
    }
    return null;
  }

  // Widget

  render() {
    this.elem_ = h('div', { class: 'tab-bar' });
    this.renderTabs_();
    this.addEventListeners_();
    return this.elem_;
  }

  appendTab(tab) {
    this.tabs_.push(tab);
    if (this.elem_) {
      this.renderTabs_();
    }
  }

  // Private

  renderTabs_() {
    const selected = this.selectedTab;
    super.clear();
    for (const tab of this.tabs_) {
      const label = h('span', {
        'class': 'tab-label',
        'data-id': tab.id,
      }, t(tab.label));
      if (tab === selected) {
        label.classList.add('selected');
      }
      this.elem_.appendChild(label);
    }
  }

  addEventListeners_() {
    this.elem_.addEventListener('click', (event) => {
      this.elem_.querySelectorAll('.tab-label').forEach((label) => {
        label.classList.remove('selected');
        this.emit('deselect', this.findTabById_(label.dataset.id));
      });
      event.target.classList.add('selected');
      this.emit('select', this.findTabById_(event.target.dataset.id));
    });
  }

  findTabById_(id) {
    return this.tabs_.find((tab) => tab.id === id);
  }
}
