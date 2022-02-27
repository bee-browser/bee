'use strict';

import { h, formatBeeRect } from './helper.js';
import Widget from './widget.js';
import BoxOverlay from './box_overlay.js';
import Surface from './surface.js';
import BoxPropView from './box_prop_view.js';

const BOX_OUTLINE_PROP_MAP = {
  top: 'y', left: 'x', width: 'width', height: 'height'
};

export default class BoxView extends Widget {
  constructor() {
    super();
    this.props_ = {};

    this.overlay_ = new BoxOverlay();

    this.surface_ = new Surface();
    this.surface_.on('click', (event) => {
      event.preventDefault();
      event.stopPropagation();
      this.emit('select', event.userdata.layoutId);
    });
    this.surface_.on('mouseover', (event) => {
      event.preventDefault();
      event.stopPropagation();
      this.emit('target', event.userdata.layoutId);
    });
    this.surface_.on('mouseout', (event) => {
      event.preventDefault();
      event.stopPropagation();
      this.emit('untarget', event.userdata.layoutId);
    });

    this.propView_ = new BoxPropView();
  }

  render() {
    this.elem_ =
      h('div', { id: 'box-view' },
        h('div', { 'class': 'grid' },
          h('div', { 'id': 'surface-container' },
            this.overlay_.render(),
            this.surface_.render()),
          this.propView_.render()));

    return this.elem_;
  }

  clear() {
    this.props_ = {};
    this.overlay_.clear();
    this.surface_.clear();
  }

  selectObject(layoutId) {
    this.surface_.selectObject(layoutId);
    const prop = this.props_[layoutId];
    if (prop === undefined) {
      return;
    }
    this.propView_.setProp(prop);
  }

  targetObject(layoutId) {
    const prop = this.props_[layoutId];
    if (prop === undefined) {
      return;
    }
    this.surface_.targetObject(layoutId);
    this.overlay_.show(prop.rects);
  }

  untargetObject(layoutId) {
    this.overlay_.hide();
  }

  handleMessage(msg) {
    switch (msg.type) {
    case 'asset.add':
      this.surface_.addAsset(msg.data);
      break;
    case 'render.start':
      this.surface_.start(msg.data);
      break;
    case 'render.render_box':
      this.surface_.renderBox(msg.data);
      break;
    case 'render.render_asset':
      this.surface_.renderAsset(msg.data);
      break;
    case 'render.end':
      this.surface_.end();
      break;
    case 'layout.monitor.render_box':
      if (!(msg.data.object_id in this.props_)) {
        this.props_[msg.data.object_id] = {};
      }
      this.props_[msg.data.object_id].rects = {
        marginRect: msg.data.margin_rect,
        borderRect: msg.data.border_rect,
        paddingRect: msg.data.padding_rect,
        contentRect: msg.data.content_rect,
      };
      break;
    }
  }
}
