'use strict';

import { h, t } from './helper';
import Widget from './widget';

export default class NavigationBar extends Widget {
  constructor() {
    super();
  }

  render() {
    this.elem_ =
      h('div', { id: 'navigation-bar' },
        h('div', { id: 'loading-icon' }, t(' ')),
        h('form', { id: 'address-bar' }, h('input')),
        h('button', { id: 'rsf-button' }, t('Remote Surface')));

    this.loadingIcon_ = this.elem_.querySelector('#loading-icon');

    this.addressBar_ = this.elem_.querySelector('#address-bar');
    this.addressBar_.addEventListener(
      'submit', this.handleAddressBarSubmit_.bind(this));

    this.addressInput_ = this.addressBar_.querySelector('input');
    this.addressInput_.addEventListener(
      'focus', () => this.addressInput_.select());

    this.rsfButton_ = this.elem_.querySelector('#rsf-button');
    this.rsfButton_.addEventListener(
      'click', this.toggleRemoteSurface_.bind(this));

    return this.elem_;
  }

  setUri(uri) {
    this.addressBar_.querySelector('input').value = uri;
    this.navigate(uri);
  }

  handleAddressBarSubmit_(event) {
    event.preventDefault();
    event.stopPropagation();
    const input = this.elem_.querySelector('input');
    const uri = input.value;
    if (uri.length > 0) {
      this.navigate(uri);
    }
  }

  navigate(uri) {
    this.emit('debcon.navigation.go', uri);
  }

  toggleRemoteSurface_(event) {
    const enable = this.rsfButton_.classList.toggle('enable');
    this.emit('debcon.remoteSurface', enable);
  }

  handleMessage(msg) {
    switch (msg.type) {
    case 'navigation.start':
      this.loadingIcon_.innerText = '*';
      break;
    case 'navigation.end':
      this.loadingIcon_.innerText = ' ';
      break;
    }
  }
}
