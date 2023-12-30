'use strict';
import MainView from './main_view.js';
const widget = new MainView;
document.body.appendChild(widget.render());
widget.start();
