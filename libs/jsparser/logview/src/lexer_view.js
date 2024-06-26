import { h, t } from '../../../../webui/src/helper.js';
import Widget from '../../../../webui/src/widget.js';

export default class LexerView extends Widget {
  constructor() {
    super();
    this.cursorPos_ = 0;
  }

  render() {
    this.elem_ = h(
      'div',
      { id: 'lexer-view' },
      h('div', { id: 'lexer-cursor' }, t('0, 0')),
      h('div', { id: 'lexer-state' }),
      h('div', { id: 'lexical-goal' }),
      h('div', { id: 'candidate-token' }),
      h('div', { id: 'candidate-lexeme' }),
    );
    return this.elem_;
  }

  clear() {
    this.elem_.replaceChildren(
      h('div', { id: 'lexer-cursor' }, t('0, 0')),
      h('div', { id: 'lexer-state' }),
      h('div', { id: 'lexical-goal' }),
      h('div', { id: 'candidate-token' }),
      h('div', { id: 'candidate-lexeme' }),
    );
  }

  feed(data) {
    switch (data.opcode) {
      case 'set_goal':
        this.setGoal_(data.goal);
        break;
      case 'init':
        this.setState_(data.state);
        break;
      case 'next':
        this.setState_(data.state);
        break;
      case 'accept':
        this.setToken_({
          kind: data['token.kind'],
          lexeme: data['token.lexeme'],
        });
        break;
      case 'consume':
        this.cursorTokenEnd_ = data['cursor.token_end'];
        this.updateCursor_();
        break;
      case 'advance':
        this.cursorPos_ = data['cursor.pos'];
        this.updateCursor_();
        break;
    }
  }

  updateCursor_() {
    document.getElementById('lexer-cursor').innerHTML = `${this.cursorPos_}`;
  }

  setState_(state) {
    if (state === 'State(0)') {
      this.setToken_(null);
    }
    document.getElementById('lexer-state').innerHTML = '';
    document.getElementById('lexer-state').appendChild(t(state));
  }

  setGoal_(goal) {
    document.getElementById('lexical-goal').innerHTML = '';
    document.getElementById('lexical-goal').appendChild(t(goal));
  }

  setToken_(token) {
    document.getElementById('candidate-token').innerHTML = '';
    document.getElementById('candidate-lexeme').innerHTML = '';
    if (token) {
      document.getElementById('candidate-token').appendChild(t(token.kind));
      document.getElementById('candidate-lexeme').appendChild(t(token.lexeme));
    }
  }
}
