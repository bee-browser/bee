import { h, t } from '../../../../webui/src/helper.js';
import Widget from '../../../../webui/src/widget.js';

export default class LogView extends Widget {
  constructor(log) {
    super();
    this.log_ = log;
  }

  render() {
    let target;
    if (this.log_.target.startsWidth('jsparser::parser')) {
      type = 'parser';
    } else if (this.log_.target.startsWidth('jsparser::lexer')) {
      type = 'lexer';
    }

    this.elem_ = h('tr', {});
    this.elem_.appendChild(h('td', { 'class': 'log-type' }, t(type)));
    this.elem_.appendChild(h('td', { 'class': 'log-level' }, t(this.log_.level)));
    this.elem_.appendChild(
      h('td', { 'class': 'log-fields' }, t(JSON.stringify(this.log_.fields))),
    );
    return this.elem_;
  }
}
