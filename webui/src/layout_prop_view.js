'use strict';

import { f, h, t, formatBeeNumber, formatBeePoint } from './helper.js';
import Widget from './widget.js';
import TabBar from './tab_bar.js';

export default class LayoutPropView extends Widget {
  constructor() {
    super();

    this.basicView_ = new LayoutBasicPropView();
    this.stylesView_ = new LayoutStylesView();
    this.boxModelView_ = new LayoutBoxModelView();
    this.layerView_ = new LayoutLayerView();

    this.tabBar_ = new TabBar();
    this.tabBar_.appendTab({
      id: 'basic', label: 'Basic', content: this.basicView_
    });
    this.tabBar_.appendTab({
      id: 'styles', label: 'Styles', content: this.stylesView_
    });
    this.tabBar_.appendTab({
      id: 'box-model', label: 'Box Model', content: this.boxModelView_
    });
    this.tabBar_.appendTab({
      id: 'layer', label: 'Layer', content: this.layerView_
    });
    this.tabBar_.on('select', (tab) => tab.content.show());
    this.tabBar_.on('deselect', (tab) => tab.content.hide());
  }

  setProp(object) {
    this.basicView_.setProp(object);
    this.stylesView_.setProp(object.style);
    this.boxModelView_.setProp(object.dimension);
    this.layerView_.setProp(object.layer);
  }

  // Widget

  render() {
    this.elem_ =
      h('div', { id: 'layout-prop-view' },
        this.tabBar_.render(),
        this.basicView_.render(),
        this.stylesView_.render(),
        this.boxModelView_.render(),
        this.layerView_.render());

    this.tabBar_.selectedTab.content.show();

    return this.elem_;
  }

  clear() {
  }
}

class LayoutBasicPropView extends Widget {
  constructor() {
    super();
  }

  setProp(object) {
    super.clear();
    this.elem_.appendChild(this.renderProp_(object));
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'layout-prop-basic hide' });
    return this.elem_;
  }

  // Private

  renderProp_(prop) {
    return h('section', { class: 'layout-prop-section' },
             h('table', { class: 'layout-prop-table' },
               renderProp_('ID', prop.id.toString(16).toUpperCase()),
               renderProp_('Label', prop.label),
               renderProp_('Object Schema', prop.schema.object),
               renderProp_('Content Schema', prop.schema.content),
               renderProp_('Layer Schema', prop.schema.layer),
               renderProp_('Margin Wall', prop.schema.margin_wall)));
  }
}

class LayoutStylesView extends Widget {
  constructor() {
    super();
  }

  setProp(style) {
    super.clear();
    this.elem_.appendChild(this.renderProp_(style));
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'layout-prop-styles hide' });
    return this.elem_;
  }

  // Private

  renderProp_(style) {
    return h('section', { class: 'layout-prop-section' },
             this.renderStyleTable_(style));
  }

  renderStyleTable_(style) {
    return Object.entries(style).reduce((table, [name, value]) => {
      table.appendChild(renderProp_(name, value));
      return table;
    }, h('table', { class: 'layout-prop-table' }));
  }
}

class LayoutBoxModelView extends Widget {
  constructor() {
    super();
  }

  setProp(dim) {
    super.clear();
    if (dim) {
      this.elem_.appendChild(this.renderProp_(dim));
    }
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'layout-prop-box-model hide' });
    return this.elem_;
  }

  // Private

  renderProp_(dim) {
    return h('section', { class: 'layout-prop-section' },
             this.renderDimensionTable_(dim));
  }

  renderDimensionTable_(dim) {
    const SIDES = ['top', 'right', 'bottom', 'left'];
    const table = h('table', { class: 'layout-prop-table' });
    table.appendChild(
      renderProp_('width', formatBeeNumber(dim.width)));
    table.appendChild(
      renderProp_('min-width', formatBeeNumber(dim.minWidth)));
    table.appendChild(
      renderProp_('max-width', formatBeeNumber(dim.maxWidth)));
    table.appendChild(
      renderProp_('height', formatBeeNumber(dim.height)));
    table.appendChild(
      renderProp_('min-height', formatBeeNumber(dim.minHeight)));
    table.appendChild(
      renderProp_('max-height', formatBeeNumber(dim.maxHeight)));
    SIDES.forEach((side) => {
      table.appendChild(
        renderProp_(`padding-${side}`, formatBeeNumber(dim.padding[side])));
    });
    SIDES.forEach((side) => {
      table.appendChild(
        renderProp_(`border-${side}`, formatBeeNumber(dim.border[side])));
    });
    SIDES.forEach((side) => {
      table.appendChild(
        renderProp_(`margin-${side}`, formatBeeNumber(dim.margin[side])));
    });
    return table;
  }
}

class LayoutLayerView extends Widget {
  constructor() {
    super();
  }

  setProp(layer) {
    super.clear();
    if (layer) {
      this.elem_.appendChild(this.renderProp_(layer));
    }
  }

  // Widget

  render() {
    this.elem_ = h('div', { 'class': 'layout-prop-box-model hide' });
    return this.elem_;
  }

  // Private

  renderProp_(layer) {
    return f(this.renderLayerBasicSection_(layer),
             h('hr'),
             this.renderLayerPositionSection_(layer));
  }

  renderLayerBasicSection_(layer) {
    return h('section', { class: 'layout-prop-section' },
             h('table', { class: 'layout-prop-table' },
               renderProp_(
                 'ID', layer.id.toString(16).toUpperCase()),
               renderProp_(
                 'Owner ID', layer.ownerId.toString(16).toUpperCase()),
               renderProp_(
                 'Container ID', layer.containerId.toString(16).toUpperCase()),
               renderProp_(
                 'New Stacking Context', layer.newStackingContext),
               renderProp_(
                 'Stack Level', layer.stackLevel)));
  }

  renderLayerPositionSection_(layer) {
    return h('section', { class: 'layout-prop-section' },
             h('table', { class: 'layout-prop-table' },
               renderProp_(
                 'Position', formatBeePoint(layer.position)),
               renderProp_(
                 'Static Position', formatBeePoint(layer.staticPosition)),
               renderProp_(
                 'Extra Offset', formatBeePoint(layer.extraOffset))));
  }
}

function renderProp_(name, value) {
  return h('tr', { class: 'layout-prop' },
           h('td', { class: 'layout-prop-name' }, t(name)),
           h('td', { class: 'layout-prop-value' }, t(value)));
}
