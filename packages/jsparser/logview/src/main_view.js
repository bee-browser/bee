import { h } from '../../../../webui/src/helper.js';
import Widget from '../../../../webui/src/widget.js';
import ParserView from './parser_view.js';
import LexerView from './lexer_view.js';

export default class MainView extends Widget {
  constructor() {
    super();

    this.pc_ = 0;
    this.logs_ = [];

    this.parserView_ = new ParserView();
    this.lexerView_ = new LexerView();

    this.on('log', this.handleLog_.bind(this));
  }

  render() {
    this.elem_ =
      h('div', { id: 'main-view' },
        this.parserView_.render(),
        this.lexerView_.render());
    return this.elem_;
  }

  start() {
    const es = new EventSource('/logs');
    es.addEventListener('spawned', (event) => {
      console.debug('spawned');
    });
    es.addEventListener('log', (event) => {
      const log = JSON.parse(event.data);
      //console.debug('log', log);
      this.emit('log', log);
    });
    es.addEventListener('terminated', (event) => {
      console.debug('terminated');
      event.target.close();
    });
    es.addEventListener('error', (event) => {
      console.error('error');
      event.target.close();
    });
  }

  dispatch_() {
    const log = this.logs_[this.pc_];
    if (log === undefined) {
      return;
    }
    this.pc_++;
    switch (log.type) {
    case 'parser':
      this.parserView_.feed(log.data);
      break;
    case 'lexer':
      this.lexerView_.feed(log.data);
      break;
    }
  }

  handleLog_(log) {
    if (log.target.startsWith('bee_jsparser::parser')) {
      switch (log.level) {
      case 'TRACE':
        this.logs_.push({
          type: 'parser',
          level: 'trace',
          data: log.fields,
        });
        break;
      }
    }
    if (log.target.startsWith('bee_jsparser::lexer')) {
      switch (log.level) {
      case 'TRACE':
        this.logs_.push({
          type: 'lexer',
          level: 'trace',
          data: log.fields,
        });
        break;
      }
    }
    this.dispatch_();
  }
}
