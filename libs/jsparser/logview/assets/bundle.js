'use strict';
function h(tagName, attrs = {}, ...children) {
    const elem = document.createElement(tagName);
    for (const attr of Object.entries(attrs)){
        elem.setAttribute(attr[0], attr[1]);
    }
    for (const child of children){
        elem.appendChild(child);
    }
    return elem;
}
function t(text) {
    return document.createTextNode(text);
}
class EventEmitter {
    events = new Map();
    maxListeners;
    #defaultMaxListeners = 10;
    get defaultMaxListeners() {
        return this.#defaultMaxListeners;
    }
    set defaultMaxListeners(n) {
        if (Number.isInteger(n) || n < 0) {
            const error = new RangeError('The value of "defaultMaxListeners" is out of range. It must be a non-negative integer. Received ' + n + '.');
            throw error;
        }
        this.#defaultMaxListeners = n;
    }
    addListener(eventName, listener) {
        return this.on(eventName, listener);
    }
    emit(eventName, ...args) {
        const listeners = this.events.get(eventName);
        if (listeners === undefined) {
            if (eventName === 'error') {
                const error = args[0];
                if (error instanceof Error) throw error;
                throw new Error('Unhandled error.');
            }
            return false;
        }
        const copyListeners = [
            ...listeners
        ];
        for (const listener of copyListeners){
            listener.apply(this, args);
        }
        return true;
    }
    setMaxListeners(n) {
        if (!Number.isInteger(n) || n < 0) {
            throw new RangeError('The value of "n" is out of range. It must be a non-negative integer. Received ' + n + '.');
        }
        this.maxListeners = n;
        return this;
    }
    getMaxListeners() {
        if (this.maxListeners === undefined) {
            return this.defaultMaxListeners;
        }
        return this.maxListeners;
    }
    listenerCount(eventName) {
        const events = this.events.get(eventName);
        return events === undefined ? 0 : events.length;
    }
    eventNames() {
        return Reflect.ownKeys(this.events);
    }
    listeners(eventName) {
        const listeners = this.events.get(eventName);
        return listeners === undefined ? [] : listeners;
    }
    off(eventName, listener) {
        return this.removeListener(eventName, listener);
    }
    on(eventName, listener, prepend) {
        if (this.events.has(eventName) === false) {
            this.events.set(eventName, []);
        }
        const events = this.events.get(eventName);
        if (prepend) {
            events.unshift(listener);
        } else {
            events.push(listener);
        }
        if (eventName !== "newListener" && this.events.has("newListener")) {
            this.emit('newListener', eventName, listener);
        }
        const maxListener = this.getMaxListeners();
        const eventLength = events.length;
        if (maxListener > 0 && eventLength > maxListener && !events.warned) {
            events.warned = true;
            const warning = new Error(`Possible EventEmitter memory leak detected.
         ${this.listenerCount(eventName)} ${eventName.toString()} listeners.
         Use emitter.setMaxListeners() to increase limit`);
            warning.name = "MaxListenersExceededWarning";
            console.warn(warning);
        }
        return this;
    }
    removeAllListeners(eventName) {
        const events = this.events;
        if (!events.has('removeListener')) {
            if (arguments.length === 0) {
                this.events = new Map();
            } else if (events.has(eventName)) {
                events.delete(eventName);
            }
            return this;
        }
        if (arguments.length === 0) {
            for (const key of events.keys()){
                if (key === 'removeListener') continue;
                this.removeAllListeners(key);
            }
            this.removeAllListeners('removeListener');
            this.events = new Map();
            return this;
        }
        const listeners = events.get(eventName);
        if (listeners !== undefined) {
            listeners.map((listener)=>{
                this.removeListener(eventName, listener);
            });
        }
        return this;
    }
    removeListener(eventName, listener) {
        const events = this.events;
        if (events.size === 0) return this;
        const list = events.get(eventName);
        if (list === undefined) return this;
        const index = list.findIndex((item)=>item === listener || item.listener === listener);
        if (index === -1) return this;
        list.splice(index, 1);
        if (list.length === 0) this.events.delete(eventName);
        if (events.has('removeListener')) {
            this.emit('removeListener', eventName, listener);
        }
        return this;
    }
    once(eventName, listener) {
        this.on(eventName, this.onceWrap(eventName, listener));
        return this;
    }
    onceWrap(eventName, listener) {
        const wrapper = function(...args) {
            this.context.removeListener(this.eventName, this.wrapedListener);
            this.listener.apply(this.context, args);
        };
        const wrapperContext = {
            eventName: eventName,
            listener: listener,
            wrapedListener: wrapper,
            context: this
        };
        const wrapped = wrapper.bind(wrapperContext);
        wrapperContext.wrapedListener = wrapped;
        wrapped.listener = listener;
        return wrapped;
    }
    prependListener(eventName, listener) {
        return this.on(eventName, listener, true);
    }
    prependOnceListener(eventName, listener) {
        this.prependListener(eventName, this.onceWrap(eventName, listener));
        return this;
    }
    rawListeners(eventName) {
        const events = this.events;
        if (events === undefined) return [];
        const listeners = events.get(eventName);
        if (listeners === undefined) return [];
        return [
            ...listeners
        ];
    }
}
'use strict';
class Widget extends EventEmitter {
    constructor(){
        super();
        this.elem_ = null;
    }
    render() {
        throw new Error('must be override');
    }
    clear() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.innerHTML = '';
    }
    show() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.remove('hide');
    }
    hide() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.add('hide');
    }
    select() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.add('selected');
    }
    deselect() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.classList.remove('selected');
    }
    scrollIntoView() {
        if (!this.hasRendered()) {
            return;
        }
        this.elem_.scrollIntoViewIfNeeded(true);
    }
    hasRendered() {
        return this.elem_ !== null;
    }
}
'use strict';
class Toolbar extends Widget {
    constructor(){
        super();
        this.running_ = false;
        this.actionButton_ = h('button', {
            id: 'action'
        }, t('Start'));
        this.actionButton_.userdata_ = {
            started: false
        };
        this.actionButton_.addEventListener('click', ()=>{
            if (this.running_) {
                this.emit('pause');
                this.actionButton_.replaceChildren('Start');
            } else {
                this.emit('start');
                this.actionButton_.replaceChildren('Pause');
            }
            this.running_ = !this.running_;
            this.nextButton_.disabled = this.running_;
        });
        this.nextButton_ = h('button', {
            id: 'next'
        }, t('Next'));
        this.nextButton_.addEventListener('click', ()=>this.emit('next'));
        this.resetButton_ = h('button', {
            id: 'reset'
        }, t('Reset'));
        this.resetButton_.addEventListener('click', ()=>this.emit('reset'));
    }
    render() {
        this.elem_ = h('div', {
            id: 'toolbar'
        }, this.actionButton_, this.nextButton_, this.resetButton_);
        return this.elem_;
    }
}
class ParserView extends Widget {
    constructor(){
        super();
        this.stackView_ = new StackView();
    }
    render() {
        this.elem_ = h('div', {
            id: 'parser-view'
        }, this.stackView_.render());
        return this.elem_;
    }
    clear() {
        this.stackView_.clear();
        this.elem_.replaceChildren(this.stackView_.render());
    }
    feed(data) {
        switch(data.opcode){
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
    constructor(){
        super();
        this.views_ = [];
    }
    render() {
        this.elem_ = h('div', {
            id: 'parser-stack'
        });
        for (const view of this.views_){
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
        while(n > 0){
            const view = this.views_.pop();
            this.elem_.removeChild(view.elem_);
            n--;
        }
    }
}
class StateView extends Widget {
    constructor(id, items){
        super();
        this.id_ = id;
        this.items_ = items;
    }
    render() {
        const items = h('div', {
            class: 'parser-state-items'
        });
        for (const item of this.items_){
            items.appendChild(h('div', {
                class: 'parser-state-item'
            }, t(item)));
        }
        this.elem_ = h('div', {
            class: 'parser-state'
        }, h('div', {
            class: 'parser-state-id'
        }, t(this.id_)), items);
        return this.elem_;
    }
}
class LexerView extends Widget {
    constructor(){
        super();
        this.cursorPos_ = 0;
    }
    render() {
        this.elem_ = h('div', {
            id: 'lexer-view'
        }, h('div', {
            id: 'lexer-cursor'
        }, t('0, 0')), h('div', {
            id: 'lexer-state'
        }), h('div', {
            id: 'lexical-goal'
        }), h('div', {
            id: 'candidate-token'
        }), h('div', {
            id: 'candidate-lexeme'
        }));
        return this.elem_;
    }
    clear() {
        this.elem_.replaceChildren(h('div', {
            id: 'lexer-cursor'
        }, t('0, 0')), h('div', {
            id: 'lexer-state'
        }), h('div', {
            id: 'lexical-goal'
        }), h('div', {
            id: 'candidate-token'
        }), h('div', {
            id: 'candidate-lexeme'
        }));
    }
    feed(data) {
        switch(data.opcode){
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
                    lexeme: data['token.lexeme']
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
'use strict';
class MainView extends Widget {
    constructor(){
        super();
        this.pc_ = 0;
        this.logs_ = [];
        this.toolbar_ = new Toolbar();
        this.toolbar_.on('start', ()=>this.startReplay_());
        this.toolbar_.on('pause', ()=>this.pauseReplay_());
        this.toolbar_.on('reset', ()=>this.resetReplay_());
        this.toolbar_.on('next', ()=>this.dispatch_());
        this.parserView_ = new ParserView();
        this.lexerView_ = new LexerView();
        this.on('log', this.handleLog_.bind(this));
    }
    render() {
        this.elem_ = h('div', {
            id: 'main-view'
        }, this.toolbar_.render(), h('div', {
            id: 'views'
        }, this.parserView_.render(), this.lexerView_.render()));
        return this.elem_;
    }
    start() {
        const es = new EventSource('/logs');
        es.addEventListener('spawned', (event)=>{
            console.debug('spawned');
        });
        es.addEventListener('log', (event)=>{
            const log = JSON.parse(event.data);
            this.emit('log', log);
        });
        es.addEventListener('terminated', (event)=>{
            console.debug('terminated');
            event.target.close();
        });
        es.addEventListener('error', (event)=>{
            console.error('error');
            event.target.close();
        });
    }
    startReplay_() {
        this.timer_ = setInterval(()=>this.dispatch_(), 100);
    }
    pauseReplay_() {
        clearInterval(this.timer_);
    }
    resetReplay_() {
        this.pc_ = 0;
        this.parserView_.clear();
        this.lexerView_.clear();
    }
    dispatch_() {
        const log = this.logs_[this.pc_];
        if (log === undefined) {
            this.pauseReplay_();
            return;
        }
        this.pc_++;
        switch(log.type){
            case 'parser':
                this.parserView_.feed(log.data);
                break;
            case 'lexer':
                this.lexerView_.feed(log.data);
                break;
        }
    }
    handleLog_(log) {
        if (log.target.startsWith('jsparser::parser')) {
            switch(log.level){
                case 'TRACE':
                    this.logs_.push({
                        type: 'parser',
                        level: 'trace',
                        data: log.fields
                    });
                    break;
            }
        }
        if (log.target.startsWith('jsparser::lexer')) {
            switch(log.level){
                case 'TRACE':
                    this.logs_.push({
                        type: 'lexer',
                        level: 'trace',
                        data: log.fields
                    });
                    break;
            }
        }
    }
}
'use strict';
const widget = new MainView();
document.body.appendChild(widget.render());
widget.start();
