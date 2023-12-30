import { h, t } from '../../../../webui/src/helper.js';
import Widget from '../../../../webui/src/widget.js';

export default class ParserView extends Widget {
  constructor() {
    super();
    this.stackView_ = new StackView();
  }

  render() {
    this.elem_ =
      h('div', { id: 'parser-view' },
        this.stackView_.render());
    return this.elem_;
  }

  clear() {
    this.stackView_.clear();
    this.elem_.replaceChildren(this.stackView_.render());
  }

  feed(data) {
    switch (data.opcode) {
    case 'push-state':
      this.stackView_.pushState(data['state.id'], data['state.label']);
      break;
    case 'pop-state':
      this.stackView_.popStates(data.num_states);
      break;
    case 'push-block-context':
      break;
    case 'pop-block-context':
      break;
    case 'push-block':
      break;
    case 'pop-block':
      break;
    case 'accept':
      break;
    case 'shift':
      break;
    case 'reduce':
      break;
    }
  }
}

class StackView extends Widget {
  constructor() {
    super();
    this.views_ = [];
  }

  render() {
    this.elem_ = h('div', { id: 'parser-stack' });
    for (const view of this.views_) {
      this.elem_.appendChild(view.render());
    }
    return this.elem_;
  }

  clear() {
    this.views_ = [];
    super.clear();
  }

  pushState(id, label) {
    const items = label.split(', ');
    const view = new StateView(id, items);
    this.elem_.appendChild(view.render());
    this.views_.push(view);
  }

  popStates(n) {
    while (n > 0) {
      const view = this.views_.pop();
      this.elem_.removeChild(view.elem_);
      n--;
    }
  }
}

class StateView extends Widget {
  constructor(id, items) {
    super();
    this.id_ = id;
    this.items_ = items;
  }

  render() {
    const items = h('div', { class: 'parser-state-items' });
    for (const item of this.items_) {
      items.appendChild(h('div', { class: 'parser-state-item' }, t(item)));
    }
    this.elem_ = h('div', { class: 'parser-state' },
                   h('div', { class: 'parser-state-id' }, t(this.id_)),
                   items);
    return this.elem_;
  }
}
