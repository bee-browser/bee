'use strict';

import { h, t, formatBeeRect } from './helper';
import Widget from './widget';
import TabBar from './tab_bar';

export default class PaintPropView extends Widget {
  constructor() {
    super();

    this.rectsView_ = new PaintRectsPropView();

    this.tabBar_ = new TabBar();
    this.tabBar_.appendTab({
      id: 'rects', label: 'Rects', content: this.rectsView_
    });
  }

  setProp(prop) {
    this.rectsView_.setProp(prop.rects);
  }

  // Widget

  render() {
    this.elem_ =
      h('div', { 'id': 'paint-prop-view' },
        this.tabBar_.render(),
        this.rectsView_.render());
    this.tabBar_.selectedTab.content.show();
    return this.elem_;
  }

  // Private
}

class PaintRectsPropView extends Widget {
  constructor() {
    super();
  }

  setProp(rects) {
    super.clear();
    if (rects) {
      this.renderProp_(rects);
    }
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'paint-prop-rects hide' });
    return this.elem_;
  }

  // Private

  renderProp_(rects) {
    const table = h('table', { 'class': 'paint-prop-table' });
    for (const type of ['margin', 'border', 'padding', 'content']) {
      if (`${type}Rect` in rects) {
        table.appendChild(
          renderProp_(`${type}-rect`, formatBeeRect(rects[`${type}Rect`])));
      }
    }
    this.elem_.appendChild(table);
  }
}

function renderProp_(name, value) {
  return h('tr', { class: 'print-prop' },
           h('td', { class: 'print-prop-name' }, t(name)),
           h('td', { class: 'print-prop-value' }, t(value)));
}
