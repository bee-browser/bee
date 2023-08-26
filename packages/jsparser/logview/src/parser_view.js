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

  feed(data) {
    switch (data.opcode) {
    case 'push':
      this.stackView_.pushState(data['state.label']);
      break;
    case 'pop':
      this.stackView_.popStates(data.num_states);
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

  pushState(state) {
    const items = state.split(', ');
    const view = new StateView(items);
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
  constructor(items) {
    super();
    this.items_ = items;
  }

  render() {
    this.elem_ = h('div', { class: 'parser-state' });
    for (const item of this.items_) {
      this.elem_.appendChild(h('div', { class: 'parser-state-item' }, t(item)));
    }
    return this.elem_;
  }
}
