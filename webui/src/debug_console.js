'use strict';

import { h } from './helper';
import Widget from './widget';
import NavigationBar from './navigation_bar';
import PaintView from './paint_view';
import TabBar from './tab_bar';
import LogListView from './log_list_view';
import LayoutView from './layout_view';

export default class DebugConsole extends Widget {
  constructor() {
    super();

    const navBar = new NavigationBar();
    navBar.on('debcon.navigation.go', this.handleNavigationGo_.bind(this));
    navBar.on('debcon.remoteSurface', this.handleRemoteSurface_.bind(this));
    this.on('message', navBar.handleMessage.bind(navBar));
    this.navBar_ = navBar;

    const paintView = new PaintView();
    this.on('message', paintView.handleMessage.bind(paintView));
    this.paintView_ = paintView;

    this.logListView_ = new LogListView();
    this.on('message', this.logListView_.handleMessage.bind(this.logListView_));

    const layoutView = new LayoutView();
    this.on('message', layoutView.handleMessage.bind(layoutView));
    this.layoutView_ = layoutView;

    this.tabBar_ = new TabBar();
    this.tabBar_.on('select', (tab) => tab.content.show());
    this.tabBar_.on('deselect', (tab) => tab.content.hide());
    this.tabBar_.appendTab({
      id: 'logs', label: 'Logs', content: this.logListView_
    });
    this.tabBar_.appendTab({
      id: 'layout', label: 'Layout', content: this.layoutView_
    });

    paintView.on('select', this.selectObject.bind(this));
    paintView.on('target', this.targetObject.bind(this));
    paintView.on('untarget', this.untargetObject.bind(this));

    layoutView.on('select', this.selectObject.bind(this));
    layoutView.on('target', this.targetObject.bind(this));
    layoutView.on('untarget', this.untargetObject.bind(this));
  }

  render() {
    this.elem_ =
      h('div', { 'id': 'debug-console', 'class': 'grid'},
        this.navBar_.render(),
        this.paintView_.render(),
        h('div', { 'id': 'multi-view-container' },
          this.tabBar_.render(),
          h('div', { 'id': 'multi-view-content' },
            this.logListView_.render(),
            this.layoutView_.render())));
    this.logListView_.hide();
    this.layoutView_.hide();
    this.tabBar_.selectedTab.content.show();
    return this.elem_;
  }

  clear() {
    this.paintView_.clear();
    this.logListView_.clear();
    this.layoutView_.clear();
  }

  start() {
    this.ws_ = new WebSocket(`ws://${location.host}/`);
    this.ws_.addEventListener('open', (event) => {
      this.emit('ready', this);
    });
    this.ws_.addEventListener('message', (event) => {
      try {
        const msg = JSON.parse(event.data);
        this.emit('message', msg);
      } catch (e) {
        console.error(
          `failed to parse a JSON message: ${event.data}: ${e.message}`);
      }
    });
  }

  load(uri) {
    this.navBar_.setUri(uri);
  }

  selectObject(layoutId) {
    this.paintView_.selectObject(layoutId);
    this.layoutView_.selectObject(layoutId);
  }

  targetObject(layoutId) {
    this.paintView_.targetObject(layoutId);
    this.layoutView_.targetObject(layoutId);
  }

  untargetObject(layoutId) {
    this.paintView_.untargetObject(layoutId);
    this.layoutView_.untargetObject(layoutId);
  }

  handleNavigationGo_(uri) {
    this.clear();
    let data = { uri };
    if (this.remoteSurface_) {
      data.remotes = {
        pusher: ['-c', 'demo']
      };
    }
    this.ws_.send(JSON.stringify({ type: 'navigation.go', data }));
  }

  handleRemoteSurface_(enable) {
    this.remoteSurface_ = enable;
  }
}
