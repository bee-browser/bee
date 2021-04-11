'use strict';

import { h } from './helper.js';
import Widget from './widget.js';
import NavigationBar from './navigation_bar.js';
import BoxView from './box_view.js';
import TabBar from './tab_bar.js';
import LogListView from './log_list_view.js';
import LayoutView from './layout_view.js';

export default class DebugConsole extends Widget {
  constructor() {
    super();

    const navBar = new NavigationBar();
    navBar.on('debcon.navigation.go', this.handleNavigationGo_.bind(this));
    navBar.on('debcon.remoteSurface', this.handleRemoteSurface_.bind(this));
    this.on('message', navBar.handleMessage.bind(navBar));
    this.navBar_ = navBar;

    const boxView = new BoxView();
    this.on('message', boxView.handleMessage.bind(boxView));
    this.boxView_ = boxView;

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

    boxView.on('select', this.selectObject.bind(this));
    boxView.on('target', this.targetObject.bind(this));
    boxView.on('untarget', this.untargetObject.bind(this));

    layoutView.on('select', this.selectObject.bind(this));
    layoutView.on('target', this.targetObject.bind(this));
    layoutView.on('untarget', this.untargetObject.bind(this));
  }

  render() {
    this.elem_ =
      h('div', { 'id': 'debug-console', 'class': 'grid'},
        this.navBar_.render(),
        this.boxView_.render(),
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
    this.boxView_.clear();
    this.logListView_.clear();
    this.layoutView_.clear();
  }

  start() {
    this.ws_ = new WebSocket(`ws://${location.host}/api/debcon`);
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
    this.boxView_.selectObject(layoutId);
    this.layoutView_.selectObject(layoutId);
  }

  targetObject(layoutId) {
    this.boxView_.targetObject(layoutId);
    this.layoutView_.targetObject(layoutId);
  }

  untargetObject(layoutId) {
    this.boxView_.untargetObject(layoutId);
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
