'use strict';

import { h } from './helper.js';
import Widget from './widget.js';
import LayoutObject from './layout_object.js';
import LayoutLayer from './layout_layer.js';
import TreeView from './tree_view.js';
import LayoutPropView from './layout_prop_view.js';

export default class LayoutView extends Widget {
  constructor() {
    super();

    this.objects_ = {};
    this.object_tree_ = new TreeView();
    this.object_tree_.on('click', (object, event) => {
      event.preventDefault();
      event.stopPropagation();
      this.emit('select', object.id);
    });
    this.object_tree_.on(
      'mouseover',
      (object) => this.emit('target', object.id),
    );
    this.object_tree_.on(
      'mouseout',
      (object) => this.emit('untarget', object.id),
    );

    this.layers_ = {};
    this.layer_tree_ = new TreeView();
    this.layer_tree_.on('click', (layer, event) => {
      event.preventDefault();
      event.stopPropagation();
      this.emit('select', layer.owner.id);
    });
    this.layer_tree_.on(
      'mouseover',
      (layer) => this.emit('target', layer.owner.id),
    );
    this.layer_tree_.on(
      'mouseout',
      (layer) => this.emit('untarget', layer.owner.id),
    );

    this.prop_view_ = new LayoutPropView();
  }

  render() {
    this.elem_ = h(
      'div',
      { id: 'layout-view' },
      h(
        'div',
        { class: 'grid' },
        h('div', { id: 'layout-object-tree' }, this.object_tree_.render()),
        h('div', { id: 'layout-layer-tree' }, this.layer_tree_.render()),
        this.prop_view_.render(),
      ),
    );
    return this.elem_;
  }

  clear() {
    this.object_tree_.clear();
    this.layer_tree_.clear();
    this.prop_view_.clear();
  }

  selectObject(objectId) {
    this.object_tree_.deselectAll();
    this.layer_tree_.deselectAll();
    const object = this.getObject_(objectId);
    if (object === null) {
      // TODO: show error
      return;
    }

    this.object_tree_.select(object);
    if (object.layer) {
      this.layer_tree_.select(object.layer);
    }
    this.prop_view_.setProp(object);
  }

  targetObject(objectId) {
    const object = this.getObject_(objectId);
    if (object === null) {
      // TODO: show error
      return;
    }
    this.object_tree_.target(object);
    if (object.layer) {
      this.layer_tree_.target(object.layer);
    }
  }

  untargetObject(objectId) {
    const object = this.getObject_(objectId);
    if (object === null) {
      // TODO: show error
      return;
    }
    this.object_tree_.untarget(object);
    if (object.layer) {
      this.layer_tree_.untarget(object.layer);
    }
  }

  handleMessage(msg) {
    if (!msg.type.startsWith('layout.monitor.')) {
      return;
    }

    switch (msg.type) {
      case 'layout.monitor.create-object':
        this.handleCreateObjectEvent_(msg.data);
        break;
      case 'layout.monitor.set-style':
        this.handleSetStyleEvent_(msg.data);
        break;
      case 'layout.monitor.set-label':
        this.handleSetLabelEvent_(msg.data);
        break;
      case 'layout.monitor.insert-object':
        this.handleInsertObjectEvent_(msg.data);
        break;
      case 'layout.monitor.remove-object':
        this.handleRemoveObjectEvent_(msg.data);
        break;
      case 'layout.monitor.create-layer':
        this.handleCreateLayerEvent_(msg.data);
        break;
      case 'layout.monitor.set-layer-style':
        this.handleSetLayerStyleEvent_(msg.data);
        break;
      case 'layout.monitor.insert-layer':
        this.handleInsertLayerEvent_(msg.data);
        break;
      case 'layout.monitor.set-layer-position':
        this.handleSetLayerPositionEvent_(msg.data);
        break;
      case 'layout.monitor.update-box-dimension':
        this.handleUpdateBoxDimensionEvent_(msg.data);
        break;
      case 'layout.monitor.update-box-height':
        this.handleUpdateBoxHeightEvent_(msg.data);
        break;
      case 'layout.monitor.render_box':
        this.handleRenderBoxEvent_(msg.data);
        break;
    }
  }

  handleCreateObjectEvent_(data) {
    const object = new LayoutObject(data);
    this.objects_[data.object_id] = object;
  }

  handleSetStyleEvent_(data) {
    const object = this.getObject_(data.object_id);
    object.style = data.style;
  }

  handleSetLabelEvent_(data) {
    const object = this.getObject_(data.object_id);
    object.label = data.label;
  }

  handleInsertObjectEvent_(data) {
    const object = this.getObject_(data.object_id);
    const parent = this.getObject_(data.parent_id);
    const sibling = this.getObject_(data.next_sibling_id);
    this.object_tree_.insertBefore(object, parent, sibling);
  }

  handleRemoveObjectEvent_(data) {
  }

  handleCreateLayerEvent_(data) {
    const owner = this.getObject_(data.owner_id);
    const layer = new LayoutLayer(data.layer_id, owner);
    owner.layer = layer;
    this.layers_[data.layer_id] = layer;
  }

  handleSetLayerStyleEvent_(data) {
    const layer = this.getLayer_(data.layer_id);
    layer.setStyle(data);
  }

  handleInsertLayerEvent_(data) {
    const layer = this.getLayer_(data.layer_id);
    const container = this.getLayer_(data.container_id);
    layer.container = container;
    const parent = this.getLayer_(data.parent_id);
    const sibling = this.getObject_(data.sibling_id);
    this.layer_tree_.insertBefore(layer, parent, sibling);
  }

  handleSetLayerPositionEvent_(data) {
    const layer = this.getLayer_(data.layer_id);
    layer.setPosition(data);
  }

  handleUpdateBoxDimensionEvent_(data) {
    const object = this.getObject_(data.object_id);
    object.dimension = data;
  }

  handleUpdateBoxHeightEvent_(data) {
    const object = this.getObject_(data.object_id);
    object.height = data.height;
  }

  handleRenderBoxEvent_(data) {
    const object = this.getObject_(data.object_id);
    object.boxes = data;
  }

  getObject_(id) {
    if (id === 0) {
      return null;
    }
    return this.objects_[id];
  }

  getLayer_(id) {
    if (id === 0) {
      return null;
    }
    return this.layers_[id];
  }
}
