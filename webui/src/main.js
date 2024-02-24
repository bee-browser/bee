'use strict';

import DebugConsole from './debug_console.js';

const widget = new DebugConsole();
document.body.appendChild(widget.render());
widget.on('ready', () => widget.load('text:HELLO, WORLD.'));
widget.start();
